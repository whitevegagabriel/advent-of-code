use crate::d07::HandStrength::{
    FiveKind, FourKind, FullHouse, HighCard, OnePair, ThreeKind, TwoPair,
};
use itertools::Itertools;
use std::cmp::Ordering;

pub fn solve(problem: &str) -> (u64, u64) {
    let hand_bids = problem
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_whitespace().collect_tuple().unwrap();
            let cards = cards
                .chars()
                .map(|c| match c {
                    d if d.is_ascii_digit() => c.to_digit(10).unwrap() as u64,
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => panic!("unexpected card"),
                })
                .collect_vec();
            let bid = bid.parse::<u64>().unwrap();
            (cards, bid)
        })
        .collect_vec();

    (solve1(&hand_bids), solve2(&hand_bids))
}

fn solve1(card_bids: &[(Vec<u64>, u64)]) -> u64 {
    card_bids
        .iter()
        .map(|(cards, bid)| (Hand::new1(cards.clone()), bid))
        .sorted_by(|(hand1, _), (hand2, _)| hand1.cmp(hand2))
        .enumerate()
        .map(|(rank, (_, bid))| (rank as u64 + 1) * bid)
        .sum()
}

fn solve2(card_bids: &[(Vec<u64>, u64)]) -> u64 {
    card_bids
        .iter()
        .map(|(cards, bid)| {
            // convert jacks to value 1
            let cards = cards
                .iter()
                .map(|card| if card == &11 { 1 } else { *card })
                .collect_vec();
            (Hand::new2(cards), bid)
        })
        .sorted_by(|(hand1, _), (hand2, _)| hand1.cmp(hand2))
        .enumerate()
        .map(|(rank, (_, bid))| (rank as u64 + 1) * bid)
        .sum()
}

struct Hand {
    cards: Vec<u64>,
    strength: HandStrength,
}

impl Hand {
    fn new1(cards: Vec<u64>) -> Self {
        let counts = cards.iter().counts();
        let frequencies_hi_to_lo = counts.values().sorted().rev().cloned().collect_vec();

        let strength = match *frequencies_hi_to_lo.as_slice() {
            [5, ..] => FiveKind,
            [4, ..] => FourKind,
            [3, 2, ..] => FullHouse,
            [3, ..] => ThreeKind,
            [2, 2, ..] => TwoPair,
            [2, ..] => OnePair,
            _ => HighCard,
        };

        Self { cards, strength }
    }

    fn new2(cards: Vec<u64>) -> Self {
        let counts = cards.iter().filter(|c| c != &&1).counts();
        let jacks = cards.iter().filter(|c| c == &&1).count();
        let mut frequencies_hi_to_lo = counts.values().sorted().rev().cloned().collect_vec();

        // adding number of jacks to highest frequency is always the right choice, thankfully
        if let Some(val) = frequencies_hi_to_lo.get_mut(0) {
            *val += jacks;
        } else {
            frequencies_hi_to_lo.push(jacks);
        }

        let strength = match *frequencies_hi_to_lo.as_slice() {
            [5, ..] => FiveKind,
            [4, ..] => FourKind,
            [3, 2, ..] => FullHouse,
            [3, ..] => ThreeKind,
            [2, 2, ..] => TwoPair,
            [2, ..] => OnePair,
            _ => HighCard,
        };

        Self { cards, strength }
    }
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.strength.eq(&other.strength) {
            self.cards.cmp(&other.cards)
        } else {
            self.strength.cmp(&other.strength)
        }
    }
}

#[derive(PartialEq, PartialOrd, Ord, Eq)]
enum HandStrength {
    FiveKind = 6,
    FourKind = 5,
    FullHouse = 4,
    ThreeKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
