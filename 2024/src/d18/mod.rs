use crate::common::{get_cross_neighbors, test_with_params, Point2};
use itertools::Itertools;
use pathfinding::prelude::astar;
use std::collections::HashSet;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test_with_params("example", MODULE, p1, (6, 6, 12), 22);
}

#[test]
fn p1_input() {
    test_with_params("input", MODULE, p1, (70, 70, 1024), 302);
}

#[test]
fn p2_example() {
    test_with_params("example", MODULE, p2, (6, 6), String::from("6,1"));
}

#[test]
fn p2_input() {
    test_with_params("input", MODULE, p2, (70, 70), String::from("24,32"));
}

fn p1(input: &str, params: (isize, isize, usize)) -> usize {
    let (max_x, max_y, num_steps) = params;
    let corrupted_tiles: HashSet<_> = parse_input(input).into_iter().take(num_steps).collect();

    length_of_shortest_path(max_x, max_y, &corrupted_tiles).unwrap()
}

fn p2(input: &str, params: (isize, isize)) -> String {
    let (max_x, max_y) = params;
    let all_corrupted_tiles = parse_input(input);

    (0..all_corrupted_tiles.len())
        .rev()
        .find_map(|to_take| {
            let corrupted_tiles: HashSet<_> =
                all_corrupted_tiles[..to_take].iter().cloned().collect();
            let maybe_len = length_of_shortest_path(max_x, max_y, &corrupted_tiles);

            if maybe_len.is_some() {
                let fatal_point = all_corrupted_tiles[to_take];
                Some(format!("{},{}", fatal_point.x, fatal_point.y))
            } else {
                None
            }
        })
        .unwrap()
}

fn length_of_shortest_path(
    max_x: isize,
    max_y: isize,
    corrupted_tiles: &HashSet<Point2<isize>>,
) -> Option<usize> {
    let x_range = 0..=max_x;
    let y_range = 0..=max_y;
    let found_path = astar(
        &Point2 { x: 0_isize, y: 0 },
        |curr| {
            get_cross_neighbors(*curr)
                .into_iter()
                .filter_map(|neighbor| {
                    if x_range.contains(&neighbor.x)
                        && y_range.contains(&neighbor.y)
                        && !corrupted_tiles.contains(&neighbor)
                    {
                        Some((neighbor, 1))
                    } else {
                        None
                    }
                })
                .collect_vec()
        },
        |curr| curr.x.abs_diff(max_x) + curr.y.abs_diff(max_y),
        |curr| curr.x == max_x && curr.y == max_y,
    );

    found_path.map(|(_, len)| len)
}

fn parse_input(input: &str) -> Vec<Point2<isize>> {
    input
        .lines()
        .map(|line| {
            let (left_str, right_str) = line.split(',').collect_tuple().unwrap();
            Point2 {
                x: left_str.parse::<isize>().unwrap(),
                y: right_str.parse::<isize>().unwrap(),
            }
        })
        .collect()
}
