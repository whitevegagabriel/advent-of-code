use crate::utils::transposed;
use itertools::Itertools;

pub fn solve(problem: &str) -> (u64, u64) {
    let problem = problem
        .split("\n\n")
        .map(|pat| pat.lines().map(|l| l.chars().collect_vec()).collect_vec())
        .collect_vec();
    (solve1(&problem), solve2(&problem))
}

fn solve1(patterns: &[Vec<Vec<char>>]) -> u64 {
    patterns
        .iter()
        .map(|pattern| {
            if let Some(row_pos) = get_reflection_pos(pattern) {
                return row_pos * 100;
            }

            if let Some(col_pos) = get_reflection_pos(&transposed(pattern)) {
                return col_pos;
            }

            unreachable!()
        })
        .sum()
}

fn solve2(patterns: &[Vec<Vec<char>>]) -> u64 {
    patterns
        .iter()
        .map(|pattern| {
            if let Some(row_pos) = get_secondary_reflection_pos(pattern) {
                return row_pos * 100;
            }

            if let Some(col_pos) = get_secondary_reflection_pos(&transposed(pattern)) {
                return col_pos;
            }

            unreachable!()
        })
        .sum()
}

fn get_reflection_pos(pattern: &[Vec<char>]) -> Option<u64> {
    for lo_start in 0..=pattern.len() - 2 {
        if (0..=lo_start)
            .rev()
            .zip(lo_start + 1..pattern.len())
            .all(|(lo, hi)| pattern[lo] == pattern[hi])
        {
            return Some((lo_start + 1) as u64);
        }
    }
    None
}

fn get_secondary_reflection_pos(pattern: &[Vec<char>]) -> Option<u64> {
    for lo_start in 0..=pattern.len() - 2 {
        let mut cleaned_smudge = false;
        let mut all_match = true;
        for (lo, hi) in (0..=lo_start).rev().zip(lo_start + 1..pattern.len()) {
            if pattern[lo] == pattern[hi] {
                continue;
            } else if !cleaned_smudge && diff(&pattern[lo], &pattern[hi]) == 1 {
                cleaned_smudge = true;
                continue;
            } else {
                all_match = false;
                break;
            }
        }

        // only care about matches that happened because of the cleaned smudge
        if cleaned_smudge && all_match {
            return Some((lo_start + 1) as u64);
        }
    }
    None
}

fn diff(l1: &[char], l2: &[char]) -> usize {
    l1.iter().zip(l2.iter()).filter(|(c1, c2)| c1 != c2).count()
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
