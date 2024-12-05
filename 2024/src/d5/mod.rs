use crate::common::test;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

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
    test("input", MODULE, p2, 0);
}

fn p1(input: &str) -> usize {
    let (rules, updates) = parse_input(input);

    updates.iter().filter_map(|pages| {
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

        Some(pages[pages.len()/2])
    }).sum()
}

fn p2(_input: &str) -> usize {
    0
}

fn parse_input(input: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let (rules_input, updates_input) = input.split("\n\n").collect_tuple().unwrap();

    let mut rules = HashMap::new();
    for line in rules_input.lines() {
        let (left, right) = line.split('|')
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
