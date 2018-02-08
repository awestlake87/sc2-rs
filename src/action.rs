use std::mem;

use futures::prelude::*;
use futures::unsync::{mpsc, oneshot};
use organelle::{Axon, Constraint, Impulse, Soma};
use sc2_proto::{debug, sc2api};

use super::{Error, IntoProto, Result};
use client::ClientTerminal;
use data::{ActionTarget, Command, DebugCommand, DebugTextTarget};
use synapses::{Dendrite, Synapse, Terminal};

pub struct ActionSoma {
    control: Option<ActionControlDendrite>,
    client: Option<ClientTerminal>,
    users: Vec<ActionDendrite>,
}

impl ActionSoma {
    pub fn axon() -> Result<Axon<Self>> {
        Ok(Axon::new(
            Self {
                control: None,
                client: None,
                users: vec![],
            },
            vec![
                Constraint::Variadic(Synapse::Action),
                Constraint::One(Synapse::ActionControl),
            ],
            vec![Constraint::One(Synapse::Client)],
        ))
    }
}

pub fn synapse() -> (ActionTerminal, ActionDendrite) {
    let (tx, rx) = mpsc::channel(10);

    (ActionTerminal { tx: tx }, ActionDendrite { rx: rx })
}

pub fn control_synapse() -> (ActionControlTerminal, ActionControlDendrite) {
    let (tx, rx) = mpsc::channel(1);

    (
        ActionControlTerminal { tx: tx },
        ActionControlDendrite { rx: rx },
    )
}

impl Soma for ActionSoma {
    type Synapse = Synapse;
    type Error = Error;

    #[async(boxed)]
    fn update(mut self, imp: Impulse<Self::Synapse>) -> Result<Self> {
        match imp {
            Impulse::AddTerminal(_, Synapse::Client, Terminal::Client(tx)) => {
                Ok(Self {
                    client: Some(tx),
                    ..self
                })
            },
            Impulse::AddDendrite(_, Synapse::Action, Dendrite::Action(rx)) => {
                self.users.push(rx);

                Ok(self)
            },
            Impulse::AddDendrite(
                _,
                Synapse::ActionControl,
                Dendrite::ActionControl(rx),
            ) => Ok(Self {
                control: Some(rx),
                ..self
            }),

            Impulse::Start(_, main_tx, handle) => {
                let (tx, rx) = mpsc::channel(10);

                // merge all queues
                for user in self.users {
                    handle.spawn(
                        tx.clone()
                            .send_all(user.rx.map_err(|_| unreachable!()))
                            .map(|_| ())
                            .map_err(|_| ()),
                    );
                }

                let task = ActionTask::new(
                    self.client.unwrap(),
                    self.control.unwrap(),
                    rx,
                );

                handle.spawn(task.run().or_else(move |e| {
                    main_tx
                        .send(Impulse::Error(e.into()))
                        .map(|_| ())
                        .map_err(|_| ())
                }));

                Ok(Self {
                    client: None,
                    control: None,
                    users: vec![],
                })
            },

            _ => bail!("unexpected impulse"),
        }
    }
}

struct ActionTask {
    client: ClientTerminal,
    control: Option<ActionControlDendrite>,
    queue: Option<mpsc::Receiver<ActionRequest>>,

    command_batch: Vec<Command>,
    debug_batch: Vec<DebugCommand>,
}

impl ActionTask {
    fn new(
        client: ClientTerminal,
        control: ActionControlDendrite,
        rx: mpsc::Receiver<ActionRequest>,
    ) -> Self {
        Self {
            client: client,
            control: Some(control),
            queue: Some(rx),

            command_batch: vec![],
            debug_batch: vec![],
        }
    }

