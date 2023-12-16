use itertools::Itertools;

pub fn solve(problem: &str) -> (usize, usize) {
    let number_lists = problem
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    (solve1(&number_lists), solve2(&number_lists))
}

fn solve1(number_lists: &[Vec<i64>]) -> usize {
    number_lists
        .iter()
        .map(|list| next_number_of(list))
        .sum::<i64>() as usize
}

fn solve2(number_lists: &[Vec<i64>]) -> usize {
    number_lists
        .iter()
        .map(|list| {
            let rev_list = list.iter().cloned().rev().collect_vec();
            next_number_of(&rev_list)
        })
        .sum::<i64>() as usize
}

fn next_number_of(numbers: &[i64]) -> i64 {
    if numbers.iter().all(|n| n == &0) {
        return 0;
    }

    let accelerated = acceleration_of(numbers);
    next_number_of(&accelerated) + numbers.last().unwrap()
}

fn acceleration_of(numbers: &[i64]) -> Vec<i64> {
    numbers
        .iter()
        .tuple_windows()
        .map(|(n1, n2)| n2 - n1)
        .collect_vec()
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
