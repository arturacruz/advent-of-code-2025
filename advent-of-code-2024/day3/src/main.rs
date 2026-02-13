use advent_of_code_lib::*; 

#[derive(PartialEq)]
enum Stage {
    Matching,
    FirstArg,
    SecondArg
}

fn main() {
    let input = get_input(InputType::Test);

    let pattern = "mul(".as_bytes();

    let mut sum = 0;

    let mut enabled = true;

    let mut first_arg = String::new();
    let mut second_arg = String::new();
    let mut stage = Stage::Matching;
    let mut i = 0;
    for char in input.chars() {
        if stage == Stage::Matching {
            if char as u8 != pattern[i] {
                i = 0;
                continue;
            }

            i += 1;
            if char == '(' {
                stage = Stage::FirstArg;
            }
        } else if stage == Stage::FirstArg {
            if char == ',' {
                stage = Stage::SecondArg;
            } else if first_arg.len() == 3 || !char.is_numeric() {
                first_arg.clear();
                stage = Stage::Matching;
                i = 0;
            } else {
                first_arg.push(char);
            }
        } else if char == ')' {
            let (n1, n2) = (parse_number(&first_arg), parse_number(&second_arg));
            if enabled {
                sum += n1 * n2;
            }
            enabled = true;
            first_arg.clear();
            second_arg.clear();
            stage = Stage::Matching;
            i = 0;
        } else if second_arg.len() == 3 || !char.is_numeric() {
            first_arg.clear();
            second_arg.clear();
            stage = Stage::Matching;
            i = 0;
        } else {
            second_arg.push(char);
        }
    }

    println!("{sum}");
}
