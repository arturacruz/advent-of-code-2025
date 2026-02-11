use advent_of_code_lib::*;

struct Report {
    levels: Vec<i64>,
}

impl Report {
    pub fn new(line: &str) -> Self {
        let levels: Vec<_> = line.split(' ')
            .map(parse_number)
            .collect();

        Self { levels }
    }

    pub fn is_safe(&self) -> bool {
        let ascending = self.levels[0] < self.levels[1];
        for i in 0..self.levels.len() - 1 {
            let (lv1, lv2) = (self.levels[i], self.levels[i + 1]);
            let diff = (lv1 - lv2).abs();
            if (ascending && lv1 >= lv2) ||
                (!ascending && lv1 <= lv2) ||
                !(1..=3).contains(&diff) {
                return false;
            }
        }
        true
    }
}
fn main() {
    let input = get_input(InputType::Input);

    let mut sum = 0;
    for line in input.lines() {
        let report = Report::new(line);
        if report.is_safe() {
            sum += 1;
        }
    }

    println!("{sum}");

}
