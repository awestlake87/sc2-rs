use std::mem;

use futures::prelude::*;
use futures::unsync::{mpsc, oneshot};
use organelle::{Axon, Constraint, Impulse, Soma};
use sc2_proto::sc2api;

use super::{Error, IntoProto, Result};
use client::ProtoClient;
use data::{Action, DebugCommand};
use synapses::{Dendrite, Synapse};

pub struct ActionSoma {
    control: Option<ActionControlDendrite>,
    client: Option<ProtoClient>,
    users: Vec<ActionDendrite>,
}

impl ActionSoma {
    pub fn axon(client: ProtoClient) -> Result<Axon<Self>> {
        Ok(Axon::new(
            Self {
                control: None,
                client: Some(client),
                users: vec![],
            },
            vec![
                Constraint::Variadic(Synapse::Action),
                Constraint::One(Synapse::ActionControl),
            ],
            vec![],
        ))
    }
}

pub fn synapse() -> (ActionTerminal, ActionDendrite) {
    let (tx, rx) = mpsc::channel(10);

    (
        ActionTerminal { tx: tx },
        ActionDendrite { rx: rx },
    )
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
    client: ProtoClient,
    control: Option<ActionControlDendrite>,
    queue: Option<mpsc::Receiver<ActionRequest>>,

    action_batch: Vec<Action>,
    debug_batch: Vec<DebugCommand>,
}

impl ActionTask {
    fn new(
        client: ProtoClient,
        control: ActionControlDendrite,
        rx: mpsc::Receiver<ActionRequest>,
    ) -> Self {
        Self {
            client: client,
            control: Some(control),
            queue: Some(rx),

            action_batch: vec![],
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
                    self = await!(self.send_actions())?;
                    self = await!(self.send_debug())?;

                    tx.send(())
                        .map_err(|_| Error::from("unable to ack step"))?;
                },
                Either::Request(ActionRequest::SendAction(action, tx)) => {
                    self.action_batch.push(action);
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
    fn send_actions(self) -> Result<Self> {
        let mut req = sc2api::Request::new();
        req.mut_action().mut_actions();

        for action in self.action_batch {
            req.mut_action()
                .mut_actions()
                .push(action.into_proto()?);
        }

        await!(self.client.clone().request(req))?;

        Ok(Self {
            action_batch: vec![],
            ..self
        })
    }

    #[async]
    fn send_debug(self) -> Result<Self> {
        let mut req = sc2api::Request::new();
        req.mut_debug().mut_debug();

        for cmd in self.debug_batch {
            req.mut_debug()
                .mut_debug()
                .push(cmd.into_proto()?);
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
    SendAction(Action, oneshot::Sender<()>),
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
    pub fn send_action(self, action: Action) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(ActionRequest::SendAction(action, tx))
                .map(|_| ())
                .map_err(|_| Error::from("unable to send command"))
        )?;
        await!(rx.map_err(|_| Error::from("unable to recv send command ack")))
    }

    /// send a debug command to the game instance
    #[async]
    pub fn send_debug<T>(self, cmd: T) -> Result<()>
    where
        T: Into<DebugCommand> + 'static,
    {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(ActionRequest::SendDebug(cmd.into(), tx))
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
