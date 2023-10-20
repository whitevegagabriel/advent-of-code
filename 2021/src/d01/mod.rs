use itertools::Itertools;

pub fn solve(problem: &[&str]) -> (u32, u32) {
    let depths = problem
        .iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect_vec();
    (solve1(&depths), solve2(&depths))
}

fn solve1(depths: &[u32]) -> u32 {
    let num_increase = depths.windows(2).filter(|w| w[1] > w[0]).count();
    u32::try_from(num_increase).unwrap()
}

fn solve2(depths: &[u32]) -> u32 {
    let sums: Vec<u32> = depths.windows(3).map(|w| w.iter().sum()).collect_vec();
    solve1(&sums)
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
