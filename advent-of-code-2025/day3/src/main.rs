use std::{char, fs};

const RADIX: u32 = 10;

fn main() {
    let text = fs::read_to_string("assets/input.txt").expect("Failed to open file.");
    let lines = text.lines();
    let mut joltages = vec![];
    
    for bank in lines {
        let mut nums = [1; 12];

        for (i, ch) in bank.chars().enumerate() {
            let num = char::to_digit(ch, RADIX).expect("Failed to parse char to digit.");
            
            for j in 0..12 {
                let comp = nums[j];
                if num > comp && i < bank.len() - 12 + 1 + j {
                    for item in nums.iter_mut().skip(j) {
                        *item = 1;
                    }
                    nums[j] = num;
                    break;
                }
            }
        }

        let whole_number_str: String = nums.iter()
            .map(|n| char::from_digit(*n, RADIX).expect("Failed to convert number to char"))
            .collect();
        let whole_number = whole_number_str.parse::<u64>().expect("Failed to parse string to number");
        joltages.push(whole_number);
    }

    let sum: u64 = joltages.iter().sum();
    println!("The sum of all joltages is {sum}!");
}
