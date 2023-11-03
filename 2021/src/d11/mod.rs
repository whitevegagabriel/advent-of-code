use crate::{d11::DumboState::*, utils::get_square_neighbors};
use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(problem: &[&str]) -> (u64, u64) {
    let num_rows = problem.len();
    let num_cols = problem[0].len();
    let dumbos: HashMap<_, _> = (0..num_rows)
        .cartesian_product(0..num_cols)
        .map(|pos| {
            let charge = problem[pos.0]
                .chars()
                .nth(pos.1)
                .unwrap()
                .to_digit(10)
                .unwrap() as u8;
            (pos, Charging(charge))
        })
        .collect();
    (solve1(dumbos.clone()), solve2(dumbos))
}

fn solve1(mut dumbos: HashMap<(usize, usize), DumboState>) -> u64 {
    let mut total_flashes = 0;
    for _ in 0..100 {
        flash_and_reset_all_applicable_dumbos(&mut dumbos);
        total_flashes +=
            u64::try_from(dumbos.values().filter(|d| d == &&Charging(0)).count()).unwrap();
    }
    total_flashes
}

fn solve2(mut dumbos: HashMap<(usize, usize), DumboState>) -> u64 {
    let mut i = 0;
    while !dumbos.values().all(|d| d == &Charging(0)) {
        flash_and_reset_all_applicable_dumbos(&mut dumbos);
        i += 1;
    }
    i
}

fn flash_and_reset_all_applicable_dumbos(dumbos: &mut HashMap<(usize, usize), DumboState>) {
    for (_, dumbo) in dumbos.iter_mut() {
        maybe_increment_and_make_ready(dumbo)
    }

    while dumbos.values().contains(&Ready) {
        let flashed_points = dumbos
            .iter_mut()
            .filter_map(|(p, dumbo)| {
                if let Ready = dumbo {
                    *dumbo = Flashed;
                    Some(*p)
                } else {
                    None
                }
            })
            .collect_vec();

        for flashed in flashed_points {
            let neighbor_points = get_square_neighbors(&flashed);
            for neighbor in neighbor_points {
                if let Some(dumbo) = dumbos.get_mut(&neighbor) {
                    maybe_increment_and_make_ready(dumbo);
                }
            }
        }
    }

    for (_, dumbo) in dumbos.iter_mut() {
        if let Flashed = dumbo {
            *dumbo = Charging(0);
        }
    }
}

fn maybe_increment_and_make_ready(dumbo: &mut DumboState) {
    if let Charging(i) = dumbo {
        *i += 1;
        if i > &mut 9 {
            *dumbo = Ready;
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum DumboState {
    Charging(u8),
    Ready,
    Flashed,
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
