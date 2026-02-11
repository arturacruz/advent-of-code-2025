use crate::position::Position;

struct Cell {
    part: Part,
    value: u128
}

impl Cell {
    fn new(part: Part) -> Self {
        Self { part, value: 0 }
    }
}

#[derive(PartialEq, Eq)]
enum Part {
    Empty,
    Splitter,
    Beam,
    Starter
}

pub struct Map {
    matrix: Vec<Vec<Cell>>,
    starting_pos: Position
}

impl Map {
    pub fn new(text: String) -> Self {
        let lines: Vec<_> = text.lines().collect();
        
        let mut matrix = vec![];
        let mut starting_pos = None;
        for (y, line) in lines.iter().enumerate() {
            let mut matrix_line = vec![];
            for (x, char) in line.chars().enumerate() {
                let value = match char {
                    '.' => Part::Empty,
                    '^' => Part::Splitter,
                    'S' => {
                        starting_pos = Some(Position::new((x as i32, y as i32)));
                        Part::Starter
                    },
                    o => panic!("Invalid character: {o}"),
                };
                matrix_line.push(Cell::new(value));
            }
            matrix.push(matrix_line);
        }
        let starting_pos = starting_pos.expect("No starting position in map (S)");
        Self { matrix, starting_pos }
    }

    fn update(&mut self, pos: &Position, value: u128) {
        let (x, y) = pos.get();
        let (x, y) = (x as usize, y as usize);
        self.matrix[y][x].value = value;
        self.matrix[y][x].part = Part::Beam;
    }

    fn is_in_bounds(&self, pos: &Position) -> bool {
        let (x, y) = pos.get();
        
        x >= 0 && x < self.matrix[0].len() as i32 && y >= 0 && y < self.matrix.len() as i32
    }

    fn get(&self, pos: &Position) -> Option<&Cell> {
        if !self.is_in_bounds(pos) {
            return None;
        }
        let (x, y) = pos.get();
        Some(&self.matrix[y as usize][x as usize])
    }

    pub fn search(&mut self) -> u128 {
        self.search_aux(self.starting_pos.clone())
    }

    fn search_aux(&mut self, pos: Position) -> u128 {
        let val = match self.get(&pos) {
            None => return 1,
            Some(b) => b,
        };

        match val.part {
            Part::Empty => {
                let res = self.search_aux(pos.clone() + Position::DOWN);
                self.update(&pos, res);
                res
            },
            Part::Splitter => {
                let left = self.search_aux(pos.clone() + Position::LEFT);
                let right = self.search_aux(pos + Position::RIGHT);
                left + right
            },
            Part::Starter => self.search_aux(pos + Position::DOWN),
            Part::Beam => self.get(&pos).expect("Expected a value in position").value,
        }
    }
}
