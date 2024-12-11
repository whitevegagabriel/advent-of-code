use crate::common::{count_digits, test};
use std::collections::HashMap;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 55312);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 198075);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 65601038650482);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 235571309320764);
}

fn p1(input: &str) -> usize {
    count_final_stones(25, &parse_input(input))
}

fn p2(input: &str) -> usize {
    count_final_stones(75, &parse_input(input))
}

fn count_final_stones(blinks: usize, stones: &[usize]) -> usize {
    let mut cache = HashMap::new();
    let mut total_stones = 0;

    for stone in stones {
        total_stones += blink_recursive(*stone, blinks, &mut cache);
    }

    total_stones
}

fn blink_recursive(
    stone: usize,
    times: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let num_stones_key = (stone, times);
    if let Some(num_stones) = cache.get(&num_stones_key) {
        return *num_stones;
    }

    if times == 0 {
        return 1;
    }

    let num_stones = if stone == 0 {
        blink_recursive(1, times - 1, cache)
    } else if count_digits(stone) % 2 == 0 {
        let (left, right) = split_digits(stone);
        blink_recursive(left, times - 1, cache) + blink_recursive(right, times - 1, cache)
    } else {
        blink_recursive(stone * 2024, times - 1, cache)
    };

    cache.insert(num_stones_key, num_stones);
    num_stones
}

fn split_digits(num: usize) -> (usize, usize) {
    let num_digits = count_digits(num);
    let factor = 10_usize.pow((num_digits / 2) as u32);
    let left = num / factor;
    let right = num - left * factor;
    (left, right)
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}
