use itertools::Itertools;

use crate::common::test;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 357);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 17554);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 3121910778619);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 175053592950232);
}

fn p1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (max1, max2) = line.chars().tuple_combinations::<(_, _)>().max().unwrap();
            format!("{max1}{max2}").parse::<usize>().unwrap()
        })
        .sum()
}

fn p2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let bank = line
                .chars()
                .map(|char| char.to_digit(10).unwrap() as usize)
                .collect_vec();
            max_combination(&bank, 12, 0)
        })
        .sum()
}

fn max_combination(digits: &[usize], take: usize, prev: usize) -> usize {
    if take > digits.len() || take == 0 || digits.is_empty() {
        return prev;
    }

    let max_value = digits[..=digits.len() - take].iter().max().unwrap();
    digits
        .iter()
        .enumerate()
        .filter_map(|(i, value)| if value == max_value { Some(i) } else { None })
        .map(|max_index| max_combination(&digits[max_index + 1..], take - 1, prev * 10 + max_value))
        .max()
        .unwrap()
}
