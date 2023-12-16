pub fn solve(_problem: &str) -> (usize, usize) {
    (solve1(), solve2())
}

fn solve1() -> usize {
    0
}

fn solve2() -> usize {
    0
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
