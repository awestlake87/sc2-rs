use futures::prelude::*;
use futures::unsync::{mpsc, oneshot};
use organelle::{Axon, Constraint, Impulse, Soma};
use tokio_core::reactor;

use super::{Error, Result};
use client::ClientTerminal;
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
                    handle: handle.clone(),
                    client: self.client.unwrap(),
                    queue: rx,
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
    handle: reactor::Handle,
    client: ClientTerminal,
    queue: mpsc::Receiver<ActionRequest>,
}

impl ActionTask {
    #[async]
    fn run(self) -> Result<()> {
        #[async]
        for req in self.queue.map_err(|_| -> Error { unreachable!() }) {
            continue;
        }

        Ok(())
    }
}

#[derive(Debug)]
enum ActionRequest {

}

#[derive(Debug, Clone)]
pub struct ActionTerminal {
    tx: mpsc::Sender<ActionRequest>,
}
#[derive(Debug)]
pub struct ActionDendrite {
    rx: mpsc::Receiver<ActionRequest>,
}
