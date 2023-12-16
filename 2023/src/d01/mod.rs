use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(problem: &str) -> (usize, usize) {
    let problem = problem.lines().collect_vec();
    (solve1(&problem), solve2(&problem))
}

fn solve1(lines: &[&str]) -> usize {
    lines
        .iter()
        .map(|line| {
            let first = line.chars().find(|c| c.is_ascii_digit()).unwrap_or('0'); // so that I can technically solve Part Two input with Part One code
            let last = line
                .chars()
                .rev()
                .find(|c| c.is_ascii_digit())
                .unwrap_or('0');
            format!("{first}{last}").parse::<usize>().unwrap()
        })
        .sum()
}

fn solve2(lines: &[&str]) -> usize {
    // problem description excludes zero
    let number_mapping = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let number_lengths_sorted = number_mapping
        .keys()
        .map(|k| k.len())
        .dedup()
        .sorted()
        .collect_vec();

    lines
        .iter()
        .map(|line| {
            let mut nums = vec![];
            // Check every index because some words bleed in together. To optimize, I could instead
            // do a reverse search to find the last number, but I don't feel like it.
            for (idx, char) in line.chars().enumerate() {
                if let Some(digit) = char.to_digit(10) {
                    nums.push(digit);
                    continue;
                }

                let maybe_num = number_lengths_sorted
                    .iter()
                    .filter_map(|len| line.get(idx..idx + len).and_then(|s| number_mapping.get(s)))
                    .next();

                if let Some(num) = maybe_num {
                    nums.push(*num);
                }
            }
            let value = nums.first().unwrap() * 10 + nums.last().unwrap();
            value
        })
        .sum::<u32>() as usize
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
