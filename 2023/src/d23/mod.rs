use crate::utils::get_cross_neighbors;
use itertools::Itertools;
use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

pub fn solve(problem: &str) -> (usize, usize) {
    let input: Vec<Vec<char>> = problem.lines().map(|line| line.chars().collect()).collect();
    (solve1(&input), solve2(&input))
}

fn solve1(input: &[Vec<char>]) -> usize {
    let height = input.len();
    let width = input[0].len();
    let neighbor_map_icy: HashMap<_, _> = (0..height)
        .cartesian_product(0..width)
        .filter_map(|point| {
            let (row, col) = point;
            let neighbors = match input[row][col] {
                '#' => return None,
                '>' => vec![Neighbor {
                    pos: (row, col + 1),
                    dist: 1,
                }],
                'v' => vec![Neighbor {
                    pos: (row + 1, col),
                    dist: 1,
                }],
                '.' => get_cross_neighbors(&point)
                    .into_iter()
                    .filter(|(r, c)| {
                        if r >= &height || c >= &width {
                            return false;
                        }
                        if r < &point.0 && input[*r][*c] == 'v' {
                            return false;
                        }
                        if c < &point.1 && input[*r][*c] == '>' {
                            return false;
                        }
                        input[*r][*c] != '#'
                    })
                    .map(|p| Neighbor { pos: p, dist: 1 })
                    .collect_vec(),
                _ => panic!("impossible"),
            };
            Some((point, neighbors))
        })
        .collect();

    find_longest_path((0, 1), (height - 1, width - 2), &neighbor_map_icy)
}

fn solve2(input: &[Vec<char>]) -> usize {
    let height = input.len();
    let width = input[0].len();
    let mut neighbor_map: HashMap<_, _> = (0..height)
        .cartesian_product(0..width)
        .filter_map(|point| {
            let (row, col) = point;
            let neighbors = match input[row][col] {
                '#' => return None,
                '>' | 'v' | '.' => get_cross_neighbors(&point)
                    .into_iter()
                    .filter(|(r, c)| {
                        if r >= &height || c >= &width {
                            return false;
                        }
                        input[*r][*c] != '#'
                    })
                    .map(|p| Neighbor { pos: p, dist: 1 })
                    .collect_vec(),
                _ => panic!("impossible"),
            };
            Some((point, neighbors))
        })
        .collect();

    while let Some((point, neighbors)) = neighbor_map.iter().find_map(|(p, neighbors)| {
        if neighbors.len() == 2 {
            Some((*p, neighbors.clone()))
        } else {
            None
        }
    }) {
        neighbor_map.remove(&point);
        let (p0, p1) = (neighbors[0].pos, neighbors[1].pos);
        let (d0, d1) = (neighbors[0].dist, neighbors[1].dist);
        let idx_curr_point0 = neighbor_map[&p0]
            .iter()
            .position(|n| n.pos == point)
            .unwrap();
        let idx_curr_point1 = neighbor_map[&p1]
            .iter()
            .position(|n| n.pos == point)
            .unwrap();
        neighbor_map.get_mut(&p0).unwrap()[idx_curr_point0] = Neighbor {
            pos: p1,
            dist: d0 + d1,
        };
        neighbor_map.get_mut(&p1).unwrap()[idx_curr_point1] = Neighbor {
            pos: p0,
            dist: d0 + d1,
        };
    }

    find_longest_path((0, 1), (height - 1, width - 2), &neighbor_map)
}

fn find_longest_path(
    start: Point,
    goal: Point,
    neighbor_map: &HashMap<Point, Vec<Neighbor>>,
) -> usize {
    let mut max_path_len = 0;

    let mut stack = vec![PathState {
        pos: start,
        prev: (0, 0),
        dist: 0,
        seen: HashSet::from([(0, 1)]),
    }];

    while let Some(state) = stack.pop() {
        if state.pos == goal {
            max_path_len = max(max_path_len, state.dist);
            continue;
        }

        let neighbors = &neighbor_map[&state.pos];

        for n in neighbors {
            if state.seen.contains(&n.pos) {
                continue;
            }
            let mut new_state = state.clone();
            new_state.prev = state.pos;
            new_state.pos = n.pos;
            new_state.dist += n.dist;
            new_state.seen.insert(n.pos);
            stack.push(new_state);
        }
    }
    max_path_len
}

type Point = (usize, usize);

#[derive(Clone)]
struct Neighbor {
    pos: Point,
    dist: usize,
}

#[derive(Clone)]
struct PathState {
    pos: Point,
    prev: Point,
    dist: usize,
    seen: HashSet<Point>,
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
