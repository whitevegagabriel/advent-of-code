use crate::common::{
    test,
    PuzzleInputType::{Example, Input},
};

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test(&Example, MODULE, p1, 0);
}

#[test]
fn p1_input() {
    test(&Input, MODULE, p1, 0);
}

#[test]
fn p2_example() {
    test(&Example, MODULE, p2, 0);
}

#[test]
fn p2_input() {
    test(&Input, MODULE, p2, 0);
}

fn p1(_input: &str) -> usize {
    0
}

fn p2(_input: &str) -> usize {
    0
}
