use crate::common::{Point2, get_square_neighbors, parse_to_char_map, test};
use itertools::Itertools;
use std::collections::HashMap;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 13);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 1363);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 43);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 8184);
}

fn p1(input: &str) -> usize {
    let rolls = parse_to_char_map::<isize>(input);

    rolls
        .keys()
        .filter(|coord| {
            if rolls[coord] != '@' {
                return false;
            }
            let count = get_square_neighbors(**coord)
                .iter()
                .map(|neighbor| rolls.get(neighbor).unwrap_or(&'.'))
                .filter(|c| c == &&'@')
                .count();
            count < 4
        })
        .count()
}

fn p2(input: &str) -> usize {
    let mut rolls = parse_to_char_map::<isize>(input);

    let mut removed = 0;
    while let removable_rolls = removable_points(&rolls)
        && !removable_rolls.is_empty()
    {
        removed += removable_rolls.len();
        for point in removable_rolls {
            rolls.remove(&point);
        }
    }

    removed
}

fn removable_points(rolls: &HashMap<Point2<isize>, char>) -> Vec<Point2<isize>> {
    rolls
        .keys()
        .cloned()
        .filter(|coord| {
            if rolls[coord] != '@' {
                return false;
            }
            let count = get_square_neighbors(*coord)
                .iter()
                .map(|neighbor| rolls.get(neighbor).unwrap_or(&'.'))
                .filter(|c| c == &&'@')
                .count();
            count < 4
        })
        .collect_vec()
}
