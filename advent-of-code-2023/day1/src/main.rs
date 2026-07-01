use advent_of_code_lib::*;

fn main() {
    let input = get_input(InputType::Input);

    let mut codes = vec![];

    for line in input.lines() {
        let nums = line
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<Vec<_>>();

        let mut number = nums.first().unwrap().to_string();
        number.push(*nums.last().unwrap());

        let number = number.parse::<u64>().unwrap();
        codes.push(number);
    }

    let sum = codes.iter().sum::<u64>();
    println!("The sum of all calibration values is {sum}!")
}
