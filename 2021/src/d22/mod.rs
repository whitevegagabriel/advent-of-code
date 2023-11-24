use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, digit1},
    combinator::{map, opt, recognize},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};
use std::{
    cmp::{max, min},
    collections::HashSet,
    ops::RangeInclusive,
};

pub fn solve(problem: &str) -> (u64, u64) {
    let (_, instructions) = separated_list1(
        tag("\n"),
        map(
            separated_pair(alpha1, tag(" "), Range3::parse),
            |(on_off, range3)| {
                let on_off = if on_off == "on" {
                    true
                } else if on_off == "off" {
                    false
                } else {
                    panic!("that was unexpected")
                };
                (on_off, range3)
            },
        ),
    )(problem)
    .unwrap();
    (solve1(&instructions), solve2(&instructions))
}

fn solve1(instructions: &[(bool, Range3)]) -> u64 {
    let instructions_iter = instructions.iter().cloned().map(|(on, instruction)| {
        let (x, y, z) = (
            instruction.x.clone(),
            instruction.y.clone(),
            instruction.z.clone(),
        );
        let (x_min, x_max) = x.into_inner();
        let (y_min, y_max) = y.into_inner();
        let (z_min, z_max) = z.into_inner();
        (
            on,
            Range3 {
                x: x_min.max(-50)..=x_max.min(50),
                y: y_min.max(-50)..=y_max.min(50),
                z: z_min.max(-50)..=z_max.min(50),
            },
        )
    });

    instructions_iter
        .fold(HashSet::new(), |mut on_set, (on, range)| {
            let operation = if on { add_to_set } else { remove_from_set };
            for ((x, y), z) in range
                .x
                .cartesian_product(range.y)
                .cartesian_product(range.z)
            {
                operation((x, y, z), &mut on_set)
            }
            on_set
        })
        .len() as u64
}

fn solve2(instructions: &[(bool, Range3)]) -> u64 {
    let mut on_cubes = Vec::new();
    for (on, cube) in instructions.iter() {
        if *on {
            let mut new_on_cubes = cube.minus_all(&on_cubes);
            on_cubes.append(&mut new_on_cubes);
            continue;
        }

        let renewed_on_cubes = on_cubes.iter().flat_map(|c| c.minus(cube)).collect_vec();

        on_cubes = renewed_on_cubes;
    }

    on_cubes.iter().map(Range3::volume).sum::<i64>() as u64
}

fn add_to_set(point3: Point3, set: &mut HashSet<Point3>) {
    set.insert(point3);
}

fn remove_from_set(point3: Point3, set: &mut HashSet<Point3>) {
    set.remove(&point3);
}

type Point3 = (i64, i64, i64);

#[derive(PartialEq, Debug, Clone)]
struct Range3 {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
    z: RangeInclusive<i64>,
}

impl Range3 {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_list1(
                tag(","),
                preceded(
                    tuple((anychar, tag("="))),
                    separated_pair(
                        recognize(tuple((opt(tag("-")), digit1))),
                        tag(".."),
                        recognize(tuple((opt(tag("-")), digit1))),
                    ),
                ),
            ),
            |ranges: Vec<(&str, &str)>| {
                let (x_min, x_max) = ranges[0];
                let (y_min, y_max) = ranges[1];
                let (z_min, z_max) = ranges[2];

                let (x_min, x_max) = (x_min.parse().unwrap(), x_max.parse().unwrap());
                let (y_min, y_max) = (y_min.parse().unwrap(), y_max.parse().unwrap());
                let (z_min, z_max) = (z_min.parse().unwrap(), z_max.parse().unwrap());

                Self {
                    x: x_min..=x_max,
                    y: y_min..=y_max,
                    z: z_min..=z_max,
                }
            },
        )(input)
    }

    fn minus_all(&self, to_subtract: &[Range3]) -> Vec<Range3> {
        let mut targets = Vec::from([self.clone()]);
        for range3 in to_subtract {
            targets = targets.iter().flat_map(|t| t.minus(range3)).collect_vec();
        }
        targets
    }

    fn minus(&self, to_subtract: &Range3) -> Vec<Self> {
        let x_intersection =
            *max(self.x.start(), to_subtract.x.start())..=*min(self.x.end(), to_subtract.x.end());
        let y_intersection =
            *max(self.y.start(), to_subtract.y.start())..=*min(self.y.end(), to_subtract.y.end());
        let z_intersection =
            *max(self.z.start(), to_subtract.z.start())..=*min(self.z.end(), to_subtract.z.end());

        if x_intersection.is_empty() || y_intersection.is_empty() || z_intersection.is_empty() {
            return vec![self.clone()];
        }

        let mut new_ranges = Vec::new();

        // right, x is (to_subtract end + 1)..=(self end), y is (self y range), z is (self z range)
        if to_subtract.x.end() < self.x.end() {
            new_ranges.push(Self {
                x: to_subtract.x.end() + 1..=*self.x.end(),
                y: self.y.clone(),
                z: self.z.clone(),
            })
        }

        // left, x is (self start)..=(to_subtract start - 1), y is (self y range), z is (self z range)
        if self.x.start() < to_subtract.x.start() {
            new_ranges.push(Self {
                x: *self.x.start()..=to_subtract.x.start() - 1,
                y: self.y.clone(),
                z: self.z.clone(),
            })
        }

        // front, x is (x intersection), y is (to_subtract end + 1)..=(self end), z is (self z range)
        if to_subtract.y.end() < self.y.end() {
            new_ranges.push(Self {
                x: x_intersection.clone(),
                y: to_subtract.y.end() + 1..=*self.y.end(),
                z: self.z.clone(),
            })
        }

        // back, x is (x intersection), y is (self start)..=(to_subtract start - 1), z is (self z range)
        if self.y.start() < to_subtract.y.start() {
            new_ranges.push(Self {
                x: x_intersection.clone(),
                y: *self.y.start()..=to_subtract.y.start() - 1,
                z: self.z.clone(),
            })
        }

        // top, x is (x intersection), y is (y intersection), z is (to_subtract end + 1)..=(self end)
        if to_subtract.z.end() < self.z.end() {
            new_ranges.push(Self {
                x: x_intersection.clone(),
                y: y_intersection.clone(),
                z: to_subtract.z.end() + 1..=*self.z.end(),
            })
        }

        // bottom, x is (x intersection), y is (y intersection), z is (self start)..=(to_subtract start - 1)
        if self.z.start() < to_subtract.z.start() {
            new_ranges.push(Self {
                x: x_intersection.clone(),
                y: y_intersection.clone(),
                z: *self.z.start()..=to_subtract.z.start() - 1,
            })
        }

        new_ranges
    }

    fn volume(&self) -> i64 {
        (self.x.end() - self.x.start() + 1)
            * (self.y.end() - self.y.start() + 1)
            * (self.z.end() - self.z.start() + 1)
    }
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}

#[test]
fn test_range_parser() {
    let input = "x=10..12,y=-10..12,z=1..2";
    let (_, actual) = Range3::parse(input).unwrap();
    assert_eq!(
        Range3 {
            x: 10..=12,
            y: -10..=12,
            z: 1..=2,
        },
        actual
    )
}
