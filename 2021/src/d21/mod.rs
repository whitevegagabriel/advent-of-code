use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_while},
    combinator::map,
    error::Error,
    sequence::{pair, preceded},
};
use std::collections::HashMap;

pub fn solve(problem: &str) -> (u64, u64) {
    let (_, (p1_start, p2_start)) = map(
        pair(
            preceded(
                tag::<_, _, Error<&str>>("Player 1 starting position: "),
                take_while(|c: char| c.is_ascii_digit()),
            ),
            preceded(
                tag("\nPlayer 2 starting position: "),
                take_while(|c: char| c.is_ascii_digit()),
            ),
        ),
        |(p1_start, p2_start): (&str, &str)| {
            (
                p1_start.parse::<u64>().unwrap(),
                p2_start.parse::<u64>().unwrap(),
            )
        },
    )(problem)
    .unwrap();
    // parse what the two players are starting with
    (solve1(p1_start, p2_start), solve2(p1_start, p2_start))
}

fn solve1(p1_start: u64, p2_start: u64) -> u64 {
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut p1_pos = p1_start;
    let mut p2_pos = p2_start;
    let mut prev_score = 0;
    let mut curr_player = 0;
    let mut curr_dice_roll = 1;

    let mut dice_rolls = 0;
    while prev_score < 1000 {
        let (score_to_update, pos_to_update) = if curr_player == 0 {
            (&mut p1_score, &mut p1_pos)
        } else {
            (&mut p2_score, &mut p2_pos)
        };

        let to_move = curr_dice_roll + (curr_dice_roll + 1) % 100 + (curr_dice_roll + 2) % 100;
        *pos_to_update = (*pos_to_update + to_move - 1) % 10 + 1;
        *score_to_update += *pos_to_update;

        prev_score = *score_to_update;
        curr_dice_roll = (curr_dice_roll + 3) % 100;
        curr_player = (curr_player + 1) % 2;
        dice_rolls += 3;
    }

    dice_rolls * p1_score.min(p2_score)
}

fn solve2(p1_start: u64, p2_start: u64) -> u64 {
    let roll_distribution = roll_distributions(); // roll, qty

    let mut player_1_winners = 0;
    let mut rem_player_1 = 1;
    let mut rem_player_2 = 1;

    let mut player_1 = HashMap::from([((0, p1_start), 1)]);
    let mut player_2 = HashMap::from([((0, p2_start), 1)]);
    let mut curr_player_id = 0;

    while !player_1.is_empty() && !player_2.is_empty() {
        let (player, player_counter) = if curr_player_id == 0 {
            (&mut player_1, &mut rem_player_1)
        } else {
            (&mut player_2, &mut rem_player_2)
        };
        let mut next_player = HashMap::<(u64, u64), usize>::new();

        let mut num_new_winners = 0;
        for ((score, position), player_qty) in player.iter() {
            *player_counter -= *player_qty;
            for (roll, roll_qty) in &roll_distribution {
                let new_pos = (position + roll - 1) % 10 + 1;
                let new_score = score + new_pos;
                let new_qty = *player_qty * roll_qty;

                if new_score >= 21 {
                    num_new_winners += new_qty;
                    continue;
                }

                *next_player.entry((new_score, new_pos)).or_default() += new_qty;
                *player_counter += new_qty;
            }
        }
        if curr_player_id == 0 {
            player_1_winners += num_new_winners * rem_player_2
        }

        curr_player_id = (curr_player_id + 1) % 2;
        *player = next_player;
    }
    player_1_winners as u64
}

fn roll_distributions() -> HashMap<u64, usize> {
    (1..=3)
        .cartesian_product(1..=3)
        .cartesian_product(1..=3)
        .map(|((a, b), c)| a + b + c)
        .counts()
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
