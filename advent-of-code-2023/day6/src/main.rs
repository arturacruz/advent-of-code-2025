use advent_of_code_lib::*;

fn main() {
    let input = get_input(InputType::Input);
    
    let (times, distances) = input.split_once('\n').unwrap();
    let t = times.get(5..).unwrap()
        .split_whitespace().collect::<String>()
        .parse::<u128>().unwrap();

    let r = distances.get(9..).unwrap()
        .split_whitespace().collect::<String>()
        .parse::<u128>().unwrap();

    let prod = {
        let delta = (t.pow(2) - 4 * r) as f64;
        if delta <= 0.0 { 
            0
        } else {
            let delta = delta.sqrt();
            let l = (t as f64 - delta) / 2.0;
            let r = (t as f64 + delta) / 2.0;

            (r.ceil() - l.floor() - 1.0) as u128
        }
    };

    println!("{prod:?}");
}
