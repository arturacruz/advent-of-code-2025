use std::fs;

fn all_equal(strnum: String, size: usize) -> bool {
    for range in 2..size / 2 {
        let mut splits = vec![];

        if size % range != 0{
            continue;
        }
        for i in 0..size / range {
            splits.push(strnum[i * range..(i+1) * range].to_string());
        }

        let mut all_equal = true;

        for i in 0..splits.len() - 1 {
            if splits[i] != splits[i + 1] {
                all_equal = false;
                break;
            }
        }

        if all_equal {
            return true;
        }
    }
    false
}

pub fn new_rules() {
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
            if size == 1 {
                continue;
            }

            if all_equal(strnum, size) {
                equals.push(num)
            }
        }
    }

    println!("{:?}", equals);
    println!("Found {} invalid IDs!", equals.len());
    println!("The sum of all IDs is {}.", equals.iter().sum::<u64>());
}
