
use cortical;
use cortical::{ Lobe, Handle, Protocol, ResultExt };
use url::Url;
use uuid::Uuid;

use super::super::{ Result, LauncherSettings };
use data::{ GameSettings };
use lobes::{ Message, Role, Effector, Cortex, RequiredOnce };
use lobes::agent::{ AgentLobe };
use lobes::client::{ ClientLobe };
use lobes::launcher::{ LauncherLobe };

/// suite of games to choose from when pitting bots against each other
pub enum MeleeSuite {
    /// play one game with the given settings
    OneAndDone(GameSettings),
}

/// settings for the melee lobe
pub struct MeleeSettings<L1: Lobe + 'static, L2: Lobe + 'static> {
    /// the settings for the launcher lobe
    pub launcher:   LauncherSettings,
    /// the player cortices
    pub players:    (L1, L2),
    /// the suite of games to choose from
    pub suite:      MeleeSuite,
}

/// lobe designed to pit two bots against each other in Sc2 games
pub struct MeleeLobe {
    effector:       RequiredOnce<Effector>,

    launcher:       RequiredOnce<Handle>,

    agent1:         RequiredOnce<Handle>,
    agent2:         RequiredOnce<Handle>,

    client1:        RequiredOnce<Handle>,
    client2:        RequiredOnce<Handle>,

    instances:      Option<((Uuid, Url), (Uuid, Url))>,

    suite:          MeleeSuite,

    ready:          (bool, bool),
}

impl MeleeLobe {
    /// melee lobe only works as a controller in a melee cortex
    fn new(suite: MeleeSuite) -> Self {
        Self {
            effector: RequiredOnce::new(),

            launcher: RequiredOnce::new(),

            agent1: RequiredOnce::new(),
            agent2: RequiredOnce::new(),

            client1: RequiredOnce::new(),
            client2: RequiredOnce::new(),

            instances: None,

            suite: suite,

            ready: (false, false),
        }
    }

    /// create the melee cortex
    pub fn cortex<L1, L2>(settings: MeleeSettings<L1, L2>) -> Result<Cortex>
        where
            L1: Lobe,
            L2: Lobe,

            Message: From<L1::Message> + From<L2::Message>,
            Role: From<L1::Role> + From<L2::Role>,

            L1::Message: From<Message>,
            L2::Message: From<Message>,

            L1::Role: From<Role>,
            L2::Role: From<Role>,
    {
        let mut cortex = Cortex::new(MeleeLobe::new(settings.suite));

        let launcher = cortex.add_lobe(LauncherLobe::from(settings.launcher)?);

        let melee = cortex.get_main_handle();

        let player1 = cortex.add_lobe(settings.players.0);
        let player2 = cortex.add_lobe(settings.players.1);

        let agent1 = cortex.add_lobe(AgentLobe::new());
        let agent2 = cortex.add_lobe(AgentLobe::new());

        let client1 = cortex.add_lobe(ClientLobe::new());
        let client2 = cortex.add_lobe(ClientLobe::new());

        cortex.connect(melee, launcher, Role::Launcher);

        cortex.connect(melee, agent1, Role::Controller);
        cortex.connect(melee, agent2, Role::Controller);
        cortex.connect(melee, client1, Role::InstanceProvider);
        cortex.connect(melee, client2, Role::InstanceProvider);

        cortex.connect(agent1, player1, Role::Agent);
        cortex.connect(agent2, player2, Role::Agent);

        cortex.connect(agent1, client1, Role::Client);
        cortex.connect(agent2, client2, Role::Client);

        Ok(cortex)
    }

    fn init(mut self, effector: Effector) -> Result<Self> {
        self.effector.set(effector)?;

        Ok(self)
    }

    fn add_output(mut self, output: Handle, role: Role)
        -> Result<Self>
    {
        match role {
            Role::Launcher => self.launcher.set(output)?,
            Role::Controller => {
                if !self.agent1.is_set() {
                    self.agent1.set(output)?;
                }
                else if !self.agent2.is_set() {
                    self.agent2.set(output)?;
                }
                else {
                    bail!("both agents are already specified")
                }
            },
            Role::InstanceProvider => {
                if !self.client1.is_set() {
                    self.client1.set(output)?;
                }
                else if !self.client2.is_set() {
                    self.client2.set(output)?;
                }
                else {
                    bail!("both clients are already specified")
                }
            },

            _ => bail!("invalid role {:#?}", role)
        }

        Ok(self)
    }

    fn start(self) -> Result<Self> {
        self.effector.get()?.send(
            *self.launcher.get()?, Message::LaunchInstance
        );
        self.effector.get()?.send(
            *self.launcher.get()?, Message::LaunchInstance
        );

        Ok(self)
    }

    fn on_instance_pool(mut self, src: Handle, instances: Vec<(Uuid, Url)>)
        -> Result<Self>
    {
        assert_eq!(src, *self.launcher.get()?);

        if instances.len() < 2 || self.instances.is_some() {
            Ok(self)
        }
        else {
            self.instances = Some(
                (instances[0].clone(), instances[1].clone())
            );

            let (i1, i2) = self.instances.clone().unwrap();

            self.effector.get()?.send(
                *self.client1.get()?, Message::ProvideInstance(i1.0, i1.1)
            );
            self.effector.get()?.send(
                *self.client2.get()?, Message::ProvideInstance(i2.0, i2.1)
            );

            self.ready = (false, false);

            Ok(self)
        }
    }

    fn on_agent_ready(mut self, src: Handle) -> Result<Self> {
        if src == *self.agent1.get()? {
            self.ready.0 = true;
        }
        else if src == *self.agent2.get()? {
            self.ready.1 = true;
        }
        else {
            bail!("expected source of Ready to be an agent")
        }

        if self.ready == (true, true) {
            let first_game = match &self.suite {
                &MeleeSuite::OneAndDone(ref game) => {
                    game.clone()
                },
            };

            self.start_game(first_game)
        }
        else {
            Ok(self)
        }
    }

    fn start_game(self, _: GameSettings) -> Result<Self> {
        println!("START GAME!");

        Ok(self)
    }
}

impl Lobe for MeleeLobe {
    type Message = Message;
    type Role = Role;

    fn update(self, msg: Protocol<Self::Message, Self::Role>)
        -> cortical::Result<Self>
    {
        match msg {
            Protocol::Init(effector) => self.init(effector),
            Protocol::AddOutput(output, role) => {
                self.add_output(output, role)
            },
            Protocol::Start => {
                self.start()
            },

            Protocol::Message(src, Message::InstancePool(instances)) => {
                self.on_instance_pool(src, instances)
            },

            Protocol::Message(src, Message::Ready) => {
                self.on_agent_ready(src)
            }

            _ => Ok(self),
        }.chain_err(
            || cortical::ErrorKind::LobeError
        )
    }
}
