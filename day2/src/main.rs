use std::fs;

fn main() {
    let text = fs::read_to_string("assets/input.txt").expect("Failed to open input file");
    let ranges = text.split(',');
    let mut equals = vec![];

    for range in ranges {
        let (min, max) = range.split_once("-").expect("No - found in range");
        let min = min.parse::<u64>().expect("Min value not a number");
        let max = max.parse::<u64>().expect("Max value not a number");

        for num in min..=max {
            let strnum = num.to_string();
            let size = strnum.len();
            if size % 2 != 0 {
                continue
            }
            let (a, b) = strnum.split_at(size / 2);
            if a == b {
                equals.push(num);
            }
        }
    }

    println!("Found {} invalid IDs!", equals.len());
    println!("The sum of all IDs is {}.", equals.iter().sum::<u64>());
}
