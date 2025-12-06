use std::{collections::HashSet, fs, ops::RangeInclusive};

fn register_ids(line: &str, ranges: &mut HashSet<RangeInclusive<u64>>) {
    let (min, max) = line.split_once('-').expect("No - in line.");
    let (min, max) = (
        min.parse::<u64>().expect("Failed to parse min number."),
        max.parse::<u64>().expect("Failed to parse max number."),
    );
    
    ranges.insert(min..=max); 
}

fn is_product_fresh(line: &str, ranges: &mut HashSet<RangeInclusive<u64>>) -> bool {
    let num = line.parse::<u64>().expect("Failed to parse product ID to number.");
    for range in ranges.iter() {
        if range.contains(&num) {
            return true;
        }
    }
    false
}

fn main() {
    let text = fs::read_to_string("assets/input.txt").expect("Failed to open file.");
    let lines = text.lines();
    let mut ranges = HashSet::new();

    // Fresh ids
    let mut creating_ids = true;
    let mut fresh = 0;
    for line in lines {
        if line.is_empty() {
            creating_ids = false;
            continue;
        }
        if creating_ids {
            register_ids(line, &mut ranges);
        } else if is_product_fresh(line, &mut ranges) {
            fresh += 1;
        }
    }

    println!("The amount of fresh products is {fresh}.");
}
