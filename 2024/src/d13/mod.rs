use itertools::Itertools;
use regex::Regex;
use crate::common::{test, Point2, Vector2};

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 480);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 35997);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 0);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 0);
}

fn p1(input: &str) -> usize {
    let machine_configs = parse_input(input);
    let a_cost = 3;
    let b_cost = 1;
    machine_configs.iter().filter_map(|config| {
        (0..100).cartesian_product(0..100).find_map(|(b_uses, a_uses)| {
            let start = Point2 { x: 0, y: 0 };
            let end = start + config.button_b_mod * b_uses + config.button_a_mod * a_uses;
            if end == config.prize {
                Some(a_cost * a_uses + b_cost * b_uses)
            } else {
                None
            }
        })
    }).sum()
}

fn p2(_input: &str) -> usize {
    0
}

fn parse_input(input: &str) -> Vec<MachineConfiguration> {
    let input_regex = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)
Button B: X\+(\d+), Y\+(\d+)
Prize: X=(\d+), Y=(\d+)").unwrap();
    input_regex.captures_iter(input).map(|cap| {
        let a_x = cap[1].parse().unwrap();
        let a_y = cap[2].parse().unwrap();
        let b_x = cap[3].parse().unwrap();
        let b_y = cap[4].parse().unwrap();
        let prize_x = cap[5].parse().unwrap();
        let prize_y = cap[6].parse().unwrap();
        MachineConfiguration {
            button_a_mod: Vector2 {
                x: a_x,
                y: a_y,
            },
            button_b_mod: Vector2 {
                x: b_x,
                y: b_y,
            },
            prize: Point2 {
                x: prize_x,
                y: prize_y,
            },
        }
    }).collect_vec()
}

#[derive(Debug)]
struct MachineConfiguration {
    button_a_mod: Vector2<usize>,
    button_b_mod: Vector2<usize>,
    prize: Point2<usize>,
}
