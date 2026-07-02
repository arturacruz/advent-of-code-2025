use advent_of_code_lib::*;

enum Color {
    Red, Green, Blue
}

struct Game {
    red: u32,
    green: u32,
    blue: u32
}

impl Game {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0
        }
    }

    fn set_max(&mut self, color: Color, amount: u32) {
        match color {
            Color::Red => if amount > self.red { self.red = amount },
            Color::Green => if amount > self.green { self.green = amount },
            Color::Blue => if amount > self.blue { self.blue = amount }
        }
    }

    fn is_possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

fn main() {
    let input = get_input(InputType::Input);

    let mut possible_games = vec![];

    for (id, line) in input.lines().enumerate() {
        let line = line
            .split(": ")
            .last()
            .unwrap();

        let infos = line.split("; ");

        let mut game = Game::new();
        for info in infos {
            let cubes = info.split(", ");
            for cube in cubes {
                let (amount, color) = cube.split_once(' ').unwrap();
                let amount = amount.parse::<u32>().unwrap();
                let color = match color {
                    "red" => Color::Red,
                    "green" => Color::Green,
                    "blue" => Color::Blue,
                    o => panic!("invalid color {o}")
                };
                game.set_max(color, amount);
            }
        }
        if game.is_possible() {
            possible_games.push(id + 1);
        }
    }

    println!("possible games: {possible_games:?}");
    
    let sum = possible_games.iter().sum::<usize>();
    println!("The sum of all possible games IDs is {sum}");
}
