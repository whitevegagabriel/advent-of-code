use crate::common::{test, MANIFEST_DIR};
use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    static ref INPUT_FILE: PathBuf = format!("{}/src/d3/input.txt", *MANIFEST_DIR).into();
    static ref EXAMPLE_FILE: PathBuf = format!("{}/src/d3/example.txt", *MANIFEST_DIR).into();
}

#[test]
fn p1_example() {
    test(&EXAMPLE_FILE, p1, 0);
}

#[test]
fn p1_input() {
    test(&INPUT_FILE, p1, 0);
}

#[test]
fn p2_example() {
    test(&EXAMPLE_FILE, p2, 0);
}

#[test]
fn p2_input() {
    test(&INPUT_FILE, p2, 0);
}

fn p1(_input: &str) -> usize {
    0
}

fn p2(_input: &str) -> usize {
    0
}
