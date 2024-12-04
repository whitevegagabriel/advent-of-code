use crate::common::test;
use itertools::Itertools;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 18);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 2549);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 9);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 2003);
}

fn p1(input: &str) -> usize {
    let grid = parse_input(input);
    let height = grid.len();
    let width = grid[0].len();

    (0..height as isize)
        .cartesian_product(0..width as isize)
        .cartesian_product([
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ])
        .filter(|(start, direction)| found_target(start, direction, &['X', 'M', 'A', 'S'], &grid))
        .count()
}

fn p2(input: &str) -> usize {
    let grid = parse_input(input);
    let height = grid.len();
    let width = grid[0].len();

    (1..height as isize - 1)
        .cartesian_product(1..width as isize - 1)
        .filter(|middle| {
            if grid[middle.0 as usize][middle.1 as usize] != 'A' {
                return false;
            }

            let qty_mas = [(-1, -1), (-1, 1), (1, -1), (1, 1)]
                .iter()
                .map(|direction| {
                    (
                        (middle.0 + direction.0, middle.1 + direction.1),
                        (-direction.0, -direction.1),
                    )
                })
                .filter(|(start, direction)| {
                    found_target(start, direction, &['M', 'A', 'S'], &grid)
                })
                .count();

            qty_mas == 2
        })
        .count()
}

fn found_target(
    start: &(isize, isize),
    direction: &(isize, isize),
    target: &[char],
    grid: &[Vec<char>],
) -> bool {
    let height = grid.len();
    let width = grid[0].len();
    let mut target_idx = 0;
    let mut grid_pos = *start;
    
    while target_idx < target.len() {
        if grid_pos.0 < 0 {
            return false;
        }

        if grid_pos.0 >= width as isize {
            return false;
        }

        if grid_pos.1 < 0 {
            return false;
        }

        if grid_pos.1 >= height as isize {
            return false;
        }

        if grid[grid_pos.0 as usize][grid_pos.1 as usize] != target[target_idx] {
            return false;
        }

        target_idx += 1;
        grid_pos = (grid_pos.0 + direction.0, grid_pos.1 + direction.1);
    }

    true
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}
