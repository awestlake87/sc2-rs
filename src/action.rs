use std::mem;

use futures::prelude::*;
use futures::unsync::{mpsc, oneshot};
use organelle::{Axon, Constraint, Impulse, Soma};
use sc2_proto::sc2api;
use tokio_core::reactor;

use super::{Error, IntoProto, Result};
use client::ProtoClient;
use data::{Action, DebugCommand};
use synapses::{Dendrite, Synapse, Terminal};

pub struct ActionBuilder {
    control_tx: mpsc::Sender<ActionControlRequest>,
    control_rx: mpsc::Receiver<ActionControlRequest>,

    action_tx: mpsc::Sender<ActionRequest>,
    action_rx: mpsc::Receiver<ActionRequest>,

    client: Option<ProtoClient>,
}

impl ActionBuilder {
    pub fn new() -> Self {
        let (control_tx, control_rx) = mpsc::channel(1);
        let (action_tx, action_rx) = mpsc::channel(10);

        Self {
            control_tx: control_tx,
            control_rx: control_rx,

            action_tx: action_tx,
            action_rx: action_rx,

            client: None
        }
    }

    pub fn client(self, client: ProtoClient) -> Self {
        Self {
            client: Some(client),
            ..self
        }
    }

    pub fn fork_control(&self) -> ActionControlClient {
        ActionControlClient { tx: self.control_tx.clone() }
    }

    pub fn fork(&self) -> ActionClient {
        ActionClient { tx: self.action_tx.clone() }
    }

    pub fn spawn(self, handle: &reactor::Handle) -> Result<()> {
        let task = ActionTask::new(
            self.client.unwrap(),
            self.control_rx,
            self.action_rx,
        );

        handle.spawn(task.run()
                .map_err(|_| ())
        );

        Ok(())
    }
}

struct ActionTask {
    client: ProtoClient,
    control_rx: Option<mpsc::Receiver<ActionControlRequest>>,
    action_rx: Option<mpsc::Receiver<ActionRequest>>,

    action_batch: Vec<Action>,
    debug_batch: Vec<DebugCommand>,
}

impl ActionTask {
    fn new(
        client: ProtoClient,
        control_rx: mpsc::Receiver<ActionControlRequest>,
        action_rx: mpsc::Receiver<ActionRequest>,
    ) -> Self {
        Self {
            client: client,
            control_rx: Some(control_rx),
            action_rx: Some(action_rx),

            action_batch: vec![],
            debug_batch: vec![],
        }
    }

    #[async]
    fn run(mut self) -> Result<()> {
        let requests = mem::replace(&mut self.action_rx, None).unwrap();

        let queue = mem::replace(&mut self.control_rx, None)
            .unwrap()
            .map(|req| Either::Control(req))
            .select(requests.map(|req| Either::Request(req)));

        #[async]
        for req in queue.map_err(|_| -> Error { unreachable!() }) {
            match req {
                Either::Control(ActionControlRequest::Step(tx)) => {
                    self = await!(self.send_actions())?;
                    self = await!(self.send_debug())?;

                    tx.send(()).map_err(|_| Error::from("unable to ack step"))?;
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
            req.mut_action().mut_actions().push(action.into_proto()?);
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
            req.mut_debug().mut_debug().push(cmd.into_proto()?);
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
pub struct ActionControlClient {
    tx: mpsc::Sender<ActionControlRequest>,
}

impl ActionControlClient {
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
/// action interface for a game instance
#[derive(Debug, Clone)]
pub struct ActionClient {
    tx: mpsc::Sender<ActionRequest>,
}

impl ActionClient {
    /// send a command to the game instance
    pub fn send_action(
        &self,
        action: Action,
    ) -> impl Future<Item = (), Error = Error> {
        let (tx, rx) = oneshot::channel();
        let sender = self.tx.clone();

        async_block! {
            await!(
                sender
                    .send(ActionRequest::SendAction(action, tx))
                    .map(|_| ())
                    .map_err(|_| Error::from("unable to send command"))
            )?;
            await!(rx.map_err(|_| Error::from("unable to recv send command ack")))
        }
    }

    /// send a debug command to the game instance
    pub fn send_debug<T>(&self, cmd: T) -> impl Future<Item = (), Error = Error>
    where
        T: Into<DebugCommand> + 'static,
    {
        let (tx, rx) = oneshot::channel();
        let sender = self.tx.clone();

        async_block! {
            await!(
                sender
                    .send(ActionRequest::SendDebug(cmd.into(), tx))
                    .map(|_| ())
                    .map_err(|_| Error::from("unable to send debug command"))
            )?;
            await!(rx.map_err(|_| Error::from("unable to send debug ack")))
        }
    }
}