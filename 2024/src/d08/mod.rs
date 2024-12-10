use crate::common::{parse_to_char_map, test, Point2};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 14);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 400);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 34);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 1280);
}

fn p1(input: &str) -> usize {
    let grid = parse_to_char_map::<isize>(input);
    let node_positions = get_node_positions(&grid);

    let mut antinodes = HashSet::new();

    for node_type in node_positions.keys() {
        for (node1, node2) in node_positions[node_type].iter().tuple_combinations() {
            let dir_vector = *node2 - *node1;
            let antinode1 = *node1 - dir_vector;
            let antinode2 = *node2 + dir_vector;

            if grid.contains_key(&antinode1) {
                antinodes.insert(antinode1);
            }
            if grid.contains_key(&antinode2) {
                antinodes.insert(antinode2);
            }
        }
    }

    antinodes.len()
}

fn p2(input: &str) -> usize {
    let grid = parse_to_char_map::<isize>(input);
    let node_positions = get_node_positions(&grid);

    let mut antinodes = HashSet::new();

    for node_type in node_positions.keys() {
        for (node1, node2) in node_positions[node_type].iter().tuple_combinations() {
            let mut dir_vector = *node2 - *node1;
            dir_vector.simplify();

            let mut maybe_antinode = *node1;
            while grid.contains_key(&maybe_antinode) {
                antinodes.insert(maybe_antinode);
                maybe_antinode -= dir_vector;
            }

            let mut maybe_antinode = *node1 + dir_vector;
            while grid.contains_key(&maybe_antinode) {
                antinodes.insert(maybe_antinode);
                maybe_antinode += dir_vector;
            }
        }
    }

    antinodes.len()
}

fn get_node_positions(grid: &HashMap<Point2<isize>, char>) -> HashMap<char, Vec<Point2<isize>>> {
    let mut node_positions: HashMap<char, Vec<_>> = HashMap::new();
    for (k, v) in grid.iter().filter(|(_, v)| v != &&'.') {
        node_positions.entry(*v).or_default().push(*k);
    }
    node_positions
}
