use crate::common::{get_cross_neighbors, parse_to_usize_map, test, Point2};
use itertools::Itertools;
use std::collections::HashMap;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 36);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 482);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 81);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 1094);
}

fn p1(input: &str) -> usize {
    let grid = parse_to_usize_map::<isize>(input);

    grid.iter()
        .filter_map(|(k, v)| {
            if v != &0 {
                return None;
            }

            Some(
                reachable_peaks_with_repetition(*k, &grid)
                    .iter()
                    .unique()
                    .count(),
            )
        })
        .sum()
}

fn p2(input: &str) -> usize {
    let grid = parse_to_usize_map::<isize>(input);

    grid.iter()
        .filter_map(|(k, v)| {
            if v != &0 {
                return None;
            }

            Some(reachable_peaks_with_repetition(*k, &grid).len())
        })
        .sum()
}

fn reachable_peaks_with_repetition(
    curr: Point2<isize>,
    grid: &HashMap<Point2<isize>, usize>,
) -> Vec<Point2<isize>> {
    let curr_height = grid[&curr];
    get_cross_neighbors(curr)
        .iter()
        .filter_map(|neighbor| {
            let neighbor_height = grid.get(neighbor)?;

            if *neighbor_height != curr_height + 1 {
                return None;
            }

            if neighbor_height == &9 {
                Some(vec![*neighbor])
            } else {
                Some(reachable_peaks_with_repetition(*neighbor, grid))
            }
        })
        .flatten()
        .collect()
}
