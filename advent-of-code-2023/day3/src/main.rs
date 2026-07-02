use std::collections::HashMap;

use advent_of_code_lib::*;

#[derive(Debug)]
struct Number {
    value: usize,
    positions: Vec<(usize, usize)>
}

impl Number {
    fn new() -> Self {
        Self { value: 0, positions: Vec::new() }
    }

    fn add_value(&mut self, value: usize, pos: (usize, usize)) {
        self.value *= 10;
        self.value += value;
        self.positions.push(pos);
    }

    fn start(&mut self) {
        self.value = 0;
    }

    fn has_started(&self) -> bool {
        !self.positions.is_empty()
    }

    fn stop(&mut self) -> (usize, Vec<(usize, usize)>) {
        let pos = self.positions.clone();
        self.positions.clear();
        (self.value, pos)
    }
}

#[derive(Debug)]
struct Symbol {
    pos: (usize, usize)
}

impl Symbol {
    fn new(pos: (usize, usize)) -> Self {
        Self { pos }
    }
}

fn store_numbers(num: &mut Number, numbers: &mut HashMap<(usize, usize), usize>) {
    if num.has_started() {
        let (n, positions) = num.stop();
        for pos in positions {
            numbers.insert(pos, n);
        }
    }
}

fn add_directions(pos: (usize, usize), dir: (i32, i32)) -> Option<(usize, usize)> {
    let x = pos.0 as i32 + dir.0;
    let y = pos.1 as i32 + dir.1;

    if x < 0 || y < 0 {
        None
    } else {
        Some((x as usize, y as usize))
    }
}

fn main() {
    let input = get_input(InputType::Input);

    let mut numbers = HashMap::new();
    let mut symbols = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut num = Number::new();
        for (x, c) in line.char_indices() {
            if c.is_numeric() {
                // If c is a number, add to current existing number, or start a new one
                let c = c.to_digit(10).unwrap();
                if !num.has_started() {
                    num.start();
                }
                num.add_value(c as usize, (x, y));
            } else {
                store_numbers(&mut num, &mut numbers);

                // If c is not a number or a ., it is a symbol
                if c != '.' {
                    symbols.push(Symbol::new((x, y)))
                }
            }
        }

        store_numbers(&mut num, &mut numbers);
    }

    let mut gear_ratios = vec![];

    'symbol: for symbol in symbols {
        let mut part_nums = vec![];
        for y in -1..=1 {
            for x in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }
                let pos = match add_directions(symbol.pos, (x, y)) {
                    None => continue,
                    Some(d) => d,
                };

                if let Some(num) = numbers.get(&pos) && !part_nums.contains(&num) {
                    if part_nums.len() >= 2 {
                        continue 'symbol;
                    }
                    part_nums.push(num);
                }
            }
        }

        if part_nums.len() == 2 {
            gear_ratios.push(part_nums[0] * part_nums[1]);
        }
    }

    let sum = gear_ratios.iter().sum::<usize>();
    println!("The sum of all gear ratios is {sum}");
}
