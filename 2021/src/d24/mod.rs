use itertools::Itertools;

pub fn solve(problem: &str) -> (u64, u64) {
    let _problem = &problem.lines().collect_vec();
    (solve1(), solve2())
}

fn solve1() -> u64 {
    0
}

fn solve2() -> u64 {
    0
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
