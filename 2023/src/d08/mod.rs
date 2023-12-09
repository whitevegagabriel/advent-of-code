use itertools::Itertools;
use num::Integer;
use std::collections::HashMap;

pub fn solve(problem: &str) -> (u64, u64) {
    let steps = problem
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Step::Left,
            'R' => Step::Right,
            _ => panic!("expected L or R"),
        })
        .collect_vec();

    let node_map = problem
        .lines()
        .skip(2)
        .map(|line| {
            // ZZZ = (ZZZ, ZZZ)
            (&line[..3], (&line[7..10], &line[12..15]))
        })
        .collect::<HashMap<_, _>>();

    (solve1(&steps, &node_map), solve2(&steps, &node_map))
}

fn solve1(steps: &[Step], node_map: &HashMap<&str, (&str, &str)>) -> u64 {
    let mut node = "AAA";
    let mut total_steps = 0;
    for step in steps.iter().cycle() {
        node = match step {
            Step::Left => node_map[node].0,
            Step::Right => node_map[node].1,
        };
        total_steps += 1;
        if node == "ZZZ" {
            break;
        }
    }
    total_steps
}

fn solve2(steps: &[Step], node_map: &HashMap<&str, (&str, &str)>) -> u64 {
    let nodes = node_map
        .keys()
        .cloned()
        .filter(|key| key.ends_with('A'))
        .collect_vec();

    let loop_sizes = nodes
        .iter()
        .map(|node| steps_to_see_z(node, steps, node_map))
        .collect_vec();

    loop_sizes
        .into_iter()
        .reduce(|prev_loop, curr_loop| prev_loop.lcm(&curr_loop))
        .unwrap() as u64
}

fn steps_to_see_z(
    start_node: &str,
    steps: &[Step],
    node_map: &HashMap<&str, (&str, &str)>,
) -> usize {
    let mut node = start_node;
    for (steps_taken, step) in steps.iter().cycle().enumerate() {
        if node.ends_with('Z') {
            // assume the next node loops back to the start, as this problem is to be solved
            // using LCM
            return steps_taken;
        }
        node = match step {
            Step::Left => node_map[node].0,
            Step::Right => node_map[node].1,
        };
    }
    unreachable!()
}

enum Step {
    Left,
    Right,
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
