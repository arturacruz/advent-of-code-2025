mod coords;

use std::fs;

use crate::coords::Coord;

fn main() {
    let text = fs::read_to_string("assets/input.txt").expect("File not found");

    let mut coords = vec![];

    for line in text.lines() {
        let (x, y) = line.split_once(',').expect("Invalid coordinate format");
        coords.push(Coord::new(x, y));
    }

    let mut largest_area = 0;
    
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let area = coords[i].area(&coords[j]);
            if area > largest_area {
                largest_area = area;
            }
        }
    }

    println!("The largest area is {largest_area}.");
}
