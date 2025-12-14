mod juncbox;
mod coords;

use std::{collections::{HashMap, HashSet}, fs, rc::Rc};

use crate::{coords::Coordinates, juncbox::JunctionBox};

fn main() {
    let text = fs::read_to_string("assets/test.txt").expect("Could not find file");

    let mut boxes = vec![];

    for line in text.lines() {
        let coord = line.split(",").collect::<Coordinates>();

        let mut circuit = HashSet::new();
        circuit.insert(coord.clone());

        let juncbox = JunctionBox::new(coord, Rc::new(circuit));
        boxes.push(juncbox);
    }

}
