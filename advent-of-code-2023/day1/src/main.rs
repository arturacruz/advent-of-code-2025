use advent_of_code_lib::*;

const NUMBERS_AS_STRINGS: [&str; 10] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

fn main() {
    let input = get_input(InputType::Input);

    let mut codes = vec![];

    for line in input.lines() {

        let mut nums = vec![];

        for (i, letter) in line.char_indices() {
            // Check for number literally
            if letter.is_numeric() {
                nums.push(letter.to_digit(10).unwrap());
                continue;
            }

            // Check for number as str
            for (num, txt) in NUMBERS_AS_STRINGS.iter().enumerate() {
                let len = txt.len();
                if let Some(line) = line.get(i..i + len) && line == *txt {
                    nums.push(num as u32);
                }
            }
        }

        let number = nums.first().unwrap() * 10 + nums.last().unwrap();
        codes.push(number);
    }

    let sum = codes.iter().sum::<u32>();
    println!("The sum of all calibration values is {sum}!")
}
