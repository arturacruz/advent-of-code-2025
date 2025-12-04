use std::fs;

struct Lock {
    // I know this could be an u8 but the logic would be a bit trickier
    num: i32,
    zeroes: u32
}

impl Lock {
    pub fn new() -> Self {
        Self { num: 50, zeroes: 0 }
    }

    pub fn right(&mut self, amount: i32) {
        self.num += amount;
        if self.num >= 100 {
            self.num %= 100;
        }
        
        if self.num == 0 {
            self.zeroes += 1;
        }
    }

    pub fn left(&mut self, mut amount: i32) {
        amount %= 100;
        self.num -= amount;
        if self.num < 0 {
            self.num += 100;
        }

        if self.num == 0 {
            self.zeroes += 1;
        }
    }
}

fn main() {
    let mut lock = Lock::new();

    let input = fs::read_to_string("assets/input.txt").expect("Failed to open file.");

    for line in input.lines() {
        let dir = &line[0..1];
        let amount = &line[1..].parse::<i32>().expect("Could not parse line. Not a valid integer.");

        match dir {
            "R" => lock.right(*amount),
            "L" => lock.left(*amount),
            _ => panic!("Invalid lock turn!"),
        }
    }

    println!("The total amount of zeroes was {}!", lock.zeroes);
}
