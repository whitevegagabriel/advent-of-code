use crate::common::test;
use regex::Regex;
use std::sync::LazyLock;

const MODULE: &str = module_path!();
static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());

#[test]
fn p1_example() {
    test("example", MODULE, p1, 161);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 163931492);
}

#[test]
fn p2_example() {
    test("example2", MODULE, p2, 48);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 76911921);
}

fn p1(input: &str) -> usize {
    sum_multiplications(input)
}

fn p2(input: &str) -> usize {
    input
        .split("do()")
        .map(|between_do| {
            let valid_section = between_do.split("don't()").next().unwrap();
            sum_multiplications(valid_section)
        })
        .sum()
}

fn sum_multiplications(input: &str) -> usize {
    RE.captures_iter(input)
        .map(|cap| {
            let left = cap[1].parse::<usize>().unwrap();
            let right = cap[2].parse::<usize>().unwrap();
            left * right
        })
        .sum()
}
