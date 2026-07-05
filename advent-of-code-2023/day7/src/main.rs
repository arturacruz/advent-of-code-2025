use std::collections::HashMap;

use advent_of_code_lib::*;

type Int = u64;
const BASE: Int = 15;

#[derive(PartialEq, Debug)]
enum Hands {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

fn get_cards_score_map() -> HashMap<char, Int> {
    let mut map = HashMap::new();

    let cards = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

    for (i, card) in cards.iter().enumerate() {
        map.insert(*card, (i + 1) as Int);
    }

    map
}

fn main() {
    let input = get_input(InputType::Input);
    let lines = input.lines();

    let hands = lines
        .map(|l| l.split_once(' ').unwrap())
        .map(|(h, b)| (h, b.trim().parse::<Int>().unwrap()));

    let cards_score = get_cards_score_map();
    let mut scores_and_bids = vec![];

    for (hand, bid) in hands {
        let mut score = 0;
        let mut counts = [0; 13];
        for (i, c) in hand.chars().rev().enumerate() {
            let card_score = *cards_score.get(&c).unwrap();
            score += card_score * (BASE as Int).pow(i as u32 + 1);
            counts[card_score as usize - 1] += 1;
        }

        counts.sort();
        counts.reverse();

        let mut hand_type = Hands::HighCard;

        for count in counts {
            match count {
                5 => hand_type = Hands::FiveOfAKind,
                4 => hand_type = Hands::FourOfAKind,
                3 => hand_type = Hands::ThreeOfAKind,
                2 => match hand_type {
                    Hands::ThreeOfAKind => hand_type = Hands::FullHouse,
                    Hands::OnePair => hand_type = Hands::TwoPair,
                    Hands::HighCard => hand_type = Hands::OnePair,
                    o => panic!("There should not be a {o:?} here.")
                },
                _ => break,
            }
        }

        score += hand_type as Int * (BASE as Int).pow(6);
        scores_and_bids.push((score, bid));
    }

    scores_and_bids.sort_by_key(|(s, _)| *s);

    let total_winnings = scores_and_bids.iter().enumerate()
        .map(|(i, (_, bid))| bid * (i as Int + 1))
        .sum::<Int>();

    println!("The total winnings were {total_winnings}");
}
