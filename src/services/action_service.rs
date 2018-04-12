use std::mem;

use futures::prelude::*;
use futures::unsync::{mpsc, oneshot};
use sc2_proto::sc2api;
use tokio_core::reactor;

use super::client_service::ProtoClient;
use action::Action;
use constants::sc2_bug_tag;
use data::DebugCommand;
use {Error, IntoProto, Result};

pub struct ActionBuilder {
    client: Option<ProtoClient>,

    control_tx: mpsc::Sender<ActionControlRequest>,
    control_rx: mpsc::Receiver<ActionControlRequest>,

    user_tx: mpsc::Sender<ActionRequest>,
    user_rx: mpsc::Receiver<ActionRequest>,
}

impl ActionBuilder {
    pub fn new() -> Self {
        let (control_tx, control_rx) = mpsc::channel(10);
        let (user_tx, user_rx) = mpsc::channel(10);

        Self {
            client: None,

            control_tx: control_tx,
            control_rx: control_rx,

            user_tx: user_tx,
            user_rx: user_rx,
        }
    }

    pub fn proto_client(self, client: ProtoClient) -> Self {
        Self {
            client: Some(client),
            ..self
        }
    }

    pub fn add_action_client(&self) -> ActionClient {
        ActionClient {
            tx: self.user_tx.clone(),
        }
    }

    pub fn add_debug_client(&self) -> DebugClient {
        DebugClient {
            tx: self.user_tx.clone(),
        }
    }

    pub fn add_control_client(&self) -> ActionControlClient {
        ActionControlClient {
            tx: self.control_tx.clone(),
        }
    }

    pub fn spawn(self, handle: &reactor::Handle) -> Result<()> {
        let service = ActionService::new(
            self.client.unwrap(),
            self.control_rx,
            self.user_rx,
        );

        handle.spawn(service.run().map_err(|e| {
            panic!(
                "{}: ActionService exited unexpectedly - {:#?}",
                sc2_bug_tag(),
                e,
            )
        }));

        Ok(())
    }
}

struct ActionService {
    client: ProtoClient,
    control: Option<mpsc::Receiver<ActionControlRequest>>,
    queue: Option<mpsc::Receiver<ActionRequest>>,

    action_batch: Vec<Action>,
    debug_batch: Vec<DebugCommand>,
}

impl ActionService {
    fn new(
        client: ProtoClient,
        control: mpsc::Receiver<ActionControlRequest>,
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
            .map(|req| Either::Control(req))
            .select(requests.map(|req| Either::Request(req)));

        #[async]
        for req in queue.map_err(|_| -> Error { unreachable!() }) {
            match req {
                Either::Control(ActionControlRequest::Step(tx)) => {
                    self = await!(self.send_actions())?;
                    self = await!(self.send_debug())?;

                    tx.send(()).expect(&format!(
                        "{}: Unable to ack Step in ActionService",
                        sc2_bug_tag()
                    ));
                },
                Either::Request(ActionRequest::SendAction(action, tx)) => {
                    self.action_batch.push(action);
                    tx.send(()).expect(&format!(
                        "{}: Unable to ack SendAction in ActionService",
                        sc2_bug_tag()
                    ));
                },
                Either::Request(ActionRequest::SendDebug(cmd, tx)) => {
                    self.debug_batch.push(cmd);
                    tx.send(()).expect(&format!(
                        "{}: Unable to ack SendDebug in ActionService",
                        sc2_bug_tag()
                    ));
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
pub struct ActionControlClient {
    tx: mpsc::Sender<ActionControlRequest>,
}

impl ActionControlClient {
    /// Step the action service and send all commands to the game instance.
    #[async]
    pub fn step(self) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        await!(
            self.tx
                .send(ActionControlRequest::Step(tx))
                .map(|_| ())
                .map_err(|_| -> Error {
                    unreachable!("{}: Unable to req step", sc2_bug_tag())
                })
        )?;
        await!(rx.map_err(|_| -> Error {
            unreachable!("{}: Unable to ack step", sc2_bug_tag())
        }))
    }
}

/// Action interface for a game instance.
#[derive(Debug, Clone)]
pub struct ActionClient {
    tx: mpsc::Sender<ActionRequest>,
}

impl ActionClient {
    /// Send a command to the game instance.
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
                    .map_err(|_| -> Error { unreachable!("{}: Unable to req action ", sc2_bug_tag()) })
            )?;
            await!(rx.map_err(|_| -> Error { unreachable!("{}: Unable to ack action", sc2_bug_tag()) }))
        }
    }
}

/// Debug interface for a game instance.
#[derive(Debug, Clone)]
pub struct DebugClient {
    tx: mpsc::Sender<ActionRequest>,
}

impl DebugClient {
    /// Send a debug command to the game instance.
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
                    .map_err(|_| -> Error { unreachable!("{}: Unable to req debug command", sc2_bug_tag()) })
            )?;
            await!(rx.map_err(|_| -> Error { unreachable!("{}: Unable to ack debug command", sc2_bug_tag()) }))
        }
    }
}
