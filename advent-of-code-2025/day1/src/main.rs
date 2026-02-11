use std::fs;

struct Lock {
    // I know this could be an u8 but the logic would be a bit trickier
    num: u8,
    zeroes: u32
}

impl Lock {
    pub fn new() -> Self {
        Self { num: 50, zeroes: 0 }
    }

    pub fn right(&mut self, amount: u32) {
        for _ in 0..amount {
            self.num += 1;
            if self.num == 100 {
                self.num = 0;
                self.zeroes += 1;
            }
        }
    }

    pub fn left(&mut self, amount: u32) {
        for _ in 0..amount {
            if self.num == 1 {
                self.zeroes += 1;
            }
            if self.num == 0 {
                self.num = 100;
            }
            self.num -= 1;
        }
    }
}

fn main() {
    let mut lock = Lock::new();

    let input = fs::read_to_string("assets/input.txt").expect("Failed to open file.");

    for line in input.lines() {
        let dir = &line[0..1];
        let amount = &line[1..].parse::<u32>().expect("Could not parse line. Not a valid integer.");

        match dir {
            "R" => lock.right(*amount),
            "L" => lock.left(*amount),
            _ => panic!("Invalid lock turn!"),
        }
    }

    println!("The total amount of zeroes was {}!", lock.zeroes);
}
