use core::panic;
use itertools::Itertools;
use std::{
    collections::{BTreeMap, HashMap},
    ops::AddAssign,
};

pub fn solve(problem: &[&str]) -> (u64, u64) {
    let template = problem[0];
    let rules: HashMap<_, _> = problem[2..]
        .iter()
        .map(|l| {
            let (pairing, to_insert) = l.split(" -> ").collect_tuple().unwrap();
            (
                pairing.chars().collect_tuple::<(_, _)>().unwrap(),
                to_insert.chars().next().unwrap(),
            )
        })
        .collect();
    (solve1(template, &rules), solve2(template, &rules))
}

fn solve1(template: &str, rules: &HashMap<(char, char), char>) -> u64 {
    // brute force
    let polymer = (0..10).fold(template.chars().collect_vec(), |state, _| {
        let poly_insertions = state
            .clone()
            .into_iter()
            .tuple_windows::<(_, _)>()
            .map(|t| *rules.get(&t).unwrap());

        state.into_iter().interleave(poly_insertions).collect_vec()
    });
    let counts = polymer.iter().counts();
    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    u64::try_from(max - min).unwrap()
}

fn solve2(template: &str, rules: &HashMap<(char, char), char>) -> u64 {
    // cache-based recursive solution
    let mut counts = template.chars().counts();
    let mut cache = HashMap::new();
    for pair in template.chars().tuple_windows::<(_, _)>() {
        let sub_counts = polymer_counts(&pair, rules, 40, &mut cache);
        for (key, value) in sub_counts {
            counts.entry(key).or_default().add_assign(value);
        }
    }
    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    u64::try_from(max - min).unwrap()
}

fn polymer_counts(
    pair: &(char, char),
    rules: &HashMap<(char, char), char>,
    depth: usize,
    cache: &mut HashMap<((char, char), usize), BTreeMap<char, usize>>,
) -> BTreeMap<char, usize> {
    let cache_key = (*pair, depth);
    if let Some(counts) = cache.get(&cache_key) {
        return counts.clone();
    }

    let to_insert = rules.get(pair).unwrap();
    if depth == 1 {
        let mut counts = BTreeMap::new();
        counts.insert(*to_insert, 1);
        cache.insert(cache_key, counts.clone());
        counts
    } else if depth > 1 {
        let left_path = (pair.0, *to_insert);
        let right_path = (*to_insert, pair.1);
        let mut left_counts = polymer_counts(&left_path, rules, depth - 1, cache);
        let right_counts = polymer_counts(&right_path, rules, depth - 1, cache);
        for (key, value) in right_counts {
            left_counts.entry(key).or_default().add_assign(value);
        }
        left_counts.entry(*to_insert).or_default().add_assign(1);
        cache.insert(cache_key, left_counts.clone());
        left_counts
    } else {
        panic!("how about no")
    }
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
