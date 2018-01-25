use organelle;

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
}

/// senders for synapses
#[derive(Debug)]
pub enum Terminal {
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
}

/// receivers for synapses
#[derive(Debug)]
pub enum Dendrite {
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
}

impl organelle::Synapse for Synapse {
    type Terminal = Terminal;
    type Dendrite = Dendrite;

    fn synapse(self) -> (Self::Terminal, Self::Dendrite) {
        match self {
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
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum PlayerSynapse {
    Agent,
    Observer,
}

#[derive(Debug)]
pub enum PlayerTerminal {
    Agent(AgentTerminal),
    Observer(ObserverTerminal),
}

#[derive(Debug)]
pub enum PlayerDendrite {
    Agent(AgentDendrite),
    Observer(ObserverDendrite),
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
        }
    }
}

impl From<PlayerSynapse> for Synapse {
    fn from(synapse: PlayerSynapse) -> Self {
        match synapse {
            PlayerSynapse::Agent => Synapse::Agent,
            PlayerSynapse::Observer => Synapse::Observer,
        }
    }
}
impl From<Synapse> for PlayerSynapse {
    fn from(synapse: Synapse) -> Self {
        match synapse {
            Synapse::Agent => PlayerSynapse::Agent,
            Synapse::Observer => PlayerSynapse::Observer,
            _ => panic!("invalid conversion from internal sc2 synapse"),
        }
    }
}

impl From<PlayerTerminal> for Terminal {
    fn from(terminal: PlayerTerminal) -> Self {
        match terminal {
            PlayerTerminal::Agent(tx) => Terminal::Agent(tx),
            PlayerTerminal::Observer(tx) => Terminal::Observer(tx),
        }
    }
}
impl From<Terminal> for PlayerTerminal {
    fn from(terminal: Terminal) -> Self {
        match terminal {
            Terminal::Agent(tx) => PlayerTerminal::Agent(tx),
            Terminal::Observer(tx) => PlayerTerminal::Observer(tx),
            _ => panic!("invalid conversion from internal sc2 terminal"),
        }
    }
}

impl From<PlayerDendrite> for Dendrite {
    fn from(dendrite: PlayerDendrite) -> Self {
        match dendrite {
            PlayerDendrite::Agent(rx) => Dendrite::Agent(rx),
            PlayerDendrite::Observer(rx) => Dendrite::Observer(rx),
        }
    }
}
impl From<Dendrite> for PlayerDendrite {
    fn from(dendrite: Dendrite) -> Self {
        match dendrite {
            Dendrite::Agent(rx) => PlayerDendrite::Agent(rx),
            Dendrite::Observer(rx) => PlayerDendrite::Observer(rx),
            _ => panic!("invalid conversion from internal sc2 dendrite"),
        }
    }
}
