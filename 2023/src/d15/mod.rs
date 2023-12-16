use bitvec::macros::internal::funty::Fundamental;
use itertools::Itertools;
use linked_hash_map::LinkedHashMap;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};

pub fn solve(problem: &str) -> (usize, usize) {
    let problem = problem.lines().next().unwrap().split(',').collect_vec();

    let s1 = solve1(&problem);
    let s2 = solve2(&problem);
    let s2_alt = solve2_alt(&problem);
    assert_eq!(s2, s2_alt);
    (s1, s2)
}

fn solve1(init_sequence: &[&str]) -> usize {
    init_sequence.iter().map(|s| hash(s)).sum()
}

fn solve2(operation_sequence: &[&str]) -> usize {
    let mut box_hash_map = vec![vec![]; 256];
    for label_operation in operation_sequence {
        let (label, op) = parse_operation(label_operation).unwrap().1;
        let lens_box = &mut box_hash_map[hash(label)];
        let maybe_lens_idx = lens_box
            .iter()
            .find_position(|(l, _)| l == &label)
            .map(|(idx, _)| idx);
        if let Some(lens_idx) = maybe_lens_idx {
            match op {
                Operation::Add(focal_length) => lens_box[lens_idx] = (label, focal_length),
                Operation::Remove => {
                    lens_box.remove(lens_idx);
                }
            }
        } else if let Operation::Add(focal_length) = op {
            lens_box.push((label, focal_length));
        }
    }

    box_hash_map
        .iter()
        .enumerate()
        .map(|(box_idx, lens_box)| {
            lens_box
                .iter()
                .enumerate()
                .map(|(lens_idx, (_, focal_length))| (lens_idx + 1) * focal_length)
                .sum::<usize>()
                * (box_idx + 1)
        })
        .sum::<usize>()
}

// I saw someone use a LinkedHashMap and thought I'd try it
fn solve2_alt(operation_sequence: &[&str]) -> usize {
    let mut box_hash_map = vec![LinkedHashMap::<&str, usize>::new(); 256];
    for label_operation in operation_sequence {
        let (label, op) = parse_operation(label_operation).unwrap().1;
        let lens_box = &mut box_hash_map[hash(label)];
        match op {
            Operation::Add(focal_length) => {
                *lens_box.entry(label).or_default() = focal_length;
            }
            Operation::Remove => {
                lens_box.remove(label);
            }
        }
    }

    box_hash_map
        .iter()
        .enumerate()
        .map(|(box_idx, lens_box)| {
            lens_box
                .iter()
                .enumerate()
                .map(|(lens_idx, (_, focal_length))| (lens_idx + 1) * focal_length)
                .sum::<usize>()
                * (box_idx + 1)
        })
        .sum()
}

fn hash(input: &str) -> usize {
    input
        .chars()
        .map(|c| c.as_usize())
        .fold(0, |acc, curr| ((acc + curr) * 17) % 256)
}

#[derive(Debug)]
enum Operation {
    Add(usize),
    Remove,
}

fn parse_operation(input: &str) -> IResult<&str, (&str, Operation)> {
    tuple((
        alpha1,
        alt((
            map(preceded(tag("="), digit1), |num: &str| {
                Operation::Add(num.parse().unwrap())
            }),
            map(tag("-"), |_| Operation::Remove),
        )),
    ))(input)
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
