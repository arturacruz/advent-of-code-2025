use std::fmt::Debug;

use crate::coords::Coord;

#[derive(PartialEq)]
pub enum Tile {
    Green, Red, Other
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Green => 'X',
            Self::Red => '#',
            Self::Other => '.'
        };
        write!(f, "{}", s)
    }
}

pub struct Grid {
    pub map: Vec<Vec<Tile>>,
    points: Vec<Coord>
}

impl Grid {
    pub fn new(x: usize, y: usize) -> Self {
        let mut lines = Vec::with_capacity(y);
        for _ in 0..=y {
            let mut line = Vec::with_capacity(x);
            for _ in 0..=x {
                line.push(Tile::Other);
            }
            lines.push(line);
        }

        Grid { map: lines, points: Vec::new() }
    }

    fn draw_lines(&mut self) {
        for i in 0..self.points.len() {
            let (p1, p2);
            if i == self.points.len() - 1 {
                p1 = self.points.last().unwrap();
                p2 = self.points.first().unwrap();
            } else {
                p1 = &self.points[i];
                p2 = &self.points[i + 1]
            }

            if p1.x == p2.x {
                let min = p1.y.min(p2.y);
                let max = p1.y.max(p2.y);
                for y in min + 1..max {
                    self.map[y][p1.x] = Tile::Green;
                }
            } else {
                let min = p1.x.min(p2.x);
                let max = p1.x.max(p2.x);
                for x in min + 1..max {
                    self.map[p1.y][x] = Tile::Green;
                }
            }
        }
    }

    pub fn add(&mut self, coord: Coord) {
        self.map[coord.y][coord.x] = Tile::Red;
        self.points.push(coord);
    }

    pub fn get_biggest(&mut self) {
        self.draw_lines();
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        for line in &self.map {
            for tile in line {
                let char = match tile {
                    Tile::Green => 'X',
                    Tile::Red => '#',
                    _ => '.'
                };
                res.push(char);
            }
            res.push('\n');
        }

        write!(f, "{}", res)
    }
}
