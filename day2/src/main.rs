use std::fs;

fn has_pattern(strnum: String, size: usize) -> bool {
    for range in 1..size {
        if !size.is_multiple_of(range) {
            continue
        }

        let mut slices = vec![];

        for i in 0..(size / range) {
            slices.push(
                strnum[i * range..(i+1) * range].to_string()
            );
        }

        let mut equal = true;

        for i in 0..slices.len() - 1 {
            if slices[i] != slices[i + 1] {
                equal = false;
                break;
            }
        }

        if equal {
            return true;
        }
    }

    false
}

fn main() {
    let text = fs::read_to_string("assets/input.txt").expect("Failed to open input file");
    let text = text.trim();
    let ranges = text.split(',');
    let mut equals = vec![];

    for range in ranges {
        let (min, max) = range.split_once("-").expect("No - found in range");
        let min = min.parse::<u64>().expect("Min value not a number");
        let max = max.parse::<u64>().expect("Max value not a number");

        for num in min..=max {
            let strnum = num.to_string();
            let size = strnum.len();
            if has_pattern(strnum, size) {
                equals.push(num);
            }
        }
    }

    println!("Found {} invalid IDs!", equals.len());
    println!("The sum of all IDs is {}.", equals.iter().sum::<u64>());
}
