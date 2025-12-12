use crate::common::{test, transpose};
use itertools::Itertools;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 4277556);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 4583860641327);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 3263827);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 11602774058280);
}

fn p1(input: &str) -> usize {
    let input_lines = input.lines().collect_vec();
    let vec_of_nums = transpose(
        &input_lines[0..input_lines.len() - 1]
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect_vec()
            })
            .collect_vec(),
    );
    let operations =
        input_lines.last().unwrap().split_whitespace().collect_vec();

    vec_of_nums
        .iter()
        .zip(operations)
        .map(|(nums, op)| {
            if op == "+" {
                nums.iter().sum::<usize>()
            } else {
                nums.iter().product()
            }
        })
        .sum()
}

fn p2(input: &str) -> usize {
    let input_lines = input.lines().collect_vec();

    /*
     * 123_328    1__
     * _45_64_ => 24_
     * __6_98_    356
     *            ___
     *            369
     *            248
     *            8__
     */
    let char_matrix = transpose(
        &input_lines
            .iter()
            .take(input_lines.len() - 1)
            .map(|line| line.chars().collect_vec())
            .collect_vec(),
    );
    let operations =
        input_lines.last().unwrap().split_whitespace().collect_vec();

    let all_nums = char_matrix
        .iter()
        .map(|vec| vec.iter().collect::<String>().trim().parse::<usize>().ok())
        .collect_vec();

    all_nums
        // operations are separated by empty rows. these end up becoming "unparsed" nums that we can split on.
        .split(|num| num.is_none())
        .map(|nums| nums.iter().filter_map(|&num| num))
        .zip(operations)
        .map(|(nums, op)| {
            if op == "+" {
                nums.sum::<usize>()
            } else {
                nums.product()
            }
        })
        .sum()
}
