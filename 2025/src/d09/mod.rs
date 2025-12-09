use crate::common::test;
use itertools::Itertools;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 50);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 4715966250);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 24);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 0);
}

fn p1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1))
        .max()
        .unwrap()
}

fn p2(input: &str) -> usize {
    0
}
