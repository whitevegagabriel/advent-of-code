use crate::common::{test, Point2, RotationDirection::Counterclockwise, Vector2};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 41);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 0);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 0);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 0);
}

fn p1(input: &str) -> usize {
    let (grid, start) = parse_input(input);

    let guard_position_iter = GuardPositionIterator {
        grid,
        position: Some(start),
        direction: Vector2 { x: 0, y: -1 },
    };

    let mut seen = HashSet::from([start]);
    let mut c = 0;

    for pos in guard_position_iter {
        if c < 10 {
            dbg!(&pos);
            c += 1;
        }
        seen.insert(pos);
    }

    seen.len()
}

fn p2(_input: &str) -> usize {
    0
}

fn parse_input(input: &str) -> (HashMap<Point2<isize>, char>, Point2<isize>) {
    let grid: HashMap<_, _> = input
        .lines()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(c_idx, c)| {
                    (
                        Point2 {
                            x: c_idx as isize,
                            y: line_idx as isize,
                        },
                        c,
                    )
                })
                .collect_vec()
        })
        .collect();

    let start = grid
        .iter()
        .find_map(|(point, c)| if c == &'^' { Some(*point) } else { None })
        .unwrap();

    (grid, start)
}

struct GuardPositionIterator {
    grid: HashMap<Point2<isize>, char>,
    position: Option<Point2<isize>>,
    direction: Vector2<isize>,
}

impl Iterator for GuardPositionIterator {
    type Item = Point2<isize>;

    fn next(&mut self) -> Option<Self::Item> {
        let position = self.position?;

        if !self.grid.contains_key(&(position + self.direction)) {
            self.position = None;
            return None;
        }

        while self.grid[&(position + self.direction)] == '#' {
            self.direction.rotate_90(Counterclockwise)
        }

        self.position = Some(position + self.direction);

        self.position
    }
}
