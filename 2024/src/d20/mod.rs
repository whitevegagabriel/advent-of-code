use crate::common::{
    get_cross_neighbors, manhattan_dist, parse_to_char_map, test_with_params, Point2,
};
use itertools::Itertools;
use pathfinding::prelude::dijkstra_all;
use std::collections::HashMap;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test_with_params("example", MODULE, p1, 2, 44);
}

#[test]
fn p1_input() {
    test_with_params("input", MODULE, p1, 100, 1381);
}

#[test]
fn p2_example() {
    test_with_params("example", MODULE, p2, 50, 285);
}

#[test]
fn p2_input() {
    test_with_params("input", MODULE, p2, 100, 982124);
}

fn p1(input: &str, time_to_save: usize) -> usize {
    let grid = parse_to_char_map::<isize>(input);
    let start = *grid.iter().find(|(_, v)| v == &&'S').unwrap().0;
    let end = *grid.iter().find(|(_, v)| v == &&'E').unwrap().0;

    qty_shortcuts_saving_at_least(time_to_save, 2, &start, &end, &grid)
}

fn p2(input: &str, time_to_save: usize) -> usize {
    let grid = parse_to_char_map::<isize>(input);
    let start = *grid.iter().find(|(_, v)| v == &&'S').unwrap().0;
    let end = *grid.iter().find(|(_, v)| v == &&'E').unwrap().0;

    qty_shortcuts_saving_at_least(time_to_save, 20, &start, &end, &grid)
}

fn qty_shortcuts_saving_at_least(
    time_to_save: usize,
    shortcut_len: usize,
    start: &Point2<isize>,
    end: &Point2<isize>,
    grid: &Grid,
) -> usize {
    let mut reachable_from_start = dijkstra_all(start, |curr| get_neighbors(curr, grid));
    reachable_from_start.insert(*start, (*start, 0));

    let mut reachable_from_end = dijkstra_all(end, |curr| get_neighbors(curr, grid));
    reachable_from_end.insert(*end, (*end, 0));

    let orig_dist = reachable_from_start[end].1;

    grid.iter()
        .filter(|(_, v)| v != &&'#')
        .tuple_combinations()
        .filter_map(|((k1, _), (k2, _))| {
            let dist_12 = manhattan_dist(k1, k2) as usize;
            if dist_12 > shortcut_len {
                None
            } else {
                Some([(dist_12, k1, k2), (dist_12, k2, k1)])
            }
        })
        .flatten()
        .filter(|(dist_12, k1, k2)| {
            let dist_s12e = reachable_from_start[k1].1 + *dist_12 + reachable_from_end[k2].1;
            dist_s12e + time_to_save <= orig_dist
        })
        .count()
}

fn get_neighbors(curr: &Point2<isize>, grid: &Grid) -> Vec<(Point2<isize>, usize)> {
    get_cross_neighbors(*curr)
        .into_iter()
        .filter_map(|n| {
            if let Some(c) = grid.get(&n)
                && c != &'#'
            {
                Some((n, 1))
            } else {
                None
            }
        })
        .collect_vec()
}

type Grid = HashMap<Point2<isize>, char>;