    #[async]
    fn run(mut self) -> Result<()> {
        let requests = mem::replace(&mut self.queue, None).unwrap();

        let queue = mem::replace(&mut self.control, None)
            .unwrap()
            .rx
            .map(|req| Either::Control(req))
            .select(requests.map(|req| Either::Request(req)));

        #[async]
        for req in queue.map_err(|_| -> Error { unreachable!() }) {
            match req {
                Either::Control(ActionControlRequest::Step(tx)) => {
                    self = await!(self.send_commands())?;
                    self = await!(self.send_debug())?;

                    tx.send(()).map_err(|_| Error::from("unable to ack step"))?;
                },
                Either::Request(ActionRequest::SendCommand(cmd, tx)) => {
                    self.command_batch.push(cmd);
                    tx.send(()).map_err(|_| {
                        Error::from("unable to ack send command")
                    })?;
                },
                Either::Request(ActionRequest::SendDebug(cmd, tx)) => {
                    self.debug_batch.push(cmd);
                    tx.send(())
                        .map_err(|_| Error::from("unable to ack send debug"))?;
                },
            }
        }

        Ok(())
    }

    #[async]
    fn send_commands(self) -> Result<Self> {
        let mut req = sc2api::Request::new();
        req.mut_action().mut_actions();

        for cmd in self.command_batch {
            match cmd {
                Command::Action {
                    units,
                    ability,
                    target,
                } => {
                    let mut a = sc2api::Action::new();

                    {
                        let cmd = a.mut_action_raw().mut_unit_command();

                        cmd.set_ability_id(ability.into_proto()? as i32);

                        match target {
                            Some(ActionTarget::UnitTag(tag)) => {
                                cmd.set_target_unit_tag(tag);
                            },
                            Some(ActionTarget::Location(pos)) => {
                                let target = cmd.mut_target_world_space_pos();
                                target.set_x(pos.x);
                                target.set_y(pos.y);
                            },
                            None => (),
                        }

                        for u in units {
                            cmd.mut_unit_tags().push(u.tag);
                        }
                    }

                    req.mut_action().mut_actions().push(a);
                },
            }
        }

        await!(self.client.clone().request(req))?;

        Ok(Self {
            command_batch: vec![],
            ..self
        })
    }

    #[async]
    fn send_debug(self) -> Result<Self> {
        let mut req = sc2api::Request::new();
        req.mut_debug().mut_debug();

        for cmd in self.debug_batch {
            match cmd {
                DebugCommand::Text {
                    text,
                    target,
                    color,
                } => {
                    let mut cmd = debug::DebugCommand::new();
                    let mut debug_text = debug::DebugText::new();

                    debug_text.set_text(text);

                    match target {
                        Some(DebugTextTarget::Screen(p)) => {
                            debug_text.mut_virtual_pos().set_x(p.x);
                            debug_text.mut_virtual_pos().set_y(p.y);
                        },
                        Some(DebugTextTarget::World(p)) => {
                            debug_text.mut_world_pos().set_x(p.x);
                            debug_text.mut_world_pos().set_y(p.y);
                            debug_text.mut_world_pos().set_z(p.z);
                        },
                        None => (),
                    }

                    debug_text.mut_color().set_r(color.0 as u32);
                    debug_text.mut_color().set_g(color.1 as u32);
                    debug_text.mut_color().set_b(color.2 as u32);

                    cmd.mut_draw().mut_text().push(debug_text);
                    req.mut_debug().mut_debug().push(cmd);
                },
                DebugCommand::Line { p1, p2, color } => {
                    let mut cmd = debug::DebugCommand::new();
                    let mut debug_line = debug::DebugLine::new();

                    debug_line.mut_line().mut_p0().set_x(p1.x);
                    debug_line.mut_line().mut_p0().set_y(p1.y);
                    debug_line.mut_line().mut_p0().set_z(p1.z);

                    debug_line.mut_line().mut_p1().set_x(p2.x);
                    debug_line.mut_line().mut_p1().set_y(p2.y);
                    debug_line.mut_line().mut_p1().set_z(p2.z);

                    debug_line.mut_color().set_r(color.0 as u32);
                    debug_line.mut_color().set_g(color.1 as u32);
                    debug_line.mut_color().set_b(color.2 as u32);

                    cmd.mut_draw().mut_lines().push(debug_line);
                    req.mut_debug().mut_debug().push(cmd);
                },
                DebugCommand::Box { min, max, color } => {
                    let mut cmd = debug::DebugCommand::new();
                    let mut debug_box = debug::DebugBox::new();

                    debug_box.mut_min().set_x(min.x);
                    debug_box.mut_min().set_y(min.y);
                    debug_box.mut_min().set_z(min.z);

                    debug_box.mut_max().set_x(max.x);
                    debug_box.mut_max().set_y(max.y);
                    debug_box.mut_max().set_z(max.z);

                    debug_box.mut_color().set_r(color.0 as u32);
                    debug_box.mut_color().set_g(color.1 as u32);
                    debug_box.mut_color().set_b(color.2 as u32);

                    cmd.mut_draw().mut_boxes().push(debug_box);
                    req.mut_debug().mut_debug().push(cmd);
                },
                DebugCommand::Sphere {
                    center,
                    radius,
                    color,
                } => {
                    let mut cmd = debug::DebugCommand::new();
                    let mut debug_sphere = debug::DebugSphere::new();

                    debug_sphere.mut_p().set_x(center.x);
                    debug_sphere.mut_p().set_y(center.y);
                    debug_sphere.mut_p().set_z(center.z);

                    debug_sphere.set_r(radius);

                    debug_sphere.mut_color().set_r(color.0 as u32);
                    debug_sphere.mut_color().set_g(color.1 as u32);
                    debug_sphere.mut_color().set_b(color.2 as u32);

                    cmd.mut_draw().mut_spheres().push(debug_sphere);
                    req.mut_debug().mut_debug().push(cmd);
                },
            }
        }

        await!(self.client.clone().request(req))?;

        Ok(Self {
            debug_batch: vec![],
            ..self
        })
    }
}

