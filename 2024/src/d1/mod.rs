use crate::common::test;
use itertools::Itertools;

const INPUT_FILE: &str = "src/d1/input.txt";
const EXAMPLE_FILE: &str = "src/d1/example.txt";

#[test]
fn p1_example() {
    test(EXAMPLE_FILE, p1, 11);
}

#[test]
fn p1_input() {
    test(INPUT_FILE, p1, 1834060);
}

#[test]
fn p2_example() {
    test(EXAMPLE_FILE, p2, 31);
}

#[test]
fn p2_input() {
    test(INPUT_FILE, p2, 21607792);
}

fn p1(input: &str) -> usize {
    let (mut left_vec, mut right_vec): (Vec<_>, Vec<_>) = parse_nums(input);

    left_vec.sort();
    right_vec.sort();

    let total_diff = left_vec
        .iter()
        .zip(&right_vec)
        .map(|(left, right)| left.abs_diff(*right))
        .sum();

    total_diff
}

fn p2(input: &str) -> usize {
    let (left_vec, right_vec): (Vec<_>, Vec<_>) = parse_nums(input);

    let right_counts = right_vec.iter().counts();

    let similarity_score = left_vec
        .iter()
        .map(|num| num * right_counts.get(num).unwrap_or(&0))
        .sum();

    similarity_score
}

fn parse_nums(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip()
}
