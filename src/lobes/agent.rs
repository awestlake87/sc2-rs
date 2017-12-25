
use cortical;
use cortical::{ ResultExt, Handle, Lobe, Protocol };

use super::super::{ Result };
use super::{ Message, Effector, Role, RequiredOnce };

use data::{ GameSettings };

pub struct AgentLobe {
    effector:           RequiredOnce<Effector>,

    controller:         RequiredOnce<Handle>,
    client:             RequiredOnce<Handle>,
    player:             RequiredOnce<Handle>,
}

impl AgentLobe {
    pub fn new() -> Self {
        Self {
            effector: RequiredOnce::new(),

            controller: RequiredOnce::new(),
            client: RequiredOnce::new(),
            player: RequiredOnce::new(),
        }
    }
    fn init(mut self, effector: Effector) -> Result<Self> {
        self.effector.set(effector)?;

        Ok(self)
    }

    fn add_input(mut self, input: Handle, role: Role)
        -> Result<Self>
    {
        match role {
            Role::Controller => self.controller.set(input)?,

            _ => bail!("invalid input role {:#?}", role)
        }

        Ok(self)
    }

    fn add_output(mut self, output: Handle, role: Role)
        -> Result<Self>
    {
        match role {
            Role::Agent => self.player.set(output)?,
            Role::Client => self.client.set(output)?,

            _ => bail!("invalid output role {:#?}", role)
        }

        Ok(self)
    }

    fn on_connected(self, src: Handle) -> Result<Self> {
        assert_eq!(src, *self.client.get()?);

        self.effector.get()?.send(*self.controller.get()?, Message::Ready);

        Ok(self)
    }

    fn create_game(self, src: Handle, settings: GameSettings) -> Result<Self> {
        assert_eq!(src, *self.controller.get()?);

        self.effector.get()?.send(
            *self.player.get()?, Message::CreateGame(settings)
        );

        Ok(self)
    }
}

impl Lobe for AgentLobe {
    type Message = Message;
    type Role = Role;

    fn update(self, msg: Protocol<Message, Role>)
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

            Protocol::Message(src, Message::Connected) => {
                self.on_connected(src)
            },

            Protocol::Message(src, Message::CreateGame(settings)) => {
                self.create_game(src, settings)
            },

            _ => Ok(self),
        }.chain_err(
            || cortical::ErrorKind::LobeError
        )
    }
}
