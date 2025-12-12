use crate::common::test;
use itertools::Itertools;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 3);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 701);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 14);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 352340558684863);
}

fn p1(input: &str) -> usize {
    let (ranges_str, ingredients_str) =
        input.split("\n\n").collect_tuple().unwrap();
    let ranges = ranges_str
        .lines()
        .map(|line| {
            let (start, end) = line
                .split('-')
                .map(|num| num.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            start..=end
        })
        .collect_vec();
    ingredients_str
        .lines()
        .filter(|line| {
            let ingredient = line.parse::<usize>().unwrap();
            ranges.iter().any(|range| range.contains(&ingredient))
        })
        .count()
}

fn p2(input: &str) -> usize {
    input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split('-')
                .map(|num| num.parse::<usize>().unwrap())
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .sorted()
        .fold(Vec::<(usize, usize)>::new(), |mut ranges, next_range| {
            let (next_start, next_end) = next_range;
            match ranges.last().cloned() {
                None => {
                    ranges.push(next_range);
                }
                Some((_, last_end)) if next_start > last_end => {
                    ranges.push(next_range);
                }
                Some((last_start, last_end)) if next_end > last_end => {
                    ranges.pop();
                    ranges.push((last_start, next_end));
                }
                _ => {}
            }

            ranges
        })
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}
