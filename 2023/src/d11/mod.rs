use crate::utils::{manhattan_distance, transposed};
use itertools::Itertools;
use std::cmp::{max, min};

pub fn solve(problem: &str) -> (usize, usize) {
    (solve1(problem), solve2(problem))
}

fn solve1(problem: &str) -> usize {
    solve_with_expansion(problem, 2)
}

fn solve2(problem: &str) -> usize {
    solve_with_expansion(problem, 1_000_000)
}

fn solve_with_expansion(problem: &str, expansion_size: usize) -> usize {
    let image = problem
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let height = image.len();
    let width = image[0].len();

    let galaxies = (0..height)
        .cartesian_product(0..width)
        .filter(|(row, col)| image[*row][*col] == '#')
        .collect_vec();

    let expanding_rows = image
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|c| c == &'.'))
        .map(|(idx, _)| idx)
        .collect_vec();

    let expanding_cols = transposed(&image)
        .iter()
        .enumerate()
        .filter(|(_, col)| col.iter().all(|c| c == &'.'))
        .map(|(idx, _)| idx)
        .collect_vec();

    galaxies
        .iter()
        .tuple_combinations()
        .map(|(g1, g2)| {
            let (r1, c1) = g1;
            let (r2, c2) = g2;

            let r_range = min(r1, r2)..max(r1, r2);
            let c_range = min(c1, c2)..max(c1, c2);

            let mut dist = manhattan_distance(g1, g2);
            dist += expanding_rows
                .iter()
                .filter(|r| r_range.contains(r))
                .count()
                * (expansion_size - 1);
            dist += expanding_cols
                .iter()
                .filter(|c| c_range.contains(c))
                .count()
                * (expansion_size - 1);
            dist
        })
        .sum()
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
