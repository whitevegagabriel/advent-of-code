use crate::common::test;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    ops::AddAssign,
};

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 143);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 5948);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 123);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 3062);
}

fn p1(input: &str) -> usize {
    let (rules, updates) = parse_input(input);

    updates
        .iter()
        .filter_map(|pages| {
            let mut seen = HashSet::new();
            for page in pages.iter() {
                seen.insert(*page);

                let Some(rule) = rules.get(page) else {
                    continue;
                };

                if rule.iter().any(|r| seen.contains(r)) {
                    return None;
                }
            }

            Some(pages[pages.len() / 2])
        })
        .sum()
}

fn p2(input: &str) -> usize {
    let (rules, updates) = parse_input(input);

    let mut incorrect_updates = updates
        .into_iter()
        .filter(|pages| {
            let mut seen = HashSet::new();
            for page in pages.iter() {
                seen.insert(*page);

                let Some(rule) = rules.get(page) else {
                    continue;
                };

                if rule.iter().any(|r| seen.contains(r)) {
                    return true;
                }
            }

            false
        })
        .collect_vec();

    for incorrect_update in incorrect_updates.iter_mut() {
        let rankings = generate_rankings(&rules, incorrect_update);

        incorrect_update.sort_by_key(|n| rankings.get(n).unwrap_or(&0))
    }

    incorrect_updates
        .iter()
        .map(|pages| pages[pages.len() / 2])
        .sum()
}

fn generate_rankings(
    rules: &HashMap<usize, Vec<usize>>,
    values: &[usize],
) -> HashMap<usize, usize> {
    let mut new_rules = rules.clone();
    let values: HashSet<_> = values.iter().collect();

    new_rules.retain(|k, _| values.contains(k));

    for rights in new_rules.values_mut() {
        rights.retain(|v| values.contains(v));
    }

    let mut rankings = HashMap::<usize, usize>::new();

    for left in new_rules.keys() {
        recursive_increment(left, &new_rules, &mut rankings);
    }

    rankings
}

fn recursive_increment(
    left: &usize,
    rules: &HashMap<usize, Vec<usize>>,
    rankings: &mut HashMap<usize, usize>,
) {
    let Some(rights) = rules.get(left) else {
        return;
    };

    for right in rights {
        rankings.entry(*right).or_insert(0).add_assign(1);
        recursive_increment(right, rules, rankings);
    }
}

fn parse_input(input: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let (rules_input, updates_input) = input.split("\n\n").collect_tuple().unwrap();

    let mut rules = HashMap::new();
    for line in rules_input.lines() {
        let (left, right) = line
            .split('|')
            .map(|n| n.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        rules.entry(left).or_insert(vec![]).push(right);
    }

    let updates = updates_input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    (rules, updates)
}
