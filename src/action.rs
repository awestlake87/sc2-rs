use std::mem;

use futures::prelude::*;
use futures::unsync::{mpsc, oneshot};
use organelle::{Axon, Constraint, Impulse, Soma};
use sc2_proto::sc2api;

use super::{Error, IntoProto, Result};
use client::ClientTerminal;
use data::{ActionTarget, Command};
use synapses::{Dendrite, Synapse, Terminal};

pub struct ActionSoma {
    client: Option<ClientTerminal>,
    users: Vec<ActionDendrite>,
}

impl ActionSoma {
    pub fn axon() -> Result<Axon<Self>> {
        Ok(Axon::new(
            Self {
                client: None,
                users: vec![],
            },
            vec![Constraint::Variadic(Synapse::Action)],
            vec![Constraint::One(Synapse::Client)],
        ))
    }
}

pub fn synapse() -> (ActionTerminal, ActionDendrite) {
    let (tx, rx) = mpsc::channel(10);

    (ActionTerminal { tx: tx }, ActionDendrite { rx: rx })
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

                let task = ActionTask {
                    client: self.client.unwrap(),
                    queue: Some(rx),
                };

                handle.spawn(task.run().or_else(move |e| {
                    main_tx
                        .send(Impulse::Error(e.into()))
                        .map(|_| ())
                        .map_err(|_| ())
                }));

                Ok(Self {
                    client: None,
                    users: vec![],
                })
            },

            _ => bail!("unexpected impulse"),
        }
    }
}

struct ActionTask {
    client: ClientTerminal,
    queue: Option<mpsc::Receiver<ActionRequest>>,
}

impl ActionTask {
    #[async]
    fn run(mut self) -> Result<()> {
        let queue = mem::replace(&mut self.queue, None).unwrap();
        #[async]
        for req in queue.map_err(|_| -> Error { unreachable!() }) {
            match req {
                ActionRequest::SendCommand(cmd, tx) => {
                    self = await!(self.send_command(cmd))?;
                    tx.send(()).map_err(|_| {
                        Error::from("unable to ack send command")
                    })?;
                },
            }
        }

        Ok(())
    }

    #[async]
    fn send_command(self, cmd: Command) -> Result<Self> {
        let mut req = sc2api::Request::new();
        req.mut_action().mut_actions();

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

        await!(self.client.clone().request(req))?;

        Ok(self)
    }
}

#[derive(Debug)]
enum ActionRequest {
    SendCommand(Command, oneshot::Sender<()>),
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
}

/// internal action receiver for action soma
#[derive(Debug)]
pub struct ActionDendrite {
    rx: mpsc::Receiver<ActionRequest>,
}
