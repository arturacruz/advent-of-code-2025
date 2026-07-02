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

    fn power(&self) -> usize {
        self.red as usize * self.green as usize * self.blue as usize
    }
}

fn main() {
    let input = get_input(InputType::Test);

    let mut games = vec![];

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
        games.push(game.power());
    }

    let sum = games.iter().sum::<usize>();
    println!("The sum of all games' power is {sum}");
}
