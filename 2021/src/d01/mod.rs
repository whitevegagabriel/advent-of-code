use itertools::Itertools;

pub fn solve(problem: &Vec<String>) -> (u32, u32) {
    let depths = problem
        .iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect_vec();
    (solve1(&depths), solve2(&depths))
}

fn solve1(depths: &Vec<u32>) -> u32 {
    let num_increase = depths.windows(2).filter(|w| w[1] > w[0]).count();
    u32::try_from(num_increase).unwrap()
}

fn solve2(depths: &Vec<u32>) -> u32 {
    let sums: Vec<u32> = depths.windows(3).map(|w| w.iter().sum()).collect_vec();
    solve1(&sums)
}

#[test]
fn test() {
    use itertools::Itertools;
    let input = include_str!("example.txt")
        .lines()
        .map(String::from)
        .collect_vec();
    let examples = crate::utils::parse_example_testcases(&input);
    for (idx, example) in examples.iter().enumerate() {
        println!("Example {}", idx + 1);
        let (answer1, answer2) = solve(&example.problem);
        assert_eq!(example.answer1, answer1);
        assert_eq!(example.answer2, answer2);
    }
}
