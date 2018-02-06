use organelle::{self, probe};

use action::{self, ActionDendrite, ActionTerminal};
use agent::{self, AgentDendrite, AgentTerminal};
use client::{self, ClientDendrite, ClientTerminal};
use launcher::{self, LauncherDendrite, LauncherTerminal};
use melee::{self, MeleeDendrite, MeleeTerminal};
use observer::{
    self,
    ObserverControlDendrite,
    ObserverControlTerminal,
    ObserverDendrite,
    ObserverTerminal,
};

/// the synapses that can be formed between somas
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Synapse {
    /// probe
    Probe,
    /// launch game instances
    Launcher,
    /// coordinate versus games between agents
    Melee,
    /// client to the game instance
    Client,
    /// observer controller
    ObserverControl,
    /// observer
    Observer,
    /// agent
    Agent,
    /// action
    Action,
}

/// senders for synapses
#[derive(Debug)]
pub enum Terminal {
    Probe(probe::Terminal),
    /// launcher sender
    Launcher(LauncherTerminal),
    /// melee sender
    Melee(MeleeTerminal),
    /// client sender
    Client(ClientTerminal),
    /// observer control sender
    ObserverControl(ObserverControlTerminal),
    /// observer sender
    Observer(ObserverTerminal),
    /// agent sender
    Agent(AgentTerminal),
    /// action sender
    Action(ActionTerminal),
}

/// receivers for synapses
#[derive(Debug)]
pub enum Dendrite {
    Probe(probe::Dendrite),
    /// launcher receiver
    Launcher(LauncherDendrite),
    /// melee receiver
    Melee(MeleeDendrite),
    /// client receiver
    Client(ClientDendrite),
    /// observer control receiver
    ObserverControl(ObserverControlDendrite),
    /// observer receiver
    Observer(ObserverDendrite),
    /// agent receiver
    Agent(AgentDendrite),
    /// agent receiver
    Action(ActionDendrite),
}

impl organelle::Synapse for Synapse {
    type Terminal = Terminal;
    type Dendrite = Dendrite;

    fn synapse(self) -> (Self::Terminal, Self::Dendrite) {
        match self {
            Synapse::Probe => {
                let (tx, rx) = probe::synapse();

                (Terminal::Probe(tx), Dendrite::Probe(rx))
            },
            Synapse::Launcher => {
                let (tx, rx) = launcher::synapse();

                (Terminal::Launcher(tx), Dendrite::Launcher(rx))
            },
            Synapse::Melee => {
                let (tx, rx) = melee::synapse();

                (Terminal::Melee(tx), Dendrite::Melee(rx))
            },
            Synapse::Client => {
                let (tx, rx) = client::synapse();

                (Terminal::Client(tx), Dendrite::Client(rx))
            },
            Synapse::ObserverControl => {
                let (tx, rx) = observer::control_synapse();

                (Terminal::ObserverControl(tx), Dendrite::ObserverControl(rx))
            },
            Synapse::Observer => {
                let (tx, rx) = observer::synapse();

                (Terminal::Observer(tx), Dendrite::Observer(rx))
            },
            Synapse::Agent => {
                let (tx, rx) = agent::synapse();

                (Terminal::Agent(tx), Dendrite::Agent(rx))
            },
            Synapse::Action => {
                let (tx, rx) = action::synapse();

                (Terminal::Action(tx), Dendrite::Action(rx))
            },
        }
    }
}

impl From<probe::Synapse> for Synapse {
    fn from(synapse: probe::Synapse) -> Self {
        match synapse {
            probe::Synapse::Probe => Synapse::Probe,
        }
    }
}

impl From<Synapse> for probe::Synapse {
    fn from(synapse: Synapse) -> Self {
        match synapse {
            Synapse::Probe => probe::Synapse::Probe,
            _ => panic!("invalid conversion"),
        }
    }
}

impl From<probe::Terminal> for Terminal {
    fn from(terminal: probe::Terminal) -> Self {
        Terminal::Probe(terminal)
    }
}

impl From<Terminal> for probe::Terminal {
    fn from(terminal: Terminal) -> Self {
        match terminal {
            Terminal::Probe(terminal) => terminal,
            _ => panic!("invalid conversion"),
        }
    }
}

