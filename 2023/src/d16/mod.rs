use crate::d16::Direction::{Down, Left, Right, Up};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;

pub fn solve(problem: &str) -> (usize, usize) {
    let problem = problem
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    (solve1(&problem), solve2(&problem))
}

fn solve1(mirrors: &[Vec<char>]) -> usize {
    num_energized(mirrors, ((0, 0), Right))
}

fn solve2(mirrors: &[Vec<char>]) -> usize {
    let last_row = mirrors.len() - 1;
    let last_col = mirrors[0].len() - 1;

    let configs = (0..=last_col)
        .map(|col| ((0, col), Down))
        .chain((0..=last_col).map(|col| ((last_row, col), Up)))
        .chain((0..=last_row).map(|row| ((row, 0), Right)))
        .chain((0..=last_row).map(|row| ((row, last_col), Left)))
        .collect_vec();

    configs
        .into_par_iter()
        .map(|config| num_energized(mirrors, config))
        .max()
        .unwrap()
}

fn num_energized(mirrors: &[Vec<char>], initial_config: ((usize, usize), Direction)) -> usize {
    let height = mirrors.len();
    let width = mirrors[0].len();
    let mut seen = HashSet::new();
    let mut beams = Vec::from([initial_config]);
    while let Some(beam) = beams.pop() {
        if seen.contains(&beam) {
            continue;
        }
        seen.insert(beam.clone());
        let (beam_coord, dir) = beam;
        let dir = match mirrors[beam_coord.0][beam_coord.1] {
            '.' => dir,
            '-' => match dir {
                Left | Right => dir,
                Up | Down => {
                    beams.push((beam_coord, Left));
                    beams.push((beam_coord, Right));
                    continue;
                }
            },
            '|' => match dir {
                Up | Down => dir,
                Left | Right => {
                    beams.push((beam_coord, Up));
                    beams.push((beam_coord, Down));
                    continue;
                }
            },
            '/' => match dir {
                Right => Up,
                Left => Down,
                Up => Right,
                Down => Left,
            },
            '\\' => match dir {
                Right => Down,
                Left => Up,
                Up => Left,
                Down => Right,
            },
            _ => unreachable!(),
        };

        let (row, col) = beam_coord;
        let beam_coord = match dir {
            Up => {
                if row == 0 {
                    continue;
                }
                (row - 1, col)
            }
            Down => {
                if row == height - 1 {
                    continue;
                }
                (row + 1, col)
            }
            Left => {
                if col == 0 {
                    continue;
                }
                (row, col - 1)
            }
            Right => {
                if col == width - 1 {
                    continue;
                }
                (row, col + 1)
            }
        };
        beams.push((beam_coord, dir));
    }

    seen.iter().map(|(coord, _)| coord).unique().count()
}

#[derive(Eq, PartialEq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
