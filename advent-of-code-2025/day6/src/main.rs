use std::{fs, iter::zip};

#[derive(Debug, Clone)]
enum Operation {
    Add, Mult
}

fn main() {
    let text = fs::read_to_string("assets/input.txt").expect("Failed to open file.");
    let lines: Vec<_> = text.lines().collect();
    let last_line = lines.last().expect("Expected a last line.");

    let mut problems = vec![];
    let mut spaces = vec![];
    let mut operations = vec![];

    let mut space = 0;

    for c in last_line.chars() {
        let op = match c {
            '+' => Operation::Add,
            '*' => Operation::Mult,
            ' ' => { space += 1; continue },
            o => panic!("Invalid operation {o}")
        };
        if space != 0 {
            spaces.push(space);
            space = 0;
        }
        operations.push(op);
    }
    spaces.push(space + 1);

    let mut offset = 0;

    for (idx, (space, op)) in zip(spaces, operations).enumerate() {
        for i in offset..offset+space {
            let mut numberstr = String::new();
            for line in lines.iter().take(lines.len() - 1) {
                numberstr.push_str(&line[i..=i]);
            }
            let numberstr = numberstr.trim();
            if numberstr.is_empty() {
                break
            }
            let num = numberstr.parse::<u64>().expect("Failed to parse number.");
            if idx == problems.len() {
                problems.push(num);
                continue;
            }

            match op {
                Operation::Add => problems[idx] += num,
                Operation::Mult => problems[idx] *= num,
            }
        }
        offset += space + 1;
    }

    println!("The sum of all problems is {}", problems.iter().sum::<u64>());
}
