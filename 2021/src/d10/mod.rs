use crate::d10::ValidationResult::{Complete, Corrupted, Incomplete};
use itertools::Itertools;
use std::collections::LinkedList;

pub fn solve(problem: &str) -> (u64, u64) {
    let problem = &problem.lines().collect_vec();
    (solve1(problem), solve2(problem))
}

fn solve1(problem: &[&str]) -> u64 {
    problem
        .iter()
        .filter_map(|l| match validate(l) {
            Corrupted(c) => Some(c),
            _ => None,
        })
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("what?"),
        })
        .sum()
}

fn solve2(problem: &[&str]) -> u64 {
    let scores = problem
        .iter()
        .filter_map(|l| match validate(l) {
            Incomplete(chars) => Some(chars),
            _ => None,
        })
        .map(|chars| {
            println!("{chars:?}");
            let mut score = 0_u64;
            for c in chars {
                score *= 5;
                score += match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!("huh?"),
                };
            }
            println!("{score}");
            score
        })
        .sorted()
        .collect_vec();
    let midpoint = scores.len() / 2;
    scores[midpoint]
}

enum ValidationResult {
    Complete,
    Incomplete(LinkedList<char>),
    Corrupted(char),
}

fn validate(line: &str) -> ValidationResult {
    let mut stack = LinkedList::new();
    for c in line.chars() {
        if OPEN_BRACKETS.contains(&c) {
            stack.push_front(c);
            continue;
        }
        let next_bracket = stack.pop_front().unwrap_or_default();
        if c != closing_bracket_for(next_bracket) {
            return Corrupted(c);
        }
    }
    if stack.is_empty() {
        Complete
    } else {
        Incomplete(stack)
    }
}

const OPEN_BRACKETS: [char; 4] = ['[', '{', '(', '<'];

fn closing_bracket_for(c: char) -> char {
    match c {
        '[' => ']',
        '{' => '}',
        '(' => ')',
        '<' => '>',
        ' ' => ' ',
        _ => panic!("bracket does not exist"),
    }
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
