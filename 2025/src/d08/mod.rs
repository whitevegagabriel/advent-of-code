use crate::common::{test, test_with_params};
use disjoint::DisjointSet;
use itertools::Itertools;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test_with_params("example", MODULE, p1, 10, 40);
}

#[test]
fn p1_input() {
    test_with_params("input", MODULE, p1, 1000, 80446);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 25272);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 51294528);
}

fn p1(input: &str, k_closest: usize) -> usize {
    let num_coords = input.lines().count();
    let mut disjoint_set = DisjointSet::with_len(num_coords);
    for (i1, _, i2, _) in parse_combinations_sorted_by_distance(input)
        .into_iter()
        .take(k_closest)
    {
        disjoint_set.join(i1, i2);
    }
    disjoint_set
        .sets()
        .iter()
        .map(|set| set.len())
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn p2(input: &str) -> usize {
    let num_coords = input.lines().count();
    let mut disjoint_set = DisjointSet::with_len(num_coords);
    for (i1, coord1, i2, coord2) in parse_combinations_sorted_by_distance(input) {
        disjoint_set.join(i1, i2);
        if disjoint_set.sets().len() == 1 {
            return coord1.0 * coord2.0;
        }
    }
    panic!("Should have joined the disjoint set");
}

type Point3 = (usize, usize, usize);

fn parse_combinations_sorted_by_distance(input: &str) -> Vec<(usize, Point3, usize, Point3)> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect_tuple::<(_, _, _)>()
                .unwrap()
        })
        .enumerate()
        .tuple_combinations()
        .map(|((i1, coord1), (i2, coord2))| {
            let square_distance = (coord1.0.abs_diff(coord2.0)).pow(2)
                + (coord1.1.abs_diff(coord2.1)).pow(2)
                + (coord1.2.abs_diff(coord2.2)).pow(2);
            (square_distance, i1, coord1, i2, coord2)
        })
        .sorted()
        .map(|(_, i1, coord1, i2, coord2)| (i1, coord1, i2, coord2))
        .collect_vec()
}
