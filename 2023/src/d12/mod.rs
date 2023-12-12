use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(problem: &str) -> (u64, u64) {
    let springs_and_info = problem
        .lines()
        .map(|line| {
            let (springs, info) = line.split_whitespace().collect_tuple().unwrap();

            let springs = springs.chars().collect_vec();
            let info = info
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect_vec();

            (springs, info)
        })
        .collect_vec();
    (solve1(springs_and_info.clone()), solve2(springs_and_info))
}

fn solve1(mut springs_and_info: Vec<(Vec<char>, Vec<usize>)>) -> u64 {
    springs_and_info
        .iter_mut()
        .map(|(springs, info)| {
            let mut memory = HashMap::new();
            springs.push('.');
            info.push(0);
            num_configs(springs, info, 0, 0, 0, &mut memory)
        })
        .sum()
}

fn solve2(mut springs_and_info: Vec<(Vec<char>, Vec<usize>)>) -> u64 {
    springs_and_info
        .iter_mut()
        .map(|(springs, runs_of_broken_springs)| {
            let mut memory = HashMap::new();

            let mut springs = Itertools::intersperse((0..5).map(|_| springs.clone()), vec!['?'])
                .flatten()
                .collect_vec();
            springs.push('.');

            let mut runs_of_broken_springs = (0..5)
                .flat_map(|_| runs_of_broken_springs.clone())
                .collect_vec();
            runs_of_broken_springs.push(0);

            num_configs(&springs, &runs_of_broken_springs, 0, 0, 0, &mut memory)
        })
        .sum()
}

fn num_configs(
    springs: &[char],
    runs_of_broken_springs: &[usize],
    num_broken_springs: usize,
    s_idx: usize,
    r_idx: usize,
    memory: &mut HashMap<(usize, usize, usize), u64>,
) -> u64 {
    // using indexes instead of sub-slices is 4x faster
    let key = (s_idx, r_idx, num_broken_springs);
    if let Some(num) = memory.get(&key) {
        return *num;
    }

    let num_required_broken_springs = runs_of_broken_springs[r_idx];

    if springs.len() == s_idx {
        if num_required_broken_springs == 0 {
            return 1;
        }
        return 0;
    }

    let curr_spring = springs[s_idx];

    let mut should_handle_broken_spring = false;
    let mut should_handle_fixed_spring = false;

    if curr_spring == '#' {
        should_handle_broken_spring = true;
    } else if curr_spring == '.' {
        should_handle_fixed_spring = true;
    } else {
        should_handle_broken_spring = true;
        should_handle_fixed_spring = true;
    };

    let mut configs = 0;

    if should_handle_broken_spring && num_broken_springs != num_required_broken_springs {
        configs += num_configs(
            springs,
            runs_of_broken_springs,
            num_broken_springs + 1,
            s_idx + 1,
            r_idx,
            memory,
        );
    }

    if should_handle_fixed_spring {
        if num_broken_springs == 0 {
            configs += num_configs(springs, runs_of_broken_springs, 0, s_idx + 1, r_idx, memory);
        } else if num_broken_springs == num_required_broken_springs {
            configs += num_configs(springs, runs_of_broken_springs, 0, s_idx + 1, r_idx + 1, memory);
        }
    }

    memory.insert(key, configs);
    configs
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
