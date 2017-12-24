
use cortical;
use cortical::{ Lobe, Protocol, Cortex, Effector, Handle };

use super::super::{ Result, LauncherSettings };
use data::{ GameSettings };
use lobes::{ Message };
use lobes::agent::{ AgentLobe };
use lobes::launcher::{ LauncherLobe };

pub enum MeleeSuite {
    Single(GameSettings),
}

pub struct MeleeSettings<L1, L2> {
    pub launcher:   LauncherSettings,

    pub player1:    L1,
    pub player2:    L2,

    pub suite:      MeleeSuite,
}

pub struct MeleeLobe {
    effector:       Option<Effector<Message>>,
    output:         Option<Handle>,
}

impl MeleeLobe {
    pub fn new<L1, L2>(settings: MeleeSettings<L1, L2>)
        -> Result<Cortex<Message>> where
        L1: Lobe + 'static,
        L2: Lobe + 'static,

        L1::Message: From<Message> + Into<Message>,
        L2::Message: From<Message> + Into<Message>,

        Message: From<L1::Message>
            + Into<L1::Message>
            + From<L2::Message>
            + Into<L2::Message>,
    {
        let mut cortex: Cortex<Message> = Cortex::new(
            MeleeLobe {
                effector: None,
                output: None,
            },
            AgentLobe::new()
        );

        let launcher = cortex.add_lobe(LauncherLobe::from(settings.launcher)?);

        let versus = cortex.get_input();
        let agent = cortex.get_output();

        let player1 = cortex.add_lobe(settings.player1);
        let player2 = cortex.add_lobe(settings.player2);

        cortex.connect(versus, launcher);
        cortex.connect(launcher, agent);
        cortex.connect(agent, player1);
        cortex.connect(agent, player2);

        Ok(cortex)
    }

    fn effector(&self) -> &Effector<Message> {
        self.effector.as_ref().unwrap()
    }

    fn init(mut self, effector: Effector<Message>) -> Self {
        self.effector = Some(effector);

        self
    }

    fn set_output(mut self, output: Handle) -> Self {
        assert!(self.output.is_none());

        self.output = Some(output);

        self
    }

    fn launch(self) -> Self {
        self.effector().send(self.output.unwrap(), Message::LaunchInstance);

        self
    }
}

impl Lobe for MeleeLobe {
    type Message = Message;

    fn update(self, msg: Protocol<Self::Message>) -> cortical::Result<Self> {
        match msg {
            Protocol::Init(effector) => Ok(self.init(effector)),
            Protocol::AddOutput(output) => Ok(self.set_output(output)),
            Protocol::Start => Ok(self.launch().launch()),

            _ => Ok(self),
        }
    }
}
