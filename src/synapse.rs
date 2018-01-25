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
