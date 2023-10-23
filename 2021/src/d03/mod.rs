use crate::utils::transposed;
use itertools::Itertools;

pub fn solve(problem: &[&str]) -> (u64, u64) {
    (solve1(problem), solve2(problem))
}

fn solve1(problem: &[&str]) -> u64 {
    let bit_rows = transposed(
        &problem
            .iter()
            .map(|s| s.chars().collect_vec())
            .collect_vec(),
    );
    let gamma = bit_rows
        .iter()
        .map(|row| {
            let counts = row.iter().counts();
            let most_frequent = counts.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0;
            **most_frequent
        })
        .join("");
    let epsilon = gamma
        .chars()
        .map(|c| match c {
            '0' => '1',
            '1' => '0',
            _ => panic!("unexpected char for epsilon"),
        })
        .join("");

    let gamma_u64 = u64::from_str_radix(&gamma, 2).unwrap();
    let epsilon_u64 = u64::from_str_radix(&epsilon, 2).unwrap();
    gamma_u64 * epsilon_u64
}

fn solve2(problem: &[&str]) -> u64 {
    let oxygen_rating = get_rating(&mut problem.to_vec(), |p, n| most_frequent_nth(p, n, '1'));
    let scrubber_rating = get_rating(&mut problem.to_vec(), |p, n| least_frequent_nth(p, n, '0'));
    oxygen_rating * scrubber_rating
}

fn get_rating(problem: &mut Vec<&str>, comparator: impl Fn(&[&str], usize) -> char) -> u64 {
    let mut n = 0;
    while problem.len() > 1 {
        let cmp = comparator(problem, n);
        problem.retain(|line| line.chars().nth(n).unwrap() == cmp);
        n += 1;
    }
    let rating = problem.first().unwrap();
    u64::from_str_radix(rating, 2).unwrap()
}

fn most_frequent_nth(problem: &[&str], n: usize, default: char) -> char {
    let counts = problem
        .iter()
        .map(|line| line.chars().nth(n).unwrap())
        .counts();
    let mut max = *counts.get(&default).unwrap();
    let mut max_c = default;
    for (c, count) in counts {
        if count > max {
            max = count;
            max_c = c;
        }
    }
    max_c
}

fn least_frequent_nth(problem: &[&str], n: usize, default: char) -> char {
    let counts = problem
        .iter()
        .map(|line| line.chars().nth(n).unwrap())
        .counts();
    let mut min = *counts.get(&default).unwrap();
    let mut min_c = default;
    for (c, count) in counts {
        if count < min {
            min = count;
            min_c = c;
        }
    }
    min_c
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
