use crate::common::test;

const INPUT_FILE: &str = "src/d3/input.txt";
const EXAMPLE_FILE: &str = "src/d3/example.txt";

#[test]
fn p1_example() {
    test(EXAMPLE_FILE, p1, 0);
}

#[test]
fn p1_input() {
    test(INPUT_FILE, p1, 0);
}

#[test]
fn p2_example() {
    test(EXAMPLE_FILE, p2, 0);
}

#[test]
fn p2_input() {
    test(INPUT_FILE, p2, 0);
}

fn p1(_input: &str) -> usize {
    0
}

fn p2(_input: &str) -> usize {
    0
}
