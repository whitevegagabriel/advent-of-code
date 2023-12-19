use crate::utils::{get_cross_neighbors, manhattan_distance};
use itertools::Itertools;
use pathfinding::prelude::astar;
use std::{ops::Range, thread};

pub fn solve(problem: &str) -> (usize, usize) {
    let problem = problem
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec();

    let problem1 = problem.clone();
    let problem2 = problem;

    let solution1 = thread::spawn(move || solve1(&problem1));

    let solution2 = thread::spawn(move || solve2(&problem2));

    (solution1.join().unwrap(), solution2.join().unwrap())
}

fn solve1(heat_loss_map: &[Vec<usize>]) -> usize {
    min_heat_loss(heat_loss_map, false)
}

fn solve2(heat_loss_map: &[Vec<usize>]) -> usize {
    min_heat_loss(heat_loss_map, true)
}

fn min_heat_loss(heat_loss_map: &[Vec<usize>], ultra_crucible: bool) -> usize {
    let height = heat_loss_map.len();
    let width = heat_loss_map[0].len();
    let dest = (height - 1, width - 1);
    let row_bounds = 0..height;
    let col_bounds = 0..height;
    let (_, distance) = astar(
        &Crucible {
            position: (0, 0),
            prev_position: (0, 0),
            moves_without_turning: 0,
        },
        |crucible| crucible.neighbors(&row_bounds, &col_bounds, heat_loss_map, ultra_crucible),
        |crucible| manhattan_distance(&crucible.position, &dest),
        |crucible| crucible.position == dest,
    )
    .expect("should find path to destination");

    distance
}

#[allow(dead_code)]
fn display(path: &Vec<Crucible>, mut heat_loss_map: Vec<Vec<usize>>) {
    for crucible in path {
        let (r, c) = crucible.position;
        heat_loss_map[r][c] = 0;
    }

    let s = heat_loss_map
        .iter()
        .map(|line| {
            line.iter().fold(String::new(), |mut acc, n| {
                let d = if n == &0 {
                    '.'
                } else {
                    char::from_digit(*n as u32, 10).unwrap()
                };
                acc.push(d);
                acc
            })
        })
        .join("\n");
    println!("{s}\n");
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Crucible {
    position: (usize, usize),
    prev_position: (usize, usize),
    moves_without_turning: usize,
}

impl Crucible {
    fn neighbors(
        &self,
        row_bounds: &Range<usize>,
        col_bounds: &Range<usize>,
        heat_loss_map: &[Vec<usize>],
        ultra: bool,
    ) -> Vec<(Self, usize)> {
        let (min_in_a_row, max_in_a_row) = if ultra { (3, 9) } else { (0, 2) };

        get_cross_neighbors(&self.position)
            .into_iter()
            .filter_map(|neighbor| {
                let (n_row, n_col) = neighbor;
                if !row_bounds.contains(&n_row) || !col_bounds.contains(&n_col) {
                    return None;
                }

                if neighbor == self.prev_position {
                    return None;
                }

                let in_a_row = {
                    let share_row = self.prev_position.0 == self.position.0;

                    if share_row {
                        self.prev_position.0 == neighbor.0
                    } else {
                        self.prev_position.1 == neighbor.1
                    }
                };

                // must move at least this far in a row before turning is a valid move
                if self.moves_without_turning < min_in_a_row && !in_a_row {
                    return None;
                }

                // may not move further in a row after moving the max moves in a row
                if self.moves_without_turning == max_in_a_row && in_a_row {
                    return None;
                }

                let moves_without_turning = if in_a_row {
                    self.moves_without_turning + 1
                } else {
                    0
                };

                Some((
                    Self {
                        position: neighbor,
                        prev_position: self.position,
                        moves_without_turning,
                    },
                    heat_loss_map[n_row][n_col],
                ))
            })
            .collect()
    }
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
