use crate::common::{test, Point2, Vector2};
use itertools::Itertools;
use regex::Regex;

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
    test("example", MODULE, p2, 875318608908);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 82510994362072);
}

fn p1(input: &str) -> usize {
    let machine_configs = parse_input(input);
    let a_cost = 3;
    let b_cost = 1;
    machine_configs
        .iter()
        .filter_map(|config| {
            (0..100)
                .cartesian_product(0..100)
                .find_map(|(b_uses, a_uses)| {
                    let start = Point2 { x: 0, y: 0 };
                    let end = start + config.button_b_mod * b_uses + config.button_a_mod * a_uses;
                    if end == config.prize {
                        Some(a_cost * a_uses + b_cost * b_uses)
                    } else {
                        None
                    }
                })
        })
        .sum()
}

fn p2(input: &str) -> usize {
    let mut machine_configs = parse_input(input);
    for config in &mut machine_configs {
        config.prize.x += 10000000000000;
        config.prize.y += 10000000000000;
    }
    
    let a_cost = 3;
    let b_cost = 1;
    machine_configs
        .iter()
        .filter_map(|config| {
            let (ax, ay) = (config.button_a_mod.x as isize, config.button_a_mod.y as isize);
            let (bx, by) = (config.button_b_mod.x as isize, config.button_b_mod.y as isize);
            let (px, py) = (config.prize.x as isize, config.prize.y as isize);
            let b_uses = (ax * py - ay * px) / (ax * by - ay * bx);
            let a_uses = (px - bx * b_uses) / ax;
            
            if b_uses < 0 || a_uses < 0 {
                return None;
            }
            
            if ax * a_uses + bx * b_uses == px && ay * a_uses + by * b_uses == py {
                Some((a_cost * a_uses + b_cost * b_uses) as usize)
            } else {
                None
            }
        })
        .sum()
}

fn parse_input(input: &str) -> Vec<MachineConfiguration> {
    let input_regex = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)
Button B: X\+(\d+), Y\+(\d+)
Prize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    input_regex
        .captures_iter(input)
        .map(|cap| {
            let a_x = cap[1].parse().unwrap();
            let a_y = cap[2].parse().unwrap();
            let b_x = cap[3].parse().unwrap();
            let b_y = cap[4].parse().unwrap();
            let prize_x = cap[5].parse().unwrap();
            let prize_y = cap[6].parse().unwrap();
            MachineConfiguration {
                button_a_mod: Vector2 { x: a_x, y: a_y },
                button_b_mod: Vector2 { x: b_x, y: b_y },
                prize: Point2 {
                    x: prize_x,
                    y: prize_y,
                },
            }
        })
        .collect_vec()
}

#[derive(Debug)]
struct MachineConfiguration {
    button_a_mod: Vector2<usize>,
    button_b_mod: Vector2<usize>,
    prize: Point2<usize>,
}
