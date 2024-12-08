use crate::common::{parse_to_map, test, Point2, RotationDirection::Clockwise, Vector2};
use std::collections::{HashMap, HashSet};

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 41);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 4778);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 6);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 1618);
}

fn p1(input: &str) -> usize {
    let (grid, start) = parse_input(input);

    let visited = get_visited_positions(&grid, start);

    visited.len()
}

fn p2(input: &str) -> usize {
    let (grid, start) = parse_input(input);
    let visited = get_visited_positions(&grid, start);
    let start_dir = Vector2 { x: 0, y: 1 };

    let mut guard_position_iter = GuardPositionIterator {
        grid: grid.clone(),
        position: Some(start),
        direction: start_dir,
    };
    let mut obstacle_positions = HashSet::new();

    for pos in visited {
        if grid[&pos] == '#' {
            continue;
        }

        guard_position_iter.grid.insert(pos, '#');

        let mut seen = HashSet::from([(start, start_dir)]);

        for (inner_pos, inner_dir) in &mut guard_position_iter {
            if seen.contains(&(inner_pos, inner_dir)) {
                obstacle_positions.insert(pos);
                break;
            }
            seen.insert((inner_pos, inner_dir));
        }

        guard_position_iter.grid.insert(pos, '.');
        guard_position_iter.position = Some(start);
        guard_position_iter.direction = start_dir;
    }

    obstacle_positions.len()
}

fn get_visited_positions(
    grid: &HashMap<Point2<isize>, char>,
    start: Point2<isize>,
) -> HashSet<Point2<isize>> {
    let guard_position_iter = GuardPositionIterator {
        grid: grid.clone(),
        position: Some(start),
        direction: Vector2 { x: 0, y: 1 },
    };

    let mut seen = HashSet::from([start]);

    for (pos, _) in guard_position_iter {
        seen.insert(pos);
    }

    seen
}

fn parse_input(input: &str) -> (HashMap<Point2<isize>, char>, Point2<isize>) {
    let grid = parse_to_map(input);

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
    type Item = (Point2<isize>, Vector2<isize>);

    fn next(&mut self) -> Option<Self::Item> {
        let position = self.position?;

        if !self.grid.contains_key(&(position + self.direction)) {
            self.position = None;
            return None;
        }

        while self.grid[&(position + self.direction)] == '#' {
            self.direction.rotate_90(Clockwise)
        }

        self.position = Some(position + self.direction);

        self.position.map(|p| (p, self.direction))
    }
}
