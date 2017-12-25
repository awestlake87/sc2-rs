
use cortical;
use cortical::{ Lobe, Handle, Protocol, ResultExt };
use uuid::Uuid;

use super::super::{ Result, LauncherSettings };
use data::{ GameSettings };
use lobes::{ Message, Constraint, Effector, Cortex };
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

    suite:          MeleeSuite,
}

impl MeleeLobe {
    pub fn new<L1, L2>(settings: MeleeSettings<L1, L2>) -> Result<Cortex> where
        L1: Lobe,
        L2: Lobe,

        Message: From<L1::Message> + From<L2::Message>,
        Constraint: From<L1::Constraint> + From<L2::Constraint>,

        L1::Message: From<Message>,
        L2::Message: From<Message>,
        L1::Constraint: From<Constraint>,
        L2::Constraint: From<Constraint>,
    {
        let mut cortex = Cortex::new(
            MeleeLobe {
                effector: None,
                launcher: None,
                agent1: None,
                agent2: None,
                suite: settings.suite,
            }
        );

        let launcher = cortex.add_lobe(LauncherLobe::from(settings.launcher)?);

        let melee = cortex.get_main_handle();

        let player1 = cortex.add_lobe(settings.players.0);
        let player2 = cortex.add_lobe(settings.players.1);

        let agent1 = cortex.add_lobe(AgentLobe::new());
        let agent2 = cortex.add_lobe(AgentLobe::new());

        cortex.connect(melee, launcher, Constraint::InstanceManager);
        cortex.connect(melee, agent1, Constraint::InstanceAssignment);
        cortex.connect(melee, agent2, Constraint::InstanceAssignment);

        cortex.connect(agent1, player1, Constraint::Agent);
        cortex.connect(agent2, player2, Constraint::Agent);

        Ok(cortex)
    }

    fn effector(&self) -> &Effector {
        self.effector.as_ref().unwrap()
    }

    fn init(mut self, effector: Effector) -> Self {
        self.effector = Some(effector);

        self.launcher = None;
        self.agent1 = None;
        self.agent2 = None;

        self
    }

    fn add_output(mut self, output: Handle, constraint: Constraint)
        -> Result<Self>
    {
        match constraint {
            Constraint::InstanceManager => {
                if self.launcher.is_none() {
                    self.launcher = Some(output);
                }
                else {
                    bail!("launcher already specified")
                }
            },
            Constraint::InstanceAssignment => {
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

            _ => bail!("invalid constraint")
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

    fn on_instance_pool(self, instances: Vec<Uuid>) -> Self {
        if instances.len() < 2 {
            self
        }
        else {
            match *&self.suite {
                MeleeSuite::Single(ref game) => {

                }
            }

            self
        }
    }
}

impl Lobe for MeleeLobe {
    type Message = Message;
    type Constraint = Constraint;

    fn update(self, msg: Protocol<Self::Message, Self::Constraint>)
        -> cortical::Result<Self>
    {
        match msg {
            Protocol::Init(effector) => Ok(self.init(effector)),
            Protocol::AddOutput(output, constraint) => {
                self.add_output(output, constraint)
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
