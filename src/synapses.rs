use organelle::{self, probe};

/// the synapses that can be formed between somas
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Synapse {
    /// probe
    Probe,
}

/// senders for synapses
#[derive(Debug)]
pub enum Terminal {
    Probe(probe::Terminal),
}

/// receivers for synapses
#[derive(Debug)]
pub enum Dendrite {
    Probe(probe::Dendrite),
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
