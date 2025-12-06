use std::fs;

#[derive(Debug)]
enum Operation {
    Add, Mult
}

fn main() {
    let text = fs::read_to_string("assets/input.txt").expect("Failed to open file.");
    let lines: Vec<_> = text.lines().collect();
    let last_line = lines.last().expect("Expected a last line.");

    let mut problems = vec![];
    let mut operations = vec![];

    for op in last_line.split_whitespace() {
        let o = match op {
            "+" => Operation::Add,
            "*" => Operation::Mult,
            o => panic!("Invalid operation {o}")
        };
        operations.push(o);
    }

    for line in lines.iter().take(lines.len() - 1) {
        let tokens = line.split_whitespace();
        for (i, token) in tokens.enumerate() {
            let num = token.parse::<u64>().expect("Failed to parse token");
            if i >= problems.len() {
                problems.push(num);
                continue;
            }

            match operations[i] {
                Operation::Add => problems[i] += num,
                Operation::Mult => problems[i] *= num,
            };
        }
    }

    println!("The sum of all problems is {}", problems.iter().sum::<u64>());
}
