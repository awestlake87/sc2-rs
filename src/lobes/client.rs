
use std::time::Duration;

use cortical;
use cortical::{ ResultExt, Handle, Lobe, Protocol, Constraint };
use futures::prelude::*;
use tokio_timer::{ Timer };
use tokio_tungstenite::{ connect_async };
use url::Url;
use uuid::Uuid;

use super::super::{ Result };
use super::{ Message, Soma, Role };


const NUM_RETRIES: u32 = 10;

pub enum ClientLobe {
    Init(ClientInit),
    AwaitInstance(ClientAwaitInstance),
    Connect(ClientConnect),

    GameSetup(ClientGameSetup),
}

impl ClientLobe {
    pub fn new() -> Result<Self> {
        Ok(
            ClientLobe::Init(
                ClientInit {
                    soma: Soma::new(
                        vec![
                            Constraint::RequireOne(Role::InstanceProvider),
                            Constraint::RequireOne(Role::Client)
                        ],
                        vec![ ]
                    )?,
                }
            )
        )
    }
}

impl Lobe for ClientLobe {
    type Message = Message;
    type Role = Role;

    fn update(self, msg: Protocol<Message, Role>) -> cortical::Result<Self> {
        match self {
            ClientLobe::Init(state) => state.update(msg),
            ClientLobe::AwaitInstance(state) => state.update(msg),
            ClientLobe::Connect(state) => state.update(msg),
            ClientLobe::GameSetup(state) => state.update(msg),
        }.chain_err(
            || cortical::ErrorKind::LobeError
        )
    }
}

pub struct ClientInit {
    soma:               Soma
}

impl ClientInit {
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<ClientLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Start => self.start(),

            _ => Ok(ClientLobe::Init(self))
        }
    }

    fn start(self) -> Result<ClientLobe> {
        Ok(ClientLobe::AwaitInstance(ClientAwaitInstance { soma: self.soma }))
    }
}

pub struct ClientAwaitInstance {
    soma:               Soma,
}

impl ClientAwaitInstance {
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<ClientLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(
                src, Message::ProvideInstance(instance, url)
            ) => {
                self.assign_instance(src, instance, url)
            },

            _ => Ok(ClientLobe::AwaitInstance(self))
        }
    }

    fn assign_instance(self, src: Handle, _: Uuid, url: Url)
        -> Result<ClientLobe>
    {
        assert_eq!(src, self.soma.req_input(Role::InstanceProvider)?);

        let this_lobe = self.soma.effector()?.this_lobe();
        self.soma.effector()?.send(
            this_lobe, Message::AttemptConnect(url)
        );

        Ok(
            ClientLobe::Connect(
                ClientConnect {
                    soma: self.soma,

                    timer: Timer::default(),
                    retries: NUM_RETRIES,
                }
            )
        )
    }
}

pub struct ClientConnect {
    soma:               Soma,

    timer:              Timer,
    retries:            u32,
}

impl ClientConnect {
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<ClientLobe> {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(src, Message::AttemptConnect(url)) => {
                self.attempt_connect(src, url)
            },
            Protocol::Message(src, Message::Connected) => {
                self.on_connected(src)
            },

            _ => Ok(ClientLobe::Connect(self))
        }
    }

    fn attempt_connect(mut self, src: Handle, url: Url) -> Result<ClientLobe> {
        assert_eq!(src, self.soma.effector()?.this_lobe());

        let connected_effector = self.soma.effector()?.clone();
        let retry_effector = self.soma.effector()?.clone();
        let timer_effector = self.soma.effector()?.clone();

        let client_remote = self.soma.effector()?.remote();

        if self.retries == 0 {
            bail!("unable to connect to instance")
        }
        else {
            println!(
                "attempting to connect to instance {} - retries {}",
                url,
                self.retries
            );

            self.retries -= 1;
        }

        let retry_url = url.clone();

        self.soma.effector()?.spawn(
            self.timer.sleep(Duration::from_secs(5))
                .and_then(
                    move |_| connect_async(url, client_remote)
                        .and_then(
                            move |(_ws_stream, _)| {
                                let this_lobe = connected_effector.this_lobe();
                                connected_effector.send(
                                    this_lobe, Message::Connected
                                );

                                Ok(())
                            }
                        )
                        .or_else(
                            move |_| {
                                let this_lobe = retry_effector.this_lobe();
                                retry_effector.send(
                                    this_lobe,
                                    Message::AttemptConnect(retry_url)
                                );

                                Ok(())
                            }
                        )
                )
                .or_else(
                    move |e| {
                        timer_effector.error(
                            cortical::Error::with_chain(
                                e, cortical::ErrorKind::LobeError
                            )
                        );

                        Ok(())
                    }
                )
        );

        Ok(ClientLobe::Connect(self))
    }

    fn on_connected(self, src: Handle) -> Result<ClientLobe> {
        assert_eq!(src, self.soma.effector()?.this_lobe());

        self.soma.send_req_input(Role::Client, Message::Connected)?;

        Ok(ClientLobe::GameSetup(ClientGameSetup { soma: self.soma }))
    }
}

pub struct ClientGameSetup {
    soma:           Soma,
}

impl ClientGameSetup {
    fn update(mut self, msg: Protocol<Message, Role>) -> Result<ClientLobe> {
        self.soma.update(&msg)?;

        match msg {
            _ => Ok(ClientLobe::GameSetup(self))
        }
    }
}
