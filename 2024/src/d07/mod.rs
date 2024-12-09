use crate::common::test;
use itertools::Itertools;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 3749);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 6392012777720);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 11387);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 61561126043536);
}

fn p1(input: &str) -> usize {
    let formulas = parse_input(input);

    formulas
        .iter()
        .filter_map(|f| {
            if is_valid(f, &[Operation::Add, Operation::Multiply]) {
                Some(f.result)
            } else {
                None
            }
        })
        .sum()
}

fn p2(input: &str) -> usize {
    let formulas = parse_input(input);

    formulas
        .iter()
        .filter_map(|f| {
            if is_valid(
                f,
                &[Operation::Add, Operation::Multiply, Operation::Concatenate],
            ) {
                Some(f.result)
            } else {
                None
            }
        })
        .sum()
}

fn is_valid(incomplete_formula: &IncompleteFormula, possible_operations: &[Operation]) -> bool {
    (0..incomplete_formula.inputs.len() - 1)
        .map(|_| possible_operations)
        .multi_cartesian_product()
        .any(|operations| {
            let mut op_idx = 0;
            incomplete_formula
                .inputs
                .clone()
                .into_iter()
                .reduce(|left, right| {
                    let res = match operations[op_idx] {
                        Operation::Add => left + right,
                        Operation::Multiply => left * right,
                        Operation::Concatenate => {
                            let num_digits_right = right.checked_ilog10().unwrap_or(0) + 1;
                            let left_shifted = left * 10_usize.pow(num_digits_right);
                            left_shifted + right
                        }
                    };

                    op_idx += 1;

                    res
                })
                .unwrap()
                == incomplete_formula.result
        })
}

fn parse_input(input: &str) -> Vec<IncompleteFormula> {
    input
        .lines()
        .map(|line| {
            let (result_input, inputs_input) = line.split(": ").collect_tuple().unwrap();

            IncompleteFormula {
                result: result_input.parse().unwrap(),
                inputs: inputs_input
                    .split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect(),
            }
        })
        .collect()
}

#[derive(Debug)]
struct IncompleteFormula {
    result: usize,
    inputs: Vec<usize>,
}

#[derive(Clone)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}
