use crate::common::{
    test,
    PuzzleInputType::{Example, Input},
};
use itertools::Itertools;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test(&Example, MODULE, p1, 2);
}

#[test]
fn p1_input() {
    test(&Input, MODULE, p1, 564);
}

#[test]
fn p2_example() {
    test(&Example, MODULE, p2, 4);
}

#[test]
fn p2_input() {
    test(&Input, MODULE, p2, 604);
}

fn p1(input: &str) -> usize {
    let parsed_input = parse_input(input);

    parsed_input.iter().filter(|report| is_safe(report)).count()
}

fn p2(input: &str) -> usize {
    let parsed_input = parse_input(input);

    parsed_input
        .iter()
        .filter(|report| {
            if is_safe(report) {
                return true;
            }

            (0..report.len()).any(|i| {
                let mut report = (*report).clone();
                report.remove(i);

                is_safe(&report)
            })
        })
        .count()
}

fn is_safe(levels: &[usize]) -> bool {
    let increasing = levels[1] > levels[0];

    levels
        .iter()
        .tuple_windows()
        .all(|(a, b)| a.abs_diff(*b) <= 3 && a.abs_diff(*b) > 0 && (b > a) == increasing)
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}
