use advent_of_code_lib::*;

struct Number {
    value: u32,
    is_part_number: bool,
    started: bool
}

impl Number {
    fn new() -> Self {
        Self { value: 0, is_part_number: false, started: false }
    }

    fn add_value(&mut self, value: u32) {
        self.value *= 10;
        self.value += value;
    }

    fn start(&mut self) {
        self.value = 0;
        self.is_part_number = false;
        self.started = true;
    }

    fn stop(&mut self) -> u32 {
        self.started = false;
        self.value
    }
}

fn main() {
    let input = get_input(InputType::Input);

    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();


    let mut part_nums = vec![];

    let mut num = Number::new();
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c.is_numeric() {
                // If c is a number, add to current existing number, or start a new one
                let c = c.to_digit(10).unwrap();
                if !num.started {
                    num.start();
                }
                num.add_value(c);
            } else {
                // If c is not a number, stop and add to vector the current number (if there is one)
                if num.started {
                    let n = num.stop();
                    if num.is_part_number {
                        part_nums.push(n);
                    }
                }
            }

            if !num.started || num.is_part_number {
                continue;
            }

            // Check in all directions for a symbol. If there is one, set as a part number.
            'outer: for yoff in -1..=1 {
                for xoff in -1..=1 {
                    if yoff == 0 && xoff == 0 {
                        continue;
                    }

                    let (x, y) = (x as i32 + xoff, y as i32 + yoff);
                    if y < 0 || y >= map.len() as i32 || x < 0 || x >= map[y as usize].len() as i32{
                        continue;
                    }
                    
                    let c = map[y as usize][x as usize];
                    
                    if !c.is_numeric() && c != '.' {
                        num.is_part_number = true;
                        break 'outer;
                    }
                }
            }
        }

        if num.started {
            let n = num.stop();
            if num.is_part_number {
                part_nums.push(n);
            }
        }
    }

    let sum = part_nums.iter().sum::<u32>();

    println!("The sum of all part numbers is {sum}");
}
