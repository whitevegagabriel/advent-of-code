use crate::utils::parse_numbers;

pub fn solve(problem: &[&str]) -> (u64, u64) {
    let nums = parse_numbers(problem[0]);
    (solve1(&mut nums.clone()), solve2(&mut nums.clone()))
}

fn solve1(fishes: &mut [u64]) -> u64 {
    fishes_after_days(fishes, 80)
}

fn solve2(fishes: &mut [u64]) -> u64 {
    fishes_after_days(fishes, 256)
}

fn fishes_after_days(fishes: &[u64], days: u64) -> u64 {
    let mut fish_counts = [0_u64; 9];
    for fish in fishes {
        fish_counts[*fish as usize] += 1;
    }
    for _ in 0..days {
        fish_counts.rotate_left(1);
        fish_counts[6] += fish_counts[8];
    }
    fish_counts.iter().sum::<u64>()
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
