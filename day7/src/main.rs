mod map;
mod position;

use std::fs;
use crate::map::Map;

fn main() {
    let text = fs::read_to_string("assets/input.txt").expect("Failed to open file.");

    let mut map = Map::new(text);
    let mut count = 0;
    map.search(&mut count);
    
    println!("The beam was split {count} times.");
}
