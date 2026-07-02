use advent_of_code_lib::*;

#[inline]
pub fn split_and_filter(s: &str) -> Vec<&str> {
    s.split(' ').filter(|p| !p.is_empty()).collect::<Vec<_>>()
}

fn main() {
    let input = get_input(InputType::Input);

    let cards = input
        .lines()
        .map(|line| {
            let (_, p2) = line.split_once(": ").unwrap();
            p2
        })
        .map(|line| line.split_once(" | ").unwrap())
        .map(|(p1, p2)| (
            split_and_filter(p1),
            split_and_filter(p2)
        ))
        .collect::<Vec<_>>();

    let points = cards.iter()
        .map(|(wins, owned)| 
            owned.iter().filter(|n| wins.contains(n))
        )
        .map(|nums| nums.count())
        .filter(|count| *count != 0)
        .map(|count| 2_usize.pow(count as u32 - 1))
        .sum::<usize>();

    println!("The elf's scratchcards are worth {points} points!");
}
