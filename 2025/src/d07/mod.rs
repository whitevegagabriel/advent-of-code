use crate::common::{
    DOWN_USIZE, Point2, RIGHT_USIZE, parse_to_char_map_and_find, test,
};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 21);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 1562);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 40);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 24292631346665);
}

fn p1(input: &str) -> usize {
    let (splitters, Some(start)) =
        parse_to_char_map_and_find::<usize>(input, 'S')
    else {
        panic!("did not find target char")
    };

    let mut visited = HashSet::<Point2<usize>>::new();
    let mut to_visit = vec![start];
    let mut splitters_visited = 0;
    while let Some(point) = to_visit.pop() {
        if !visited.insert(point) {
            continue;
        }
        let next_points = if splitters[&point] == '^' {
            splitters_visited += 1;
            vec![point + RIGHT_USIZE, point - RIGHT_USIZE]
        } else {
            vec![point + DOWN_USIZE]
        };

        for next_point in next_points {
            if splitters.contains_key(&next_point) {
                to_visit.push(next_point);
            }
        }
    }
    splitters_visited
}

fn p2(input: &str) -> usize {
    let (splitters, Some(start)) =
        parse_to_char_map_and_find::<usize>(input, 'S')
    else {
        panic!("did not find target char")
    };

    let mut to_visit = vec![start];

    let mut cache = HashMap::<Point2<usize>, usize>::new();
    while let Some(point) = to_visit.pop() {
        if cache.contains_key(&point) {
            continue;
        }

        let next_points = if splitters[&point] == '^' {
            vec![point + RIGHT_USIZE, point - RIGHT_USIZE]
        } else {
            vec![point + DOWN_USIZE]
        }
        .into_iter()
        .filter(|point| splitters.contains_key(point))
        .collect_vec();

        // is None if any dependencies have not been computed yet
        let maybe_realities =
            next_points.iter().try_fold(0, |acc, next_point| {
                cache.get(next_point).map(|inner| inner + acc)
            });

        if next_points.is_empty() {
            cache.insert(point, 1);
        } else if let Some(realities) = maybe_realities {
            cache.insert(point, realities);
        } else {
            to_visit.push(point);
            for next_point in next_points {
                to_visit.push(next_point);
            }
        }
    }
    cache[&start]
}
