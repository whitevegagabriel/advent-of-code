use itertools::Itertools;

pub fn solve(problem: &[&str]) -> (u64, u64) {
    let depths = problem
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();
    (solve1(&depths), solve2(&depths))
}

fn solve1(depths: &[u64]) -> u64 {
    let num_increase = depths.windows(2).filter(|w| w[1] > w[0]).count();
    u64::try_from(num_increase).unwrap()
}

fn solve2(depths: &[u64]) -> u64 {
    let sums: Vec<u64> = depths.windows(3).map(|w| w.iter().sum()).collect_vec();
    solve1(&sums)
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
