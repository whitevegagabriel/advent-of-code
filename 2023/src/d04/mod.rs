use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_until, take_while1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};
use std::{cmp::min, collections::HashSet};

pub fn solve(problem: &str) -> (usize, usize) {
    let (_, scratch_cards) = separated_list1(tag("\n"), ScratchCard::parse)(problem).unwrap();
    (solve1(&scratch_cards), solve2(&scratch_cards))
}

fn solve1(scratch_cards: &[ScratchCard]) -> usize {
    scratch_cards
        .iter()
        .map(|card| {
            let num_winners = card.num_winners();
            if num_winners > 0 {
                2_usize.pow(num_winners as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

fn solve2(scratch_cards: &[ScratchCard]) -> usize {
    let mut card_qtys = vec![1; scratch_cards.len()];
    for (idx, card) in scratch_cards.iter().enumerate() {
        let num_winners = card.num_winners();
        for jdx in idx + 1..=min(idx + num_winners, scratch_cards.len()) {
            card_qtys[jdx] += card_qtys[idx];
        }
    }
    card_qtys.iter().sum()
}

#[derive(Debug, PartialEq)]
struct ScratchCard {
    winning_numbers: Vec<usize>,
    my_numbers: Vec<usize>,
}

impl ScratchCard {
    fn parse(input: &str) -> IResult<&str, Self> {
        preceded(
            tuple((take_until(":"), tag(":"))),
            map(
                separated_list1(
                    tag(" |"),
                    map(
                        many1(preceded(
                            take_while1(|c: char| c.is_whitespace()),
                            take_while1(|c: char| c.is_ascii_digit()),
                        )),
                        |nums: Vec<&str>| {
                            nums.iter()
                                .map(|s| s.parse::<usize>().unwrap())
                                .collect_vec()
                        },
                    ),
                ),
                |numbers| {
                    let (winning_numbers, my_numbers) =
                        numbers.into_iter().collect_tuple().unwrap();
                    Self {
                        winning_numbers,
                        my_numbers,
                    }
                },
            ),
        )(input)
    }

    fn num_winners(&self) -> usize {
        let winning_numbers = self.winning_numbers.iter().collect::<HashSet<_>>();
        // assume no duplicates
        let my_numbers = self.my_numbers.iter().collect::<HashSet<_>>();
        winning_numbers.intersection(&my_numbers).count()
    }
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}

#[test]
fn test_parse_scratchcard() {
    let input = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
    let (_, card) = ScratchCard::parse(input).unwrap();
    assert_eq!(
        ScratchCard {
            winning_numbers: vec![1, 21, 53, 59, 44],
            my_numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
        },
        card
    )
}
