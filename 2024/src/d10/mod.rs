use crate::common::test;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 0);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 0);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 0);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 0);
}

fn p1(_input: &str) -> usize {
    0
}

fn p2(_input: &str) -> usize {
    0
}
