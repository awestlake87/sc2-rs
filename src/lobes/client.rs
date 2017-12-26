
use std::time::Duration;

use cortical;
use cortical::{ ResultExt, Handle, Lobe, Protocol };
use futures::prelude::*;
use tokio_timer::{ Timer };
use tokio_tungstenite::{ connect_async };
use url::Url;
use uuid::Uuid;

use super::super::{ Result };
use super::{ Message, Soma, Role };


const NUM_RETRIES: u32 = 10;

pub struct ClientLobe {
    soma:               Soma,

    provider:           RequiredOnce<Handle>,
    owner:              RequiredOnce<Handle>,

    instance:           Option<Uuid>,

    timer:              Timer,
    retries:            u32,
}

impl ClientLobe {
    pub fn new() -> Result<Self> {
        Ok(
            Self {
                effector: Soma::new(
                    vec![
                        Constraint::RequireOne(Role::InstanceProvider),
                        Constraint::RequireOne(Role::Client)
                    ],
                    vec![ ]
                )?,

                instance: None,

                timer: Timer::default(),
                retries: 0,
            }
        )
    }

    fn add_input(mut self, input: Handle, role: Role)
        -> Result<Self>
    {
        match role {
            Role::Client => self.owner.set(input)?,
            Role::InstanceProvider => self.provider.set(input)?,

            _ => bail!("invalid input role {:#?}", role)
        }

        Ok(self)
    }

    fn assign_instance(
        mut self, src: Handle, instance: Uuid, url: Url
    )
        -> Result<Self>
    {
        assert_eq!(src, *self.provider.get()?);

        if self.instance.is_none() {
            self.instance = Some(instance);
            self.retries = NUM_RETRIES;

            let this_lobe = self.soma.effector()?.this_lobe();
            self.soma.send(this_lobe, Message::AttemptConnect(url))?;

            Ok(self)
        }
        else {
            bail!("instance has already been assigned")
        }
    }

    fn attempt_connect(mut self, src: Handle, url: Url) -> Result<Self> {
        assert_eq!(src, self.soma.this_lobe()?);

        let connected_effector = self.soma.effector()?.clone();
        let retry_effector = self.soma.effector()?.clone();
        let timer_effector = self.soma.effector()?.clone();

        let client_remote = self.soma.effector()?.remote();

        let owner = self.soma.req_input(Role::Client)?;

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

        self.soma.spawn(
            self.timer.sleep(Duration::from_secs(5))
                .and_then(
                    move |_| connect_async(url, client_remote)
                        .and_then(
                            move |(_ws_stream, _)| {
                                connected_effector.send(
                                    owner, Message::Connected
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
        )?;

        Ok(self)
    }
}

impl Lobe for ClientLobe {
    type Message = Message;
    type Role = Role;

    fn update(self, msg: Protocol<Message, Role>)
        -> cortical::Result<Self>
    {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(
                src, Message::ProvideInstance(instance, url)
            ) => {
                self.assign_instance(src, instance, url)
            },
            Protocol::Message(src, Message::AttemptConnect(url)) => {
                self.attempt_connect(src, url)
            },

            _ => Ok(self),
        }.chain_err(
            || cortical::ErrorKind::LobeError
        )
    }
}
