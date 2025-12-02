use crate::common::test;
use itertools::Itertools;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 1227775554);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 53420042388);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 4174379265);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 0);
}

fn p1(input: &str) -> usize {
    solve(input, |candidate| {
        let candidate_string = candidate.to_string();
        let halfway = candidate_string.len() / 2;
        candidate_string[0..halfway] == candidate_string[halfway..]
    })
}

fn p2(input: &str) -> usize {
    solve(input, |candidate| {
        let candidate_string = candidate.to_string();
        // try all possible valid pattern lengths. short-circuit on the first pattern to succeed
        (1..=candidate_string.len() / 2).rev().any(|chunk_size| {
            candidate_string
                .chars()
                .chunks(chunk_size)
                .into_iter()
                .map(|chunk| chunk.collect_vec())
                .all_equal()
        })
    })
}

fn solve(input: &str, candidate_filter: fn(&usize) -> bool) -> usize {
    input
        .split(',')
        .flat_map(|range_str| {
            let (start, end) = range_str
                .split('-')
                .map(|num| num.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();

            start..=end
        })
        .filter(candidate_filter)
        .sum()
}
