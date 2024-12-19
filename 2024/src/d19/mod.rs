use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::common::test;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 6);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 317);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 16);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 883443544805484);
}

fn p1(input: &str) -> usize {
    let (towels, required_patterns) = parse_input(input);

    let mut cache = HashMap::new();

    required_patterns.iter().filter(|pattern| {
        num_ways_pattern_can_be_made(pattern, &towels, &mut cache) > 0
    }).count() 
}

fn p2(input: &str) -> usize {
    let (towels, required_patterns) = parse_input(input);

    let mut cache = HashMap::new();

    required_patterns.iter().map(|pattern| {
        num_ways_pattern_can_be_made(pattern, &towels, &mut cache)
    }).sum()
}

fn num_ways_pattern_can_be_made<'b, 'a: 'b>(pattern: &'a str, towels: &HashSet<&str>, cache: &'b mut HashMap<&'a str, usize>) -> usize {
    if let Some(num_ways) = cache.get(pattern) {
        return *num_ways;
    }

    if pattern == "" {
        return 1;
    }

    let num_ways = (1..=pattern.len()).map(|split_idx| {
        let left = &pattern[0..split_idx];
        let right = &pattern[split_idx..];
        if towels.contains(left) {
            num_ways_pattern_can_be_made(right, towels, cache)
        } else {
            0
        }
    }).sum::<usize>();

    cache.insert(pattern, num_ways);

    num_ways
}

fn parse_input(input: &str) -> (HashSet<&str>, Vec<&str>) {
    let (towels_str, required_patterns_str) = input.split("\n\n").collect_tuple().unwrap();
    let towels = towels_str.split(", ").collect();
    let required_patterns = required_patterns_str.lines().collect();
    (towels, required_patterns)
}
