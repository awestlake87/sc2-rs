
use std::collections::HashMap;

use cortical;
use cortical::{ Lobe, Handle, Protocol, ResultExt };
use url::Url;
use uuid::Uuid;

use super::super::{ Result };
use data::{ GameSettings, GamePorts, PortSet, PlayerSetup };
use lobes::{ Message, Role, Effector, Cortex, RequiredOnce };
use lobes::agent::{ AgentLobe };
use lobes::client::{ ClientLobe };
use lobes::launcher::{ LauncherLobe, LauncherSettings };

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

    instances:      HashMap<Uuid, (Url, PortSet)>,
    ports:          Vec<GamePorts>,

    suite:          Option<MeleeSuite>,

    provided:       bool,
    ready:          (bool, bool),
    game_settings:  Option<GameSettings>,
    game_ports:     Option<GamePorts>,
    player_setup:   (Option<PlayerSetup>, Option<PlayerSetup>),
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

            instances: HashMap::new(),
            ports: vec![ ],

            suite: Some(suite),

            provided: false,
            ready: (false, false),
            game_settings: None,
            game_ports: None,
            player_setup: (None, None),
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

    fn start(mut self) -> Result<Self> {
        if self.suite.is_none() {
            self.effector.get()?.stop();
        }
        else {
            self.provided = false;

            self.effector.get()?.send(
                *self.launcher.get()?, Message::LaunchInstance
            );
            self.effector.get()?.send(
                *self.launcher.get()?, Message::LaunchInstance
            );
        }

        Ok(self)
    }

    fn on_instance_pool(
        mut self, src: Handle, instances: HashMap<Uuid, (Url, PortSet)>
    )
        -> Result<Self>
    {
        assert_eq!(src, *self.launcher.get()?);

        self.instances = instances;
        self.provide_instances()
    }

    fn on_ports_pool(mut self, src: Handle, ports: Vec<GamePorts>)
        -> Result<Self>
    {
        assert_eq!(src, *self.launcher.get()?);

        self.ports = ports;
        self.provide_instances()
    }

    fn provide_instances(mut self) -> Result<Self> {
        if self.ports.len() < 1 {
            // not enough game ports to start
            Ok(self)
        }
        else if self.instances.len() < 2 {
            // not enough instances to start
            Ok(self)
        }
        else if !self.provided {
            // we haven't already provided the instances
            let game_ports = {
                let (id1, &(ref url1, ref ports1)) = self.instances.iter()
                    .nth(0).unwrap()
                ;
                let (id2, &(ref url2, ref ports2)) = self.instances.iter()
                    .nth(1).unwrap()
                ;

                self.effector.get()?.send(
                    *self.client1.get()?,
                    Message::ProvideInstance(*id1, url1.clone())
                );
                self.effector.get()?.send(
                    *self.client2.get()?,
                    Message::ProvideInstance(*id2, url2.clone())
                );

                let mut game_ports = self.ports[0].clone();

                game_ports.client_ports = vec![ *ports1, *ports2 ];

                game_ports
            };

            self.provided = true;
            self.ready = (false, false);
            self.game_settings = None;
            self.game_ports = Some(game_ports);
            self.player_setup = (None, None);

            Ok(self)
        }
        else {
            // everything is taken care of
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
            self.suite = match self.suite {
                Some(MeleeSuite::OneAndDone(game)) => {
                    self.game_settings = Some(game);

                    // set melee suite to none afterwards
                    None
                },

                None => bail!("expected melee suite to contain data")
            };
            assert!(self.game_settings.is_some());

            self.request_player_setup()
        }
        else {
            Ok(self)
        }
    }

    fn request_player_setup(self) -> Result<Self> {
        let settings = self.game_settings.as_ref().unwrap().clone();

        self.effector.get()?.send(
            *self.agent1.get()?, Message::RequestPlayerSetup(settings.clone())
        );
        self.effector.get()?.send(
            *self.agent2.get()?, Message::RequestPlayerSetup(settings)
        );

        Ok(self)
    }

    fn on_player_setup(mut self, src: Handle, setup: PlayerSetup)
        -> Result<Self>
    {
        if src == *self.agent1.get()? {
            self.player_setup.0 = Some(setup);
        }
        else if src == *self.agent2.get()? {
            self.player_setup.1 = Some(setup);
        }
        else {
            bail!("invalid source for player setup")
        }

        match self.player_setup {
            (Some(setup1), Some(setup2)) => {
                let settings = self.game_settings.clone().unwrap();

                self.create_game(settings, (setup1, setup2))
            },

            _ => Ok(self)
        }
    }

    fn create_game(
        self, game: GameSettings, players: (PlayerSetup, PlayerSetup)
    )
        -> Result<Self>
    {
        self.effector.get()?.send(
            *self.agent1.get()?,
            Message::CreateGame(game.clone(), vec![ players.0, players.1 ])
        );

        Ok(self)
    }

    fn on_game_created(self, src: Handle)
        -> Result<Self>
    {
        assert_eq!(src, *self.agent1.get()?);

        let setup1 = self.player_setup.0.clone().unwrap();
        let setup2 = self.player_setup.1.clone().unwrap();
        let ports1 = self.game_ports.clone().unwrap();
        let ports2 = ports1.clone();

        self.effector.get()?.send(
            *self.agent1.get()?, Message::GameReady(setup1, ports1)
        );
        self.effector.get()?.send(
            *self.agent2.get()?, Message::GameReady(setup2, ports2)
        );

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
            Protocol::Message(src, Message::PortsPool(ports)) => {
                self.on_ports_pool(src, ports)
            },

            Protocol::Message(src, Message::Ready) => {
                self.on_agent_ready(src)
            },
            Protocol::Message(src, Message::PlayerSetup(setup)) => {
                self.on_player_setup(src, setup)
            },
            Protocol::Message(src, Message::GameCreated) => {
                self.on_game_created(src)
            },

            _ => Ok(self),
        }.chain_err(
            || cortical::ErrorKind::LobeError
        )
    }
}
