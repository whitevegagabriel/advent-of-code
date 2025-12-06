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
    let operations = input_lines.last().unwrap().split_whitespace().collect_vec();

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
    let nums = 
    transpose(
        &input_lines[0..input_lines.len() - 1]
            .iter()
            .map(|line| line.chars().collect_vec())
            .collect_vec(),
    )
    .into_iter()
    .map(|vec| {
        vec.iter().filter(|c| !c.is_whitespace()).join("")
    })
    .chain(["".to_string()])
    .collect_vec();
    
    let mut result = 0;
    let mut intermediate = 0;
    let mut num_idx = 0;
    for op in input_lines.last().unwrap().split_whitespace() {
        while !nums[num_idx].is_empty() {
            let num = nums[num_idx].parse::<usize>().unwrap();
            if intermediate == 0 {
                intermediate = num;
            } else if op == "+" {
                intermediate += num;
            } else {
                intermediate *= num;
            }
            num_idx += 1
        }
        num_idx += 1;
        result += intermediate;
        intermediate = 0;
    }

    result
}