#[derive(Debug)]
enum ActionControlRequest {
    Step(oneshot::Sender<()>),
}

#[derive(Debug)]
enum ActionRequest {
    SendCommand(Command, oneshot::Sender<()>),
    SendDebug(DebugCommand, oneshot::Sender<()>),
}

#[derive(Debug)]
enum Either {
    Control(ActionControlRequest),
    Request(ActionRequest),
}

#[derive(Debug, Clone)]
pub struct ActionControlTerminal {
    tx: mpsc::Sender<ActionControlRequest>,
}

impl ActionControlTerminal {
    /// step the action soma and send all commands to the game instance
    #[async]
    pub fn step(self) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(ActionControlRequest::Step(tx))
                .map(|_| ())
                .map_err(|_| Error::from("unable to send debug command"))
        )?;
        await!(rx.map_err(|_| Error::from("unable to send debug ack")))
    }
}

#[derive(Debug)]
pub struct ActionControlDendrite {
    rx: mpsc::Receiver<ActionControlRequest>,
}

/// action interface for a game instance
#[derive(Debug, Clone)]
pub struct ActionTerminal {
    tx: mpsc::Sender<ActionRequest>,
}

impl ActionTerminal {
    /// send a command to the game instance
    #[async]
    pub fn send_command(self, cmd: Command) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(ActionRequest::SendCommand(cmd, tx))
                .map(|_| ())
                .map_err(|_| Error::from("unable to send command"))
        )?;
        await!(rx.map_err(|_| Error::from("unable to recv send command ack")))
    }

    /// send a debug command to the game instance
    #[async]
    pub fn send_debug(self, cmd: DebugCommand) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(ActionRequest::SendDebug(cmd, tx))
                .map(|_| ())
                .map_err(|_| Error::from("unable to send debug command"))
        )?;
        await!(rx.map_err(|_| Error::from("unable to send debug ack")))
    }
}

/// internal action receiver for action soma
#[derive(Debug)]
pub struct ActionDendrite {
    rx: mpsc::Receiver<ActionRequest>,
}
