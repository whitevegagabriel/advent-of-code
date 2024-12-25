use crate::common::test;
use itertools::Itertools;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 3);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 3671);
}

#[allow(clippy::needless_range_loop)]
fn p1(input: &str) -> usize {
    let mut locks = vec![];
    let mut keys = vec![];
    for grid in input.split("\n\n") {
        let grid = grid
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let mut counts = [0; 5];
        let height = grid.len();
        let width = grid[0].len();
        for col in 0..width {
            for row in 0..height {
                if grid[row][col] == '#' {
                    counts[col] += 1;
                }
            }
        }
        if grid[0] == ['#', '#', '#', '#', '#'] {
            locks.push(counts);
        } else {
            keys.push(counts);
        }
    }

    locks
        .iter()
        .cartesian_product(keys)
        .filter(|(lock, key)| {
            lock[0] + key[0] <= 7
                && lock[1] + key[1] <= 7
                && lock[2] + key[2] <= 7
                && lock[3] + key[3] <= 7
                && lock[4] + key[4] <= 7
        })
        .count()
}
