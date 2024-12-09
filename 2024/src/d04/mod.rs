use crate::common::{test, Point2, Vector2};
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
        .map(|(y, x)| Point2 { x, y })
        .cartesian_product([
            Vector2 { x: -1, y: -1 },
            Vector2 { x: -1, y: 0 },
            Vector2 { x: -1, y: 1 },
            Vector2 { x: 0, y: -1 },
            Vector2 { x: 0, y: 1 },
            Vector2 { x: 1, y: -1 },
            Vector2 { x: 1, y: 0 },
            Vector2 { x: 1, y: 1 },
        ])
        .filter(|(start, direction)| found_target(*start, *direction, &['X', 'M', 'A', 'S'], &grid))
        .count()
}

fn p2(input: &str) -> usize {
    let grid = parse_input(input);
    let height = grid.len();
    let width = grid[0].len();

    (1..height as isize - 1)
        .cartesian_product(1..width as isize - 1)
        .map(|(y, x)| Point2 { x, y })
        .filter(|middle| {
            if grid[middle.y as usize][middle.x as usize] != 'A' {
                return false;
            }

            let qty_mas = [
                Vector2 { x: -1, y: -1 },
                Vector2 { x: -1, y: 1 },
                Vector2 { x: 1, y: -1 },
                Vector2 { x: 1, y: 1 },
            ]
            .into_iter()
            .map(|direction| (*middle + direction, -direction))
            .filter(|(start, direction)| found_target(*start, *direction, &['M', 'A', 'S'], &grid))
            .count();

            qty_mas == 2
        })
        .count()
}

fn found_target(
    start: Point2<isize>,
    direction: Vector2<isize>,
    target: &[char],
    grid: &[Vec<char>],
) -> bool {
    let height = grid.len();
    let width = grid[0].len();
    let mut target_idx = 0;
    let mut grid_pos = start;

    while target_idx < target.len() {
        if grid_pos.x < 0 {
            return false;
        }

        if grid_pos.x >= width as isize {
            return false;
        }

        if grid_pos.y < 0 {
            return false;
        }

        if grid_pos.y >= height as isize {
            return false;
        }

        if grid[grid_pos.y as usize][grid_pos.x as usize] != target[target_idx] {
            return false;
        }

        target_idx += 1;
        grid_pos += direction;
    }

    true
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}
