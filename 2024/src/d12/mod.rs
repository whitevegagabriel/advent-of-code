use crate::common::{
    get_cross_neighbors, get_cross_neighbors_with_direction, parse_to_char_map, test, Direction,
    Point2, Vector2,
};
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, AddAssign},
};

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 1930);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 1473408);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 1206);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 886364);
}

fn p1(input: &str) -> usize {
    let grid = parse_to_char_map::<isize>(input);

    let mut counted = HashSet::new();
    let mut measurements = vec![];

    for point in grid.keys() {
        if counted.contains(point) {
            continue;
        }

        let plot_measurement = measure(*point, &grid, &mut counted);
        measurements.push(plot_measurement);
    }

    measurements.iter().map(|m| m.area * m.perimeter).sum()
}

fn p2(input: &str) -> usize {
    let grid = parse_to_char_map::<isize>(input);

    let mut counted = HashSet::new();
    let mut measurements = vec![];

    for point in grid.keys() {
        if counted.contains(point) {
            continue;
        }

        let mut sides = vec![];
        let area = measure_area_and_aggregate_sides(*point, &grid, &mut counted, &mut sides);
        let reduced_sides = reduce_sides(&sides);

        measurements.push((area, reduced_sides.len()));
    }

    measurements
        .iter()
        .map(|(area, num_sides)| area * num_sides)
        .sum()
}

fn reduce_sides(sides: &[Side]) -> Vec<Side> {
    let mut sides = sides.to_owned();
    let mut absorbed = HashSet::new();
    let mut reduced_sides = vec![];

    while let Some(mut side) = sides.pop() {
        if absorbed.contains(&side) {
            continue;
        }
        absorbed.insert(side);

        while let Some((idx, to_absorb)) = sides.iter().enumerate().find(|(_, to_absorb)| {
            to_absorb.end == side.start && to_absorb.direction == side.direction
        }) {
            side.start = to_absorb.start;
            absorbed.insert(*to_absorb);
            sides.remove(idx);
        }

        while let Some((idx, to_absorb)) = sides.iter().enumerate().find(|(_, to_absorb)| {
            side.end == to_absorb.start && to_absorb.direction == side.direction
        }) {
            side.end = to_absorb.end;
            absorbed.insert(*to_absorb);
            sides.remove(idx);
        }

        reduced_sides.push(side);
    }

    reduced_sides
}

fn measure(point: Point2<isize>, grid: &Grid, counted: &mut HashSet<Point2<isize>>) -> Measurement {
    let plant = grid[&point];
    counted.insert(point);
    let mut measurement = Measurement {
        area: 1,
        perimeter: 0,
    };

    for neighbor in get_cross_neighbors(point) {
        let Some(neighbor_plant) = grid.get(&neighbor) else {
            measurement.perimeter += 1;
            continue;
        };

        if *neighbor_plant != plant {
            measurement.perimeter += 1;
            continue;
        }

        if counted.contains(&neighbor) {
            continue;
        }

        measurement += measure(neighbor, grid, counted);
    }

    measurement
}

fn measure_area_and_aggregate_sides(
    point: Point2<isize>,
    grid: &Grid,
    counted: &mut HashSet<Point2<isize>>,
    sides: &mut Vec<Side>,
) -> usize {
    let plant = grid[&point];
    counted.insert(point);
    let mut area = 1;

    for (neighbor, direction) in get_cross_neighbors_with_direction(point) {
        if let Some(neighbor_plant) = grid.get(&neighbor)
            && *neighbor_plant == plant
        {
            if counted.contains(&neighbor) {
                continue;
            }

            area += measure_area_and_aggregate_sides(neighbor, grid, counted, sides);
            continue;
        }

        let (start, end) = match direction {
            Direction::Up => (
                point + Vector2 { x: 0, y: 1 },
                point + Vector2 { x: 1, y: 1 },
            ),
            Direction::Down => (point, point + Vector2 { x: 1, y: 0 }),
            Direction::Left => (point, point + Vector2 { x: 0, y: 1 }),
            Direction::Right => (
                point + Vector2 { x: 1, y: 0 },
                point + Vector2 { x: 1, y: 1 },
            ),
        };

        let new_side = Side {
            start,
            end,
            direction,
        };
        sides.push(new_side);
    }

    area
}

type Grid = HashMap<Point2<isize>, char>;

struct Measurement {
    area: usize,
    perimeter: usize,
}

impl Add<Measurement> for Measurement {
    type Output = Self;

    fn add(self, rhs: Measurement) -> Self::Output {
        Measurement {
            area: self.area + rhs.area,
            perimeter: self.perimeter + rhs.perimeter,
        }
    }
}

impl AddAssign<Measurement> for Measurement {
    fn add_assign(&mut self, rhs: Measurement) {
        self.area += rhs.area;
        self.perimeter += rhs.perimeter;
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Side {
    start: Point2<isize>,
    end: Point2<isize>,
    direction: Direction,
}
