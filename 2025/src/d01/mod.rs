use crate::common::test;
use itertools::Itertools;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 3);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 1135);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 6);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 0);
}

fn p1(input: &str) -> usize {
    let rotations = parse_input(input);

    let mut current = 50_isize;
    let mut zeroes = 0;
    for rotation in rotations {
        current = (current + rotation) % 100;
        if current == 0 {
            zeroes += 1
        }
    }

    zeroes
}

fn p2(input: &str) -> usize {
    let rotations = parse_input(input);

    let mut current = 50;
    let mut zeroes = 0;

    for rotation in rotations {
        zeroes += rotation.unsigned_abs() / 100;
        let rotation_mod = rotation % 100;
        let before = current;
        current += rotation_mod;
        if before < 100 && current > 100
            || before > 0 && current < 0
            || before != 0 && current == 0
            || before != 100 && current == 100
        {
            zeroes += 1;
        }
        current %= 100;
        if current < 0 {
            current += 100;
        }
    }

    zeroes
}

fn parse_input(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|line| {
            let direction = if &line[0..=0] == "L" { -1 } else { 1 };

            let value = line[1..].parse::<isize>().unwrap();

            value * direction
        })
        .collect_vec()
}
