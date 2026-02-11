use std::collections::HashMap;

use advent_of_code_lib::*;

struct List {
    inner: HashMap<i64, u64>
}

impl List {
    pub fn new() -> Self {
        List { inner: HashMap::new() }
    }

    pub fn add(&mut self, n: i64) {
        let v = match self.inner.get(&n) {
            Some(o) => *o,
            None => 0,
        };
        self.inner.insert(n, v + 1);
    }

    pub fn occurrences(&self, other: List) -> u64 {
        let mut sum = 0;
        for (num, occurr) in self.inner.iter() {
            let other_occurr = other.inner.get(num).unwrap_or(&0);
            sum += *num as u64 * occurr * other_occurr;
        }

        sum
    }
}
fn main() {
    let input = get_input(InputType::Input);

    let (mut list1, mut list2) = (List::new(), List::new());

    for line in input.lines() {
        let (n1, n2) = line.split_once("   ").expect("Failed to split string.");
        let (n1, n2) = (parse_number(n1), parse_number(n2));
        list1.add(n1);
        list2.add(n2);
    }
    
    let occurr = list1.occurrences(list2);
    println!("{occurr}");
}
