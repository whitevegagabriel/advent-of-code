pub fn solve(problem: &[&str]) -> (u32, u32) {
    (solve1(), solve2())
}

fn solve1() -> u32 {
    0
}

fn solve2() -> u32 {
    0
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
