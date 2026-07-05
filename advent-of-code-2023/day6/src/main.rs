use advent_of_code_lib::*;

fn main() {
    let input = get_input(InputType::Input);
    
    let (times, distances) = input.split_once('\n').unwrap();
    let times = times.get(5..).unwrap()
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap());

    let distances = distances.get(9..).unwrap()
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap());
    
    let races = times.zip(distances);

    let ways_to_win = races
        .map(|(t, r)| {
            let delta = (t.pow(2) - 4 * r) as f32;
            if delta <= 0.0 { 
                return 0; 
            }

            let delta = delta.sqrt();
            let l = (t as f32 - delta) / 2.0;
            let r = (t as f32 + delta) / 2.0;

            (r.ceil() - l.floor() - 1.0) as u32
        })
        .collect::<Vec<_>>();

    let prod = ways_to_win.iter().product::<u32>();

    println!("{prod:?}");
}
