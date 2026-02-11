mod coords;
mod grid;

use std::fs;

use crate::{coords::Coord, grid::Grid};

fn main() {
    let text = fs::read_to_string("assets/test.txt").expect("File not found");

    let mut coords = vec![];

    let mut largest_y = 0;
    let mut largest_x = 0;

    for line in text.lines() {
        let (x, y) = line.split_once(',').expect("Invalid coordinate format");
        let coord = Coord::new(x, y);
        if coord.x > largest_x {
            largest_x = coord.x;
        }
        if coord.y > largest_y {
            largest_y = coord.y;
        }
        coords.push(coord);
    }

    let mut grid = Grid::new(largest_x, largest_y);

    for coord in coords {
        grid.add(coord);
    }

    grid.get_biggest();

    println!("{:?}", grid);


    // let mut largest_area = 0;
    //
    // for i in 0..coords.len() {
    //     for j in i + 1..coords.len() {
    //         let area = coords[i].area(&coords[j]);
    //         if area > largest_area {
    //             largest_area = area;
    //         }
    //     }
    // }
    //
    // println!("The largest area is {largest_area}.");
}
