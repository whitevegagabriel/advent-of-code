use crate::common::{
    parse_to_char_map, test, Point2,
    RotationDirection::{Clockwise, Counterclockwise},
    Vector2,
};
use itertools::Itertools;
use pathfinding::prelude::{astar_bag, AstarSolution};
use std::collections::HashMap;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 11048);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 107512);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 64);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 561);
}

fn p1(input: &str) -> usize {
    let (maze, start_pos, end_pos) = parse_input(input);
    let (_, cost) = compute_shortest_paths(&maze, start_pos, end_pos);

    cost
}

fn p2(input: &str) -> usize {
    let (maze, start_pos, end_pos) = parse_input(input);
    let (solutions, _) = compute_shortest_paths(&maze, start_pos, end_pos);

    solutions.flatten().map(|(pos, _)| pos).unique().count()
}

fn compute_shortest_paths(
    maze: &HashMap<Point2<isize>, char>,
    start_pos: Point2<isize>,
    end_pos: Point2<isize>,
) -> (AstarSolution<AStarPosition>, usize) {
    astar_bag(
        &(start_pos, Vector2 { x: 1, y: 0 }),
        |(curr_pos, curr_dir)| {
            let clockwise_dir = curr_dir.rotated_90(Clockwise);
            let counterclockwise_dir = curr_dir.rotated_90(Counterclockwise);
            let mut successors = vec![
                ((*curr_pos, clockwise_dir), 1000),
                ((*curr_pos, counterclockwise_dir), 1000),
            ];
            let forward_move = *curr_pos + *curr_dir;
            if maze[&forward_move] != '#' {
                successors.push(((forward_move, *curr_dir), 1));
            }
            successors
        },
        |(curr_pos, _)| curr_pos.x.abs_diff(end_pos.x) + curr_pos.y.abs_diff(end_pos.y),
        |(curr_pos, _)| curr_pos == &end_pos,
    )
    .unwrap()
}

fn parse_input(input: &str) -> (HashMap<Point2<isize>, char>, Point2<isize>, Point2<isize>) {
    let maze = parse_to_char_map::<isize>(input);
    let start_pos = maze
        .iter()
        .find_map(|(k, v)| if v == &'S' { Some(*k) } else { None })
        .unwrap();
    let end_pos = maze
        .iter()
        .find_map(|(k, v)| if v == &'E' { Some(*k) } else { None })
        .unwrap();

    (maze, start_pos, end_pos)
}

type AStarPosition = (Point2<isize>, Vector2<isize>);