impl From<probe::Dendrite> for Dendrite {
    fn from(dendrite: probe::Dendrite) -> Self {
        Dendrite::Probe(dendrite)
    }
}

impl From<Dendrite> for probe::Dendrite {
    fn from(dendrite: Dendrite) -> Self {
        match dendrite {
            Dendrite::Probe(dendrite) => dendrite,
            _ => panic!("invalid conversion"),
        }
    }
}

/// synapse exposing only information that is important to players
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum PlayerSynapse {
    /// control the game
    Agent,
    /// observe the game state and data
    Observer,
    /// dispatch game actions
    Action,
}

/// interfaces for internal somas
#[derive(Debug)]
pub enum PlayerTerminal {
    /// agent soma's interface for players
    Agent(AgentTerminal),
    /// interface for the observer soma
    Observer(ObserverTerminal),
    /// interface for the action soma
    Action(ActionTerminal),
}

/// receivers for the terminals/interfaces of somas
#[derive(Debug)]
pub enum PlayerDendrite {
    /// exposes contract for player somas
    Agent(AgentDendrite),
    /// internal receiver for observer interface
    Observer(ObserverDendrite),
    /// internal receiver for action interface
    Action(ActionDendrite),
}

impl organelle::Synapse for PlayerSynapse {
    type Terminal = PlayerTerminal;
    type Dendrite = PlayerDendrite;

    fn synapse(self) -> (Self::Terminal, Self::Dendrite) {
        match self {
            PlayerSynapse::Agent => {
                let (tx, rx) = agent::synapse();

                (PlayerTerminal::Agent(tx), PlayerDendrite::Agent(rx))
            },
            PlayerSynapse::Observer => {
                let (tx, rx) = observer::synapse();

                (PlayerTerminal::Observer(tx), PlayerDendrite::Observer(rx))
            },
            PlayerSynapse::Action => {
                let (tx, rx) = action::synapse();

                (PlayerTerminal::Action(tx), PlayerDendrite::Action(rx))
            },
        }
    }
}

impl From<PlayerSynapse> for Synapse {
    fn from(synapse: PlayerSynapse) -> Self {
        match synapse {
            PlayerSynapse::Agent => Synapse::Agent,
            PlayerSynapse::Observer => Synapse::Observer,
            PlayerSynapse::Action => Synapse::Action,
        }
    }
}
impl From<Synapse> for PlayerSynapse {
    fn from(synapse: Synapse) -> Self {
        match synapse {
            Synapse::Agent => PlayerSynapse::Agent,
            Synapse::Observer => PlayerSynapse::Observer,
            Synapse::Action => PlayerSynapse::Action,
            _ => panic!("invalid conversion from internal sc2 synapse"),
        }
    }
}

impl From<PlayerTerminal> for Terminal {
    fn from(terminal: PlayerTerminal) -> Self {
        match terminal {
            PlayerTerminal::Agent(tx) => Terminal::Agent(tx),
            PlayerTerminal::Observer(tx) => Terminal::Observer(tx),
            PlayerTerminal::Action(tx) => Terminal::Action(tx),
        }
    }
}
impl From<Terminal> for PlayerTerminal {
    fn from(terminal: Terminal) -> Self {
        match terminal {
            Terminal::Agent(tx) => PlayerTerminal::Agent(tx),
            Terminal::Observer(tx) => PlayerTerminal::Observer(tx),
            Terminal::Action(tx) => PlayerTerminal::Action(tx),
            _ => panic!("invalid conversion from internal sc2 terminal"),
        }
    }
}

impl From<PlayerDendrite> for Dendrite {
    fn from(dendrite: PlayerDendrite) -> Self {
        match dendrite {
            PlayerDendrite::Agent(rx) => Dendrite::Agent(rx),
            PlayerDendrite::Observer(rx) => Dendrite::Observer(rx),
            PlayerDendrite::Action(rx) => Dendrite::Action(rx),
        }
    }
}
impl From<Dendrite> for PlayerDendrite {
    fn from(dendrite: Dendrite) -> Self {
        match dendrite {
            Dendrite::Agent(rx) => PlayerDendrite::Agent(rx),
            Dendrite::Observer(rx) => PlayerDendrite::Observer(rx),
            Dendrite::Action(rx) => PlayerDendrite::Action(rx),
            _ => panic!("invalid conversion from internal sc2 dendrite"),
        }
    }
}
