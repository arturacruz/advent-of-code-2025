use std::fs;

fn main() {
    let text = fs::read_to_string("assets/input.txt").expect("Failed to open file.");

    const DIRECTIONS: [(i32, i32); 8] = [
        (-1, -1), (-1, 1), (-1, 0),
        (1, -1), (1, 1), (1, 0),
        (0, 1), (0, -1)
    ];
    
    let mut matrix = vec![];

    for line in text.lines() {
        let mut matrix_line = vec![];
        for c in line.chars() {
            let val = match c {
                '@' => true,
                '.' => false,
                o => panic!("Invalid character {o}")
            };
            matrix_line.push(val);
        }
        matrix.push(matrix_line);
    }

    let mut rolls = 0;
    let mut changed = true;

    while changed {
        changed = false;
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                let paper = matrix[y][x];
                if !paper {
                    continue
                }
                let mut adjacent = 0;
                for (i, j) in DIRECTIONS {
                    let (i, j) = (y as i32 + i, x as i32 + j);
                    if i < 0 || i as usize >= matrix.len() || j < 0 || j as usize >= matrix[y].len() {
                        continue
                    }
                    let (i, j) = (i as usize, j as usize);
                    if matrix[i][j] {
                        adjacent += 1;
                    } 
                    if adjacent == 4 {
                        break;
                    }
                }

                if adjacent < 4 {
                    matrix[y][x] = false;
                    changed = true;
                    rolls += 1;
                }
            }
        }
    }

    println!("The forklifts can access {rolls} rolls");
}
