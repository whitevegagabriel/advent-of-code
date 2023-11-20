use crate::{d13::FoldInstruction::*, utils::parse_all_numbers};
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{hash_map::DefaultHasher, BTreeSet},
    hash::{Hash, Hasher},
};

pub fn solve(problem: &str) -> (u64, u64) {
    let problem = &problem.lines().collect_vec();
    let split_point = problem.iter().position(|l| l == &"").unwrap();
    let points: BTreeSet<_> = parse_all_numbers(&problem[..split_point])
        .iter()
        .map(|point| {
            let x = point[0];
            let y = point[1];
            (y, x)
        })
        .collect();
    let fold_instructions = problem[split_point + 1..]
        .iter()
        .map(|l| FoldInstruction::parse(l))
        .collect_vec();
    (
        solve1(points.clone(), &fold_instructions),
        solve2(points, &fold_instructions),
    )
}

fn solve1(points: BTreeSet<Point>, fold_instructions: &[FoldInstruction]) -> u64 {
    let points = fold_completely(points, &fold_instructions[0..1]);
    u64::try_from(points.len()).unwrap()
}

fn solve2(points: BTreeSet<Point>, fold_instructions: &[FoldInstruction]) -> u64 {
    let points = fold_completely(points, fold_instructions);
    display(&points);

    // because the technically correct answer is a String instead of a u64, I'm going to coerce it
    // into a u64
    let mut hasher = DefaultHasher::new();
    points.hash(&mut hasher);
    hasher.finish()
}

type Point = (u64, u64);

fn fold_completely(
    points: BTreeSet<Point>,
    fold_instructions: &[FoldInstruction],
) -> BTreeSet<Point> {
    fold_instructions
        .iter()
        .fold(points, |prev_points, fold_instruction| {
            let folder: Box<dyn Fn(&Point) -> Point> = match fold_instruction {
                AlongX(fold_col) => Box::new(|point| fold_across_col(*point, *fold_col)),
                AlongY(fold_row) => Box::new(|point| fold_across_row(*point, *fold_row)),
            };
            prev_points.iter().map(folder).collect()
        })
}

fn fold_across_col(point: Point, fold_col: u64) -> Point {
    let (row, col) = point;
    if col < fold_col {
        return (row, col);
    }
    let new_col = reflect(fold_col, col);
    (row, new_col)
}

fn fold_across_row(point: Point, fold_row: u64) -> Point {
    let (row, col) = point;
    if row < fold_row {
        return (row, col);
    }
    let new_row = reflect(fold_row, row);
    (new_row, col)
}

fn display(points: &BTreeSet<Point>) {
    let (min_row, max_row, min_col, max_col) =
        points
            .iter()
            .fold((u64::MAX, 0, u64::MAX, 0), |state, (row, col)| {
                let (min_row, max_row, min_col, max_col) = state;
                (
                    min_row.min(*row),
                    max_row.max(*row),
                    min_col.min(*col),
                    max_col.max(*col),
                )
            });
    let mut matrix =
        vec![vec!['.'; (max_col - min_col + 1) as usize]; (max_row - min_row + 1) as usize];
    for (row, col) in points {
        let mod_row = (row - min_row) as usize;
        let mod_col = (col - min_col) as usize;
        matrix[mod_row][mod_col] = '#';
    }
    let to_display = matrix
        .iter()
        .map(|r| r.iter().collect::<String>())
        .join("\n");
    println!("{to_display}");
}

fn reflect(stationary: u64, to_reflect: u64) -> u64 {
    let dist = stationary.abs_diff(to_reflect);
    match to_reflect.cmp(&stationary) {
        Ordering::Less => stationary + dist,
        Ordering::Equal => stationary,
        Ordering::Greater => stationary - dist,
    }
}

enum FoldInstruction {
    AlongX(u64),
    AlongY(u64),
}

impl FoldInstruction {
    fn parse(input: &str) -> Self {
        let (dir, fold_pos) = input[11..].split('=').collect_tuple().unwrap();
        let fold_pos = fold_pos.parse().unwrap();
        match dir {
            "x" => AlongX(fold_pos),
            "y" => AlongY(fold_pos),
            _ => panic!("....."),
        }
    }
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}

#[test]
fn test_reflect() {
    let (stay, to_reflect) = (2, 1);
    let reflected = reflect(stay, to_reflect);
    assert_eq!(3, reflected);

    let (stay, to_reflect) = (5, 9);
    let reflected = reflect(stay, to_reflect);
    assert_eq!(1, reflected);

    let (stay, to_reflect) = (100, 100);
    let reflected = reflect(stay, to_reflect);
    assert_eq!(100, reflected);
}
