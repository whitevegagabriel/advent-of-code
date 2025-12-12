use crate::common::test;
use itertools::Itertools;
use microlp::{ComparisonOp, OptimizationDirection, Problem};

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 7);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 527);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 33);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 19810);
}

fn p1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut items = line.split_whitespace();
            let indicators_str = items.next().unwrap();
            let indicators_bitmask = indicators_str
                .strip_circumfix('[', ']')
                .unwrap()
                .chars()
                .enumerate()
                .filter_map(|(i, c)| if c == '#' { Some(i) } else { None })
                .fold(0_u16, |acc, i| (1_u16 << i) | acc);

            let toggle_bitmasks = items
                .filter_map(|item| item.strip_circumfix('(', ')'))
                .map(|item| {
                    item.split(',')
                        .map(|num| num.parse::<usize>().unwrap())
                        .fold(0_u16, |acc, i| (1_u16 << i) | acc)
                })
                .collect_vec();

            fewest_presses_xor(indicators_bitmask, &toggle_bitmasks).unwrap()
        })
        .sum()
}

fn p2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut items = line.split_whitespace().rev();
            let joltage_str = items.next().unwrap();
            let mut joltage = [0_f64; 10];
            for (i, val) in joltage_str
                .strip_circumfix('{', '}')
                .unwrap()
                .split(',')
                .map(|num| num.parse::<f64>().unwrap())
                .enumerate()
            {
                joltage[i] = val;
            }

            let buttons = items
                .filter_map(|item| item.strip_circumfix('(', ')'))
                .map(|item| {
                    let mut button = [0_f64; 10];
                    for i in
                        item.split(',').map(|num| num.parse::<usize>().unwrap())
                    {
                        button[i] = 1.0;
                    }
                    button
                })
                .collect_vec();

            fewest_presses_microlp(joltage, &buttons)
        })
        .sum()
}

fn fewest_presses_xor(
    indicators_bitmask: u16,
    toggle_bitmasks: &[u16],
) -> Option<usize> {
    toggle_bitmasks
        .iter()
        .enumerate()
        .filter_map(|(i, toggle)| {
            if toggle == &indicators_bitmask {
                Some(1)
            } else {
                fewest_presses_xor(
                    indicators_bitmask ^ toggle,
                    &toggle_bitmasks[i + 1..],
                )
                .map(|total| total + 1)
            }
        })
        .min()
}

fn fewest_presses_microlp(joltage: [f64; 10], buttons: &[[f64; 10]]) -> usize {
    let mut problem = Problem::new(OptimizationDirection::Minimize);

    let vars = (0..buttons.len())
        .map(|_| problem.add_integer_var(1., (0, i32::MAX)))
        .collect_vec();

    for i in 0..10 {
        let expr = buttons
            .iter()
            .zip(vars.iter())
            .map(|(button, var)| (*var, button[i]))
            .collect_vec();
        problem.add_constraint(expr, ComparisonOp::Eq, joltage[i]);
    }

    problem.solve().unwrap().objective().round() as usize
}
