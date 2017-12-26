
use std::collections::HashMap;

use cortical;
use cortical::{ Lobe, Handle, Protocol, ResultExt, Constraint };
use url::Url;
use uuid::Uuid;

use super::super::{ Result };
use data::{ GameSettings, GamePorts, PortSet, PlayerSetup };
use lobes::{ Message, Role, Cortex, Soma };
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
    soma:           Soma,

    agents:         Vec<Handle>,
    clients:        Vec<Handle>,

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
    fn new(suite: MeleeSuite) -> Result<Self> {
        Ok(
            Self {
                soma: Soma::new(
                    vec![ ],
                    vec![
                        Constraint::RequireOne(Role::Launcher),

                        Constraint::Variadic(Role::Controller),
                        Constraint::Variadic(Role::InstanceProvider),
                    ]
                )?,

                agents: vec![ ],
                clients: vec![ ],

                instances: HashMap::new(),
                ports: vec![ ],

                suite: Some(suite),

                provided: false,
                ready: (false, false),
                game_settings: None,
                game_ports: None,
                player_setup: (None, None),
            }
        )
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
        let mut cortex = Cortex::new(MeleeLobe::new(settings.suite)?);

        let launcher = cortex.add_lobe(LauncherLobe::from(settings.launcher)?);

        let melee = cortex.get_main_handle();

        let player1 = cortex.add_lobe(settings.players.0);
        let player2 = cortex.add_lobe(settings.players.1);

        let agent1 = cortex.add_lobe(AgentLobe::new()?);
        let agent2 = cortex.add_lobe(AgentLobe::new()?);

        let client1 = cortex.add_lobe(ClientLobe::new()?);
        let client2 = cortex.add_lobe(ClientLobe::new()?);

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

    fn start(mut self) -> Result<Self> {
        let clients = self.soma.var_output(Role::InstanceProvider)?.clone();
        let agents = self.soma.var_output(Role::Controller)?.clone();

        assert_eq!(2, clients.len());
        assert_eq!(2, agents.len());

        if self.suite.is_none() {
            self.soma.stop()?;
        }
        else {
            self.provided = false;

            let launcher = self.soma.req_output(Role::Launcher)?;

            self.soma.send(launcher, Message::LaunchInstance)?;
            self.soma.send(launcher, Message::LaunchInstance)?;
        }

        Ok(self)
    }

    fn on_instance_pool(
        mut self, src: Handle, instances: HashMap<Uuid, (Url, PortSet)>
    )
        -> Result<Self>
    {
        assert_eq!(src, self.soma.req_output(Role::Launcher)?);

        self.instances = instances;
        self.provide_instances()
    }

    fn on_ports_pool(mut self, src: Handle, ports: Vec<GamePorts>)
        -> Result<Self>
    {
        assert_eq!(src, self.soma.req_output(Role::Launcher)?);

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

                self.soma.send(
                    self.clients[0],
                    Message::ProvideInstance(*id1, url1.clone())
                )?;
                self.soma.send(
                    self.clients[1],
                    Message::ProvideInstance(*id2, url2.clone())
                )?;

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
        if src == self.agents[0] {
            self.ready.0 = true;
        }
        else if src == self.agents[1] {
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

        self.soma.send(
            self.agents[0], Message::RequestPlayerSetup(settings.clone())
        )?;
        self.soma.send(
            self.agents[1], Message::RequestPlayerSetup(settings)
        )?;

        Ok(self)
    }

    fn on_player_setup(mut self, src: Handle, setup: PlayerSetup)
        -> Result<Self>
    {
        if src == self.agents[0] {
            self.player_setup.0 = Some(setup);
        }
        else if src == self.agents[1] {
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
        self.soma.send(
            self.agents[0],
            Message::CreateGame(game.clone(), vec![ players.0, players.1 ])
        )?;

        Ok(self)
    }

    fn on_game_created(self, src: Handle)
        -> Result<Self>
    {
        assert_eq!(src, self.agents[0]);

        let setup1 = self.player_setup.0.clone().unwrap();
        let setup2 = self.player_setup.1.clone().unwrap();
        let ports1 = self.game_ports.clone().unwrap();
        let ports2 = ports1.clone();

        self.soma.send(self.agents[0], Message::GameReady(setup1, ports1))?;
        self.soma.send(self.agents[1], Message::GameReady(setup2, ports2))?;

        Ok(self)
    }
}

impl Lobe for MeleeLobe {
    type Message = Message;
    type Role = Role;

    fn update(self, msg: Protocol<Self::Message, Self::Role>)
        -> cortical::Result<Self>
    {
        self.soma.update(&msg)?;

        match msg {
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
