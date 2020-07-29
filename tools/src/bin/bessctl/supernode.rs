use std::collections::{HashSet,HashMap};

type GateIndex = u16;

pub trait TaskGroup {
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Gate {
    pub module: String,
    pub gate: GateIndex,
}

pub struct SuperNode {
    pub modules: HashSet<String>,
    pub edges: HashMap<Gate, Gate>,
    pub periphery_in: HashMap<GateIndex, Gate>,
    pub periphery_out: HashMap<GateIndex, Gate>,
}
