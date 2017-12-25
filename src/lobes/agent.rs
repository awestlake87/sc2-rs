
use std::time::Duration;

use cortical;
use cortical::{ ResultExt, Handle, Lobe, Protocol };
use futures::prelude::*;
use tokio_timer::{ Timer };
use tokio_tungstenite::{ connect_async };
use tungstenite::{ Message as WebsockMessage };
use url::Url;
use uuid::Uuid;

use super::super::{ Result };
use super::{ Message, Effector, Role };

use data::{ GameSettings };

const NUM_RETRIES: u32 = 10;

pub struct AgentLobe {
    effector:           Option<Effector>,

    controller:         Option<Handle>,
    player:             Option<Handle>,

    instance:           Option<Uuid>,

    timer:              Timer,
    retries:            u32,
}

impl AgentLobe {
    pub fn new() -> Self {
        Self {
            effector: None,

            controller: None,
            player: None,

            instance: None,

            timer: Timer::default(),
            retries: 0,
        }
    }

    fn effector(&self) -> &Effector {
        self.effector.as_ref().unwrap()
    }

    fn init(mut self, effector: Effector) -> Result<Self> {
        self.effector = Some(effector);

        Ok(self)
    }

    fn add_input(mut self, input: Handle, role: Role)
        -> Result<Self>
    {
        match role {
            Role::Controller => {
                if self.controller.is_none() {
                    self.controller = Some(input);

                    Ok(self)
                }
                else {
                    bail!("agent can only have 1 assigner lobe")
                }
            },

            _ => bail!("invalid input role {:#?}", role)
        }
    }

    fn add_output(mut self, output: Handle, role: Role)
        -> Result<Self>
    {
        match role {
            Role::Agent => {
                if self.player.is_none() {
                    self.player = Some(output);

                    Ok(self)
                }
                else {
                    bail!("agent should only manage 1 player")
                }
            },

            _ => bail!("invalid output role {:#?}", role)
        }
    }

    fn assign_instance(
        mut self, src: Handle, instance: Uuid, url: Url
    ) -> Result<Self> {
        assert_eq!(src, self.controller.unwrap());

        if self.instance.is_none() {
            self.instance = Some(instance);
            self.retries = NUM_RETRIES;

            let this_lobe = self.effector().this_lobe();
            self.effector().send(this_lobe, Message::AttemptConnect(url));

            Ok(self)
        }
        else {
            bail!("instance has already been assigned")
        }
    }

    fn attempt_connect(mut self, url: Url) -> Result<Self> {
        let client_effector = self.effector().clone();
        let client_remote = self.effector().remote();
        let timer_effector = self.effector().clone();

        if self.retries == 0 {
            bail!("unable to connect to instance")
        }
        else {
            println!("attempting to connect to instance {}", url);
            self.retries -= 1;
        }

        let retry_url = url.clone();

        self.effector().spawn(
            self.timer.sleep(Duration::from_secs(5))
                .and_then(
                    move |_| connect_async(url, client_remote)
                        .and_then(
                            |(ws_stream, _)| {
                                println!("CONNECTED!!!!!");
                                Ok(())
                            }
                        )
                        .or_else(
                            move |e| {
                                let this_lobe = client_effector.this_lobe();
                                client_effector.send(
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

        Ok(self)
    }

    fn create_game(self, src: Handle, settings: GameSettings) -> Result<Self> {
        assert_eq!(src, self.controller.unwrap());

        self.effector().send(
            self.player.unwrap(), Message::CreateGame(settings)
        );

        Ok(self)
    }
}

impl Lobe for AgentLobe {
    type Message = Message;
    type Role = Role;

    fn update(mut self, msg: Protocol<Message, Role>)
        -> cortical::Result<Self>
    {
        match msg {
            Protocol::Init(effector) => self.init(effector),
            Protocol::AddInput(input, role) => {
                self.add_input(input, role)
            },
            Protocol::AddOutput(output, role) => {
                self.add_output(output, role)
            },

            Protocol::Message(src, Message::AssignInstance(instance, url)) => {
                self.assign_instance(src, instance, url)
            },
            Protocol::Message(src, Message::AttemptConnect(url)) => {
                assert_eq!(src, self.effector().this_lobe());

                self.attempt_connect(url)
            }
            Protocol::Message(src, Message::CreateGame(settings)) => {
                self.create_game(src, settings)
            },

            _ => Ok(self),
        }.chain_err(
            || cortical::ErrorKind::LobeError
        )
    }
}
