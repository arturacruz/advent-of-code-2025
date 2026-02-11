use advent_of_code_lib::*;

fn main() {
    let input = get_input(InputType::Input);

    let (mut column1, mut column2) = (vec![], vec![]);

    for line in input.lines() {
        let (n1, n2) = line.split_once("   ").expect("Failed to split string.");
        let (n1, n2) = (parse_number(n1), parse_number(n2));
        column1.push(n1);
        column2.push(n2);
    }

    column1.sort();
    column2.sort();

    let pairs = column1.iter().zip(column2);
    let mut diffs = vec![];

    pairs.for_each(|(n1, n2)| diffs.push((n1 - n2).abs()));

    let sum: i64 = diffs.iter().sum();
    println!("{sum}");
}
