use itertools::Itertools;
use regex::Regex;
use roots::{find_roots_quadratic, Roots};
use std::{cmp::Ordering, ops::RangeInclusive};

pub fn solve(problem: &[&str]) -> (u64, u64) {
    let re = Regex::new(r"target area: x=([0-9]+)\.\.([0-9]+), y=(-[0-9]+)\.\.(-[0-9]+)").unwrap();
    let captures = re.captures(problem[0]).unwrap();

    let (x_start, x_end, y_start, y_end) = captures
        .iter()
        .skip(1)
        .while_some()
        .map(|item| item.as_str().parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap();
    let square_range = SquareRange::new(x_start, x_end, y_start, y_end);

    (solve1(&square_range), solve2(&square_range))
}

fn solve1(square_range: &SquareRange) -> u64 {
    assert!(square_range.y_end < 0);
    assert!(square_range.x_start > 0);

    let starting_y_vel = u64::try_from(-square_range.y_start).unwrap() - 1;
    (starting_y_vel) * (starting_y_vel + 1) / 2
}

fn solve2(square_range: &SquareRange) -> u64 {
    assert!(square_range.y_end < 0);
    assert!(square_range.x_start > 0);

    let min_x_vel_inc = required_velocity_to_travel_at_least(square_range.x_start);
    let max_x_vel_inc = square_range.x_end + 1;
    let min_y_vel_inc = square_range.y_start;
    let max_y_vel_inc = -square_range.y_start - 1;

    (min_x_vel_inc..=max_x_vel_inc)
        .cartesian_product(min_y_vel_inc..=max_y_vel_inc)
        .filter(|(x_vel, y_vel)| {
            let probe = Probe::new(*x_vel, *y_vel);
            probe_crosses_square_range(probe, square_range)
        })
        .count() as u64
}

fn probe_crosses_square_range(mut probe: Probe, square_range: &SquareRange) -> bool {
    assert!(probe.x_vel > 0);
    while probe.x_pos <= square_range.x_end && probe.y_pos >= square_range.y_start {
        if square_range.contains(&probe.position()) {
            return true;
        }
        probe.tick()
    }
    false
}

fn required_velocity_to_travel_at_least(dist: i32) -> i32 {
    assert!(dist > 0);

    // solve quadratic formula that results from sum of an arithmetic series
    // where velocity starts at its maximum and decreases by one every step up to this desired dist
    let roots = find_roots_quadratic(1f64, 1f64, f64::from(dist) * -2f64);
    let vel = match roots {
        Roots::No(_) => panic!("how...?"),
        Roots::One(roots) => roots[0],
        Roots::Two(roots) => {
            let root = roots
                .iter()
                .max_by(|f1, f2| f1.partial_cmp(f2).unwrap())
                .unwrap();
            assert!(root > &0f64);
            *root
        }
        _ => panic!("impossible..."),
    };
    vel as i32
}

#[derive(Debug)]
struct Probe {
    x_pos: i32,
    y_pos: i32,
    x_vel: i32,
    y_vel: i32,
}

impl Probe {
    fn new(x_vel: i32, y_vel: i32) -> Self {
        Self {
            x_pos: 0,
            y_pos: 0,
            x_vel,
            y_vel,
        }
    }

    fn tick(&mut self) {
        self.x_pos += self.x_vel;
        self.y_pos += self.y_vel;
        self.x_vel -= match self.x_vel.cmp(&0) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };
        self.y_vel -= 1;
    }

    fn position(&self) -> (i32, i32) {
        (self.x_pos, self.y_pos)
    }
}

struct SquareRange {
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

impl SquareRange {
    fn new(x_start: i32, x_end: i32, y_start: i32, y_end: i32) -> Self {
        assert!(y_start <= y_end);
        assert!(x_start <= x_end);
        Self {
            x_start,
            x_end,
            y_start,
            y_end,
            x_range: (x_start..=x_end),
            y_range: (y_start..=y_end),
        }
    }

    fn contains(&self, pos: &(i32, i32)) -> bool {
        self.x_range.contains(&pos.0) && self.y_range.contains(&pos.1)
    }
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
