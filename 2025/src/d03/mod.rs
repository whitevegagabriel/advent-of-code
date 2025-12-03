use itertools::Itertools;

use crate::common::test;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 357);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 17554);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 0);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 0);
}

fn p1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (max1, max2) = line.chars().tuple_combinations::<(_, _)>().max().unwrap();
            format!("{max1}{max2}").parse::<usize>().unwrap()
        })
        .sum()
}

fn p2(input: &str) -> usize {
    0
}
