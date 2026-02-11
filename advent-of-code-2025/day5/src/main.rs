use std::fs;

fn main() {
    let text = fs::read_to_string("assets/input.txt").expect("Failed to open file.");
    let lines = text.lines();
    let mut ranges = vec![];

    // Fresh ids
    for line in lines {
        if line.is_empty() {
            break;
        }
        let (min, max) = line.split_once('-').expect("No - in line.");
        let (min, max) = (
            min.parse::<u64>().expect("Failed to parse min number."),
            max.parse::<u64>().expect("Failed to parse max number."),
        );

        ranges.push((min, max));
    }

    ranges.sort();

    let mut i = 0;
    while i < ranges.len() - 1 {
        let (min, max) = ranges[i];
        let (new_min, new_max) = ranges[i + 1];

        if new_min > max {
            i += 1
        } else {
            let max = if new_max > max {
                new_max
            } else {
                max
            };
            ranges[i] = (min, max);  
            ranges.remove(i + 1);
        }
    }

    let mut fresh = 0;
    for (min, max) in ranges {
        fresh += max - min + 1;
    }

    println!("The product ranges include {} fresh IDs.", fresh);
}
