mod coords;
mod distance;
mod graph;

use std::fs;

use crate::{coords::Coordinates, graph::Graph};

const INPUT: InputText = InputText::Input;

#[allow(dead_code)]
enum InputText {
    Input, Test
}

fn main() {
    let path = match INPUT {
        InputText::Test => "assets/test.txt",
        InputText::Input => "assets/input.txt"
    };
    let text = fs::read_to_string(path).expect("Could not find file");

    let mut coords = Vec::with_capacity(text.len());

    for line in text.lines() {
        let coord = line.split(",").collect::<Coordinates>();
        coords.push(coord);
    }

    let mut dists = Vec::with_capacity(coords.len());
    
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let (c1, c2) = (coords[i].clone(), coords[j].clone());
            let dist = c1.dist_to(&c2);
            dists.push(dist);
        }
    }

    dists.sort_by(|s, o| f64::total_cmp(&s.value, &o.value));

    let mut graph = Graph::new();

    let amount = match INPUT {
        InputText::Test => 10,
        InputText::Input => 1000,
    };
    for dist in dists.iter().take(amount) {
        graph.add(&dist.coords.0, &dist.coords.1);
    }

    println!("{}", graph.get_biggest());
}
