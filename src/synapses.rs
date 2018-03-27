use organelle::{self, probe};

use agent::{self, AgentDendrite, AgentTerminal};
use melee::{self, MeleeDendrite, MeleeTerminal};

/// the synapses that can be formed between somas
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Synapse {
    /// probe
    Probe,
    /// coordinate versus games between agents
    Melee,
    /// agent
    Agent,
}

/// senders for synapses
#[derive(Debug)]
pub enum Terminal {
    Probe(probe::Terminal),
    /// melee sender
    Melee(MeleeTerminal),
    /// agent sender
    Agent(AgentTerminal),
}

/// receivers for synapses
#[derive(Debug)]
pub enum Dendrite {
    Probe(probe::Dendrite),
    /// melee receiver
    Melee(MeleeDendrite),
    /// agent receiver
    Agent(AgentDendrite),
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
            Synapse::Melee => {
                let (tx, rx) = melee::synapse();

                (Terminal::Melee(tx), Dendrite::Melee(rx))
            },
            Synapse::Agent => {
                let (tx, rx) = agent::synapse();

                (Terminal::Agent(tx), Dendrite::Agent(rx))
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
}

/// interfaces for internal somas
#[derive(Debug)]
pub enum PlayerTerminal {
    /// agent soma's interface for players
    Agent(AgentTerminal),
}

/// receivers for the terminals/interfaces of somas
#[derive(Debug)]
pub enum PlayerDendrite {
    /// exposes contract for player somas
    Agent(AgentDendrite),
}

impl organelle::Synapse for PlayerSynapse {
    type Terminal = PlayerTerminal;
    type Dendrite = PlayerDendrite;

    fn synapse(self) -> (Self::Terminal, Self::Dendrite) {
        match self {
            PlayerSynapse::Agent => {
                let (tx, rx) = agent::synapse();

                (
                    PlayerTerminal::Agent(tx),
                    PlayerDendrite::Agent(rx),
                )
            },
        }
    }
}

impl From<PlayerSynapse> for Synapse {
    fn from(synapse: PlayerSynapse) -> Self {
        match synapse {
            PlayerSynapse::Agent => Synapse::Agent,
        }
    }
}
impl From<Synapse> for PlayerSynapse {
    fn from(synapse: Synapse) -> Self {
        match synapse {
            Synapse::Agent => PlayerSynapse::Agent,
            _ => panic!(
                "invalid conversion from internal sc2 synapse {:?}",
                synapse
            ),
        }
    }
}

impl From<PlayerTerminal> for Terminal {
    fn from(terminal: PlayerTerminal) -> Self {
        match terminal {
            PlayerTerminal::Agent(tx) => Terminal::Agent(tx),
        }
    }
}
impl From<Terminal> for PlayerTerminal {
    fn from(terminal: Terminal) -> Self {
        match terminal {
            Terminal::Agent(tx) => PlayerTerminal::Agent(tx),
            _ => panic!("invalid conversion from internal sc2 terminal"),
        }
    }
}

impl From<PlayerDendrite> for Dendrite {
    fn from(dendrite: PlayerDendrite) -> Self {
        match dendrite {
            PlayerDendrite::Agent(rx) => Dendrite::Agent(rx),
        }
    }
}
impl From<Dendrite> for PlayerDendrite {
    fn from(dendrite: Dendrite) -> Self {
        match dendrite {
            Dendrite::Agent(rx) => PlayerDendrite::Agent(rx),
            _ => panic!("invalid conversion from internal sc2 dendrite"),
        }
    }
}
