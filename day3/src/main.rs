use std::{char, fs};

const RADIX: u32 = 10;

fn main() {
    let text = fs::read_to_string("assets/input.txt").expect("Failed to open file.");
    let lines = text.lines();
    let mut joltages = vec![];
    
    for bank in lines {
        let mut highest_num = 0;
        let mut second_highest_num = 0;

        for (i, ch) in bank.chars().enumerate() {
            let num = char::to_digit(ch, RADIX).expect("Failed to parse char to digit.");
            if num > highest_num && i != bank.len() - 1{
                highest_num = num;
                second_highest_num = 0;
            } else if num > second_highest_num {
                second_highest_num = num;
            }
        }

        let first_char = char::from_digit(highest_num, RADIX).expect("Failed to parse first char to digit.");
        let second_char = char::from_digit(second_highest_num, RADIX).expect("Failed to parse second char to digit");
        let whole_number_str: String = [first_char, second_char].iter().collect();

        let whole_number = whole_number_str.parse::<u32>().expect("Failed to parse string to number");
        joltages.push(whole_number);
    }

    let sum: u32 = joltages.iter().sum();
    println!("The sum of all joltages is {sum}!");
}
