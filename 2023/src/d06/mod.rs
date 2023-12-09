use itertools::Itertools;
use roots::find_roots_quadratic;

pub fn solve(problem: &str) -> (u64, u64) {
    (solve1(problem), solve2(problem))
}

fn solve1(problem: &str) -> u64 {
    let (times, distances) = problem
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|s| s.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_tuple()
        .unwrap();

    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| {
            match find_roots_quadratic(1f32, -(time as f32), distance as f32 + 0.01f32) {
                roots::Roots::Two([small, big]) => big as u64 - small as u64,
                _ => panic!("should only have two roots"),
            }
        })
        .product()
}

fn solve2(problem: &str) -> u64 {
    let (time, distance) = problem
        .lines()
        .map(|line| {
            line.split(":")
                .skip(1)
                .next()
                .unwrap()
                .replace(" ", "")
                .parse::<u64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    match find_roots_quadratic(1f64, -(time as f64), distance as f64 + 0.01f64) {
        roots::Roots::Two([small, big]) => big as u64 - small as u64,
        _ => panic!("should only have two roots"),
    }
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
