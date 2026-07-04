use std::{collections::HashMap, ops::Range, str::Lines};

use advent_of_code_lib::*;

type Int = i64;

#[derive(Debug)]
struct RangeMap {
    ranges: Vec<(Range<Int>, Int)>,
}

impl RangeMap {
    fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    fn push(&mut self, range: Range<Int>, diff: Int) {
        self.ranges.push((range, diff));
    }

    fn get(&self, val: Int) -> Int {
        for (range, diff) in &self.ranges {
            if range.contains(&val) {
                return val + diff;
            }
        }

        val
    }
}

fn get_seeds(input: &mut Lines) -> Vec<Int> {
    let seeds = input.next().unwrap();
    let _ = input.next();
    seeds.get(7..)
        .unwrap()
        .split(' ')
        .map(|f| f.parse::<Int>().unwrap())
        .collect()
}

fn get_map(input: &mut Lines) -> RangeMap {
    let _ = input.next();
    let mut map = RangeMap::new();
    while let Some(line) = input.next() && !line.is_empty() {
        let mut line = line.split(' ');
        let start1 = line.next().unwrap().parse::<Int>().unwrap();
        let start2 = line.next().unwrap().parse::<Int>().unwrap();
        let len = line.next().unwrap().parse::<Int>().unwrap();

        map.push(start2..start2+len, start1 - start2)
    }
    map
}

fn main() {
    let input = get_input(InputType::Input);
    let mut lines = input.lines(); 

    let seeds = get_seeds(&mut lines);
    let seed_to_soil = get_map(&mut lines);
    let soil_to_fert = get_map(&mut lines);
    let fert_to_water = get_map(&mut lines);
    let water_to_light = get_map(&mut lines);
    let light_to_temp = get_map(&mut lines);
    let temp_to_humid = get_map(&mut lines);
    let humid_to_location = get_map(&mut lines);

    let locations = seeds.iter()
        .map(|s| seed_to_soil.get(*s))
        .map(|s| soil_to_fert.get(s))
        .map(|s| fert_to_water.get(s)) 
        .map(|s| water_to_light.get(s))
        .map(|s| light_to_temp.get(s))
        .map(|s| temp_to_humid.get(s))
        .map(|s| humid_to_location.get(s))
        .collect::<Vec<_>>();

    println!("seed to location: {seeds:?} -> {locations:?}");

    let min_location = locations.iter().min().unwrap();

    println!("The lowest location number is {min_location}");
}
