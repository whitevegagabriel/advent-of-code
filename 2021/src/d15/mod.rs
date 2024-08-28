use crate::utils::{manhattan_distance, parse_matrix_of_nums};
use itertools::Itertools;
use pathfinding::prelude::astar;
use std::{collections::HashMap, hash::Hash};

pub fn solve(problem: &str) -> (u64, u64) {
    let problem = &problem.lines().collect_vec();
    let risk_levels = parse_matrix_of_nums(problem)
        .iter()
        .map(|(key, value)| (*key, *value))
        .collect();
    (solve1(&risk_levels), solve2(&risk_levels))
}

fn solve1(costs_to_enter: &HashMap<(usize, usize), u64>) -> u64 {
    min_cost_astar(costs_to_enter)
}

fn solve2(costs_to_enter: &HashMap<(usize, usize), u64>) -> u64 {
    let width = costs_to_enter
        .iter()
        .map(|((_, col), _)| col)
        .max()
        .unwrap()
        + 1;
    let height = costs_to_enter
        .iter()
        .map(|((row, _), _)| row)
        .max()
        .unwrap()
        + 1;
    let costs_to_enter = (0..5)
        .cartesian_product(0..5)
        .flat_map(|pos| {
            let (meta_row, meta_col) = pos;
            let dist = meta_row + meta_col;
            costs_to_enter
                .iter()
                .map(|((old_row, old_col), cost)| {
                    let new_pos = (old_row + width * meta_row, old_col + height * meta_col);
                    let new_cost = (*cost + dist as u64 - 1) % 9 + 1;
                    (new_pos, new_cost)
                })
                .collect_vec()
        })
        .collect();
    min_cost_astar(&costs_to_enter)
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos((usize, usize));

impl Pos {
    fn manhattan_distance(&self, other: &Pos) -> u64 {
        manhattan_distance(&self.0, &other.0) as u64
    }

    fn successors(
        &self,
        distance_to: &HashMap<(usize, usize), u64>,
        neighbors: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    ) -> Vec<(Pos, u64)> {
        let &Pos(pos) = self;
        neighbors[&pos]
            .iter()
            .map(|neighbor| (Pos(*neighbor), distance_to[neighbor]))
            .collect()
    }
}

fn min_cost_astar(distance_to: &HashMap<(usize, usize), u64>) -> u64 {
    let max_row = distance_to.keys().map(|(row, _)| row).max().unwrap();
    let max_col = distance_to.keys().map(|(_, col)| col).max().unwrap();
    let neighbors: HashMap<_, _> = distance_to
        .keys()
        .map(|(row, col)| {
            let mut neighbors = vec![];
            if row > &0 {
                neighbors.push((row - 1, *col));
            }
            if row < max_row {
                neighbors.push((row + 1, *col));
            }
            if col > &0 {
                neighbors.push((*row, col - 1));
            }
            if col < max_col {
                neighbors.push((*row, col + 1));
            }
            ((*row, *col), neighbors)
        })
        .collect();

    let goal = Pos((*max_row, *max_col));
    let result = astar(
        &Pos((0, 0)),
        |p| p.successors(distance_to, &neighbors),
        |p| p.manhattan_distance(&goal),
        |p| *p == goal,
    )
    .unwrap();
    result.1
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
