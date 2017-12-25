
use cortical;
use cortical::{ Lobe, Handle, Protocol, ResultExt };
use url::Url;
use uuid::Uuid;

use super::super::{ Result, LauncherSettings };
use data::{ GameSettings };
use lobes::{ Message, Role, Effector, Cortex };
use lobes::agent::{ AgentLobe };
use lobes::launcher::{ LauncherLobe };

pub enum MeleeSuite {
    Single(GameSettings),
}

pub struct MeleeSettings<L1: Lobe + 'static, L2: Lobe + 'static> {
    pub launcher:   LauncherSettings,
    pub players:    (L1, L2),
    pub suite:      MeleeSuite,
}

pub struct MeleeLobe {
    effector:       Option<Effector>,

    launcher:       Option<Handle>,
    agent1:         Option<Handle>,
    agent2:         Option<Handle>,

    instances:      Option<((Uuid, Url), (Uuid, Url))>,

    suite:          MeleeSuite,
}

impl MeleeLobe {
    pub fn new<L1, L2>(settings: MeleeSettings<L1, L2>) -> Result<Cortex> where
        L1: Lobe,
        L2: Lobe,

        Message: From<L1::Message> + From<L2::Message>,
        Role: From<L1::Role> + From<L2::Role>,

        L1::Message: From<Message>,
        L2::Message: From<Message>,

        L1::Role: From<Role>,
        L2::Role: From<Role>,
    {
        let mut cortex = Cortex::new(
            MeleeLobe {
                effector: None,

                launcher: None,
                agent1: None,
                agent2: None,

                instances: None,

                suite: settings.suite,
            }
        );

        let launcher = cortex.add_lobe(LauncherLobe::from(settings.launcher)?);

        let melee = cortex.get_main_handle();

        let player1 = cortex.add_lobe(settings.players.0);
        let player2 = cortex.add_lobe(settings.players.1);

        let agent1 = cortex.add_lobe(AgentLobe::new());
        let agent2 = cortex.add_lobe(AgentLobe::new());

        cortex.connect(melee, launcher, Role::InstanceManager);
        cortex.connect(melee, agent1, Role::Controller);
        cortex.connect(melee, agent2, Role::Controller);

        cortex.connect(agent1, player1, Role::Agent);
        cortex.connect(agent2, player2, Role::Agent);

        Ok(cortex)
    }

    fn effector(&self) -> &Effector {
        self.effector.as_ref().unwrap()
    }

    fn init(mut self, effector: Effector) -> Self {
        self.effector = Some(effector);

        self
    }

    fn add_output(mut self, output: Handle, role: Role)
        -> Result<Self>
    {
        match role {
            Role::InstanceManager => {
                if self.launcher.is_none() {
                    self.launcher = Some(output);
                }
                else {
                    bail!("launcher already specified")
                }
            },
            Role::Controller => {
                if self.agent1.is_none() {
                    self.agent1 = Some(output);
                }
                else if self.agent2.is_none() {
                    self.agent2 = Some(output);
                }
                else {
                    bail!("both agents are already specified")
                }
            },

            _ => bail!("invalid role")
        }

        Ok(self)
    }

    fn start(self) -> Result<Self> {
        if self.launcher.is_none()
            || self.agent1.is_none()
            || self.agent2.is_none()
        {
            bail!("missing required output")
        }

        self.effector().send(self.launcher.unwrap(), Message::LaunchInstance);
        self.effector().send(self.launcher.unwrap(), Message::LaunchInstance);

        Ok(self)
    }

    fn on_instance_pool(mut self, instances: Vec<(Uuid, Url)>) -> Self {
        if instances.len() < 2 || self.instances.is_some() {
            self
        }
        else {
            self.instances = Some(
                (instances[0].clone(), instances[1].clone())
            );

            let (i1, i2) = self.instances.clone().unwrap();

            self.effector().send(
                self.agent1.unwrap(), Message::AssignInstance(i1.0, i1.1)
            );
            self.effector().send(
                self.agent2.unwrap(), Message::AssignInstance(i2.0, i2.1)
            );

            let first_game = match &self.suite {
                &MeleeSuite::Single(ref game) => {
                    game.clone()
                },
            };

            self.start_game(first_game)
        }
    }

    fn start_game(self, game: GameSettings) -> Self {
        self
    }
}

impl Lobe for MeleeLobe {
    type Message = Message;
    type Role = Role;

    fn update(self, msg: Protocol<Self::Message, Self::Role>)
        -> cortical::Result<Self>
    {
        match msg {
            Protocol::Init(effector) => Ok(self.init(effector)),
            Protocol::AddOutput(output, role) => {
                self.add_output(output, role)
                    .chain_err(|| cortical::ErrorKind::LobeError)
            },
            Protocol::Start => {
                self.start().chain_err(|| cortical::ErrorKind::LobeError)
            },

            Protocol::Message(src, Message::InstancePool(instances)) => Ok(
                self.on_instance_pool(instances)
            ),

            _ => Ok(self),
        }
    }
}
