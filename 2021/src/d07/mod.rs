use crate::utils::{median_round_down, parse_numbers};
use itertools::Itertools;

pub fn solve(problem: &[&str]) -> (u64, u64) {
    let nums = parse_numbers(problem[0]);
    (solve1(nums.clone()), solve2(&nums))
}

fn solve1(crabs: Vec<u64>) -> u64 {
    let median = median_round_down(crabs.clone());
    crabs.iter().map(|c| c.abs_diff(median)).sum()
}

fn solve2(crabs: &[u64]) -> u64 {
    let crab_counts = crabs.iter().counts();
    let crab_counts = crab_counts
        .iter()
        .map(|(pos, qty)| (**pos, u64::try_from(*qty).unwrap()))
        .sorted()
        .collect_vec();

    // initialize variables
    let mut fuel_cost = crab_counts
        .iter()
        .map(|(pos, qty)| ((pos + 1) * ((pos + 1) + 1) / 2) * qty)
        .sum::<u64>();
    let mut inc = 0;
    let mut dec = crab_counts
        .iter()
        .map(|(pos, qty)| (pos + 1) * qty)
        .sum::<u64>();
    let mut num_crabs_on_left = 0;
    let mut num_crabs_on_right = crab_counts.iter().map(|(_, qty)| qty).sum::<u64>();

    let mut horizontal_pos = 0;
    let mut crabs = crab_counts.iter().peekable();

    // iterate until the next iteration would cost more in fuel
    while dec > inc {
        fuel_cost = fuel_cost + inc - dec;

        dec -= num_crabs_on_right;

        // recalculate number of crabs if current position has a crab
        if horizontal_pos == crabs.peek().unwrap().0 {
            let (_, num_crabs) = crabs.next().unwrap();
            num_crabs_on_left += num_crabs;
            num_crabs_on_right -= num_crabs;
        }

        inc += num_crabs_on_left;

        horizontal_pos += 1;
    }

    fuel_cost
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
