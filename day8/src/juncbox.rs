use std::{collections::HashSet, rc::Rc};

use crate::coords::Coordinates;

pub struct JunctionBox {
    pos: Coordinates,
    circuit: Rc<HashSet<Coordinates>>
}

impl JunctionBox {
    pub fn new(pos: Coordinates, circuit: Rc<HashSet<Coordinates>>) -> Self {
        Self { pos, circuit }
    }
    
    pub fn dist_to(&self, other: &JunctionBox) -> f64 {
        self.pos.dist_to(&other.pos)
    }

    pub fn set_in_curcuit(&mut self, other: &JunctionBox) {
        let circ = other.circuit.clone();
        self.circuit = circ;
    }
}

