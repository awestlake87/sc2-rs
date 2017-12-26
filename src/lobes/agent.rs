
use cortical;
use cortical::{ ResultExt, Handle, Lobe, Protocol, Constraint };

use super::super::{ Result };
use super::{ Message, Role, Soma };

use data::{ GameSettings, GamePorts, PlayerSetup };

pub struct AgentLobe {
    soma:               Soma,
}

impl AgentLobe {
    pub fn new() -> Result<Self> {
        Ok(
            Self {
                soma: Soma::new(
                    vec![ Constraint::RequireOne(Role::Controller) ],
                    vec![
                        Constraint::RequireOne(Role::Client),
                        Constraint::RequireOne(Role::Agent),
                    ],
                )?,
            }
        )
    }

    fn on_connected(self, src: Handle) -> Result<Self> {
        assert_eq!(src, self.soma.req_output(Role::Client)?);

        self.soma.send_req_input(Role::Controller, Message::Ready)?;

        Ok(self)
    }

    fn on_req_player_setup(self, src: Handle, settings: GameSettings)
        -> Result<Self>
    {
        assert_eq!(src, self.soma.req_input(Role::Controller)?);

        self.soma.send_req_output(
            Role::Agent, Message::RequestPlayerSetup(settings)
        )?;

        Ok(self)
    }

    fn on_player_setup(self, src: Handle, setup: PlayerSetup) -> Result<Self> {
        assert_eq!(src, self.soma.req_output(Role::Agent)?);

        self.soma.send_req_input(
            Role::Controller, Message::PlayerSetup(setup)
        )?;

        Ok(self)
    }

    fn create_game(
        self, src: Handle, settings: GameSettings, players: Vec<PlayerSetup>
    )
        -> Result<Self>
    {
        assert_eq!(src, self.soma.req_input(Role::Controller)?);

        println!("create game with settings: {:#?}", settings);
        println!("fake it for now");

        self.soma.send_req_input(
            Role::Controller, Message::GameCreated
        )?;

        Ok(self)
    }

    fn on_game_ready(
        self, src: Handle, setup: PlayerSetup, ports: GamePorts
    )
        -> Result<Self>
    {
        assert_eq!(src, self.soma.req_input(Role::Controller)?);

        println!("join game with setup {:#?} and ports {:#?}", setup, ports);
        println!("fake it for now");

        Ok(self)
    }
}

impl Lobe for AgentLobe {
    type Message = Message;
    type Role = Role;

    fn update(mut self, msg: Protocol<Message, Role>)
        -> cortical::Result<Self>
    {
        self.soma.update(&msg)?;

        match msg {
            Protocol::Message(src, Message::Connected) => {
                self.on_connected(src)
            },

            Protocol::Message(src, Message::RequestPlayerSetup(settings)) => {
                self.on_req_player_setup(src, settings)
            },
            Protocol::Message(src, Message::PlayerSetup(setup)) => {
                self.on_player_setup(src, setup)
            },
            Protocol::Message(src, Message::CreateGame(settings, players)) => {
                self.create_game(src, settings, players)
            },
            Protocol::Message(src, Message::GameReady(setup, ports)) => {
                self.on_game_ready(src, setup, ports)
            }

            _ => Ok(self),
        }.chain_err(
            || cortical::ErrorKind::LobeError
        )
    }
}
