use crate::utils::get_cross_neighbors;
use itertools::Itertools;
use pathfinding::prelude::astar;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve(problem: &str) -> (u64, u64) {
    let (s_coord, pipe_map) = parse_input(problem);
    (
        solve1(&s_coord, pipe_map.clone()),
        solve2(&s_coord, &pipe_map),
    )
}

fn solve1(start_coord: &CoordI64, mut pipe_map: HashMap<CoordI64, Vec<CoordI64>>) -> u64 {
    // break loop at start to make path / length finding easier
    let coords = pipe_map.get_mut(start_coord).unwrap();
    let end = coords.pop().unwrap();
    let (_, dist) = astar(
        start_coord,
        |coord| {
            pipe_map
                .get(coord)
                .expect("successors should already be in the loop")
                .iter()
                .map(|n| (*n, 1))
                .collect_vec()
        },
        |_| 0,
        |coord| coord == &end,
    )
    .expect("should find path to end of loop");
    ((dist + 1) / 2) as u64
}

fn solve2(start_coord: &CoordI64, pipe_map: &HashMap<CoordI64, Vec<CoordI64>>) -> u64 {
    // take all the pipe locations, and double their coordinate values, also adding "intermediate"
    // pipe points to simulate a boundary for the next calculation
    let pipe_coords = doubled_state_space_keys(start_coord, pipe_map);

    // this is guaranteed to be an 'F'
    let left_top = pipe_coords.iter().min().unwrap();

    // this is guaranteed to be inside the pipe loop
    let start = (left_top.0 + 1, left_top.1 + 1);

    let mut to_explore = VecDeque::from([start]);
    let mut seen = HashSet::from([start]);
    let mut num_in_loop = 0;

    // flood fill all points that are connected to the first point, as they are all inside the loop
    while let Some(coord) = to_explore.pop_front() {
        for neighbor in get_cross_neighbors(&coord) {
            if seen.contains(&neighbor) || pipe_coords.contains(&neighbor) {
                continue;
            }
            seen.insert(neighbor);
            to_explore.push_back(neighbor);
            // is part of the original state space, and thus needs to be counted
            if neighbor.0 % 2 == 0 && neighbor.1 % 2 == 0 {
                num_in_loop += 1;
            }
        }
    }
    num_in_loop
}

fn doubled_state_space_keys(
    start_coord: &CoordI64,
    pipe_map: &HashMap<CoordI64, Vec<CoordI64>>,
) -> HashSet<CoordI64> {
    let mut new_coords = HashSet::new();

    let mut prev = *start_coord;
    let mut curr = *pipe_map[&prev].first().unwrap();
    let first = curr;

    loop {
        let doubled_prev = (prev.0 * 2, prev.1 * 2);
        let doubled_curr = (curr.0 * 2, curr.1 * 2);
        let doubled_phantom = (
            (doubled_curr.0 + doubled_prev.0) / 2,
            (doubled_curr.1 + doubled_prev.1) / 2,
        );

        new_coords.insert(doubled_phantom);
        new_coords.insert(doubled_curr);

        let new_curr = *pipe_map[&curr].iter().find(|n| n != &&prev).unwrap();
        prev = curr;
        curr = new_curr;

        // this is a do-while, and I'm too lazy to simplify into a while
        if curr == first {
            break;
        }
    }

    new_coords
}

fn parse_input(input: &str) -> (CoordI64, HashMap<CoordI64, Vec<CoordI64>>) {
    let mut pipe_map = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            let row = row as i64;
            line.char_indices()
                .filter_map(|(col, c)| {
                    let col = col as i64;
                    let neighbor_dirs = match c {
                        '-' => vec![(0, -1), (0, 1)],
                        '|' => vec![(-1, 0), (1, 0)],
                        'J' => vec![(0, -1), (-1, 0)],
                        'L' => vec![(-1, 0), (0, 1)],
                        'F' => vec![(0, 1), (1, 0)],
                        '7' => vec![(1, 0), (0, -1)],
                        _ => return None,
                    };
                    let neighbors = neighbor_dirs
                        .iter()
                        .map(|d| (row + d.0, col + d.1))
                        .collect_vec();
                    Some(((row, col), neighbors))
                })
                .collect_vec()
        })
        .collect::<HashMap<_, _>>();

    let s_coord = input
        .lines()
        .enumerate()
        .find_map(|(row, line)| {
            if line.contains('S') {
                Some((row, line))
            } else {
                None
            }
        })
        .and_then(|(row, line)| {
            line.chars()
                .find_position(|c| c == &'S')
                .map(|(col, _)| (row, col))
        })
        .unwrap();
    let s_coord_i64 = (s_coord.0 as i64, s_coord.1 as i64);

    let s_neighbors = get_cross_neighbors(&s_coord)
        .iter()
        .map(|(row, col)| (*row as i64, *col as i64))
        .filter(|n| {
            if let Some(n_neighbors) = pipe_map.get(n) {
                n_neighbors.contains(&s_coord_i64)
            } else {
                false
            }
        })
        .collect_vec();

    pipe_map.insert(s_coord_i64, s_neighbors);

    (s_coord_i64, pipe_map)
}

type CoordI64 = (i64, i64);

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}

#[test]
fn test_double_state_space() {
    //   0 1
    // 0 # #
    // 1 # #
    let input = "S7\nLJ";
    let (_, pipe_map) = parse_input(input);
    let new_state_space = doubled_state_space_keys(&(0, 0), &pipe_map);

    //   0 1 2
    // 0 # # #
    // 1     #
    // 2 # # #
    assert_eq!(
        HashSet::from([
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ]),
        new_state_space
    );
}
