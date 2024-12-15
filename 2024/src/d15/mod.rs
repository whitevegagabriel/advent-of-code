use crate::common::{parse_to_char_map, test, Point2, Vector2};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use crate::common::RotationDirection::{Clockwise, Counterclockwise};

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 10092);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 1451928);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 9021);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 1462788);
}

fn p1(input: &str) -> usize {
    let (mut grid, direction_vectors) = parse_input(input);

    let mut curr = grid
        .iter()
        .find_map(|(k, v)| if v == &'@' { Some(*k) } else { None })
        .unwrap();

    for vector in direction_vectors {
        let mut next = curr + vector;
        while grid[&next] != '#' {
            let c = grid[&next];
            if c == '.' {
                grid.insert(curr + vector, '@');
                grid.insert(curr, '.');
                if curr + vector != next {
                    grid.insert(next, 'O');
                }
                curr += vector;
                break;
            }
            next += vector;
        }
    }

    grid.iter()
        .filter_map(|(k, v)| {
            if v == &'O' {
                Some((k.y * 100 + k.x) as usize)
            } else {
                None
            }
        })
        .sum()
}

fn p2(input: &str) -> usize {
    let (original_grid, direction_vectors) = parse_input(input);
    let mut grid = updated_grid(&original_grid);
    let mut curr = grid
        .iter()
        .find_map(|(k, v)| if v == &'@' { Some(*k) } else { None })
        .unwrap();

    for direction_vector in direction_vectors {
        let mut potential_grid = grid.clone();
        let mut moved = HashSet::new();
        let moved_all = recursive_move_all(curr, direction_vector, false, &mut potential_grid, &mut moved);
        if moved_all {
            curr += direction_vector;
            grid = potential_grid;  
        }
    }
    
    grid.iter()
        .filter_map(|(k, v)| {
            if v == &'[' {
                Some((k.y * 100 + k.x) as usize)
            } else {
                None
            }
        })
        .sum()
}

fn recursive_move_all(curr: Point2<isize>, direction_vector: Vector2<isize>, allow_perpendicular_checking: bool, grid: &mut HashMap<Point2<isize>, char>, moved: &mut HashSet<Point2<isize>>) -> bool {
    if moved.contains(&curr) {
        return true;
    }
    
    if grid[&curr] == '#' {
        return false;
    }
    
    if grid[&curr] == '.' {
        return true;
    }
    
    if !recursive_move_all(curr + direction_vector, direction_vector, true, grid, moved) {
        return false;
    }
    
    if !allow_perpendicular_checking || direction_vector.y == 0 {
        grid.insert(curr + direction_vector, grid[&curr]);
        grid.insert(curr, '.');
        moved.insert(curr);
        return true;
    }
    
    let (left_dir, right_dir) = if direction_vector.y == -1 {
        (direction_vector.rotated_90(Counterclockwise), direction_vector.rotated_90(Clockwise))
    } else {
        (direction_vector.rotated_90(Clockwise), direction_vector.rotated_90(Counterclockwise))
    };
    
    let left = curr + left_dir;
    let right = curr + right_dir;

    let new_curr = match (grid[&left], grid[&curr], grid[&right]) {
        ('[', ']', _) => left,
        (_, '[', ']') => right,
        _ => panic!(),
    };

    let moved_all = recursive_move_all(new_curr, direction_vector, false, grid, moved);
    
    if moved_all {
        grid.insert(curr + direction_vector, grid[&curr]);
        grid.insert(curr, '.');
        moved.insert(curr);
    }
    
    moved_all
}

fn updated_grid(grid: &HashMap<Point2<isize>, char>) -> HashMap<Point2<isize>, char> {
    grid.iter()
        .flat_map(|(k, v)| {
            let new_point_1 = Point2 { x: k.x * 2, y: k.y };
            let new_point_2 = Point2 {
                x: k.x * 2 + 1,
                y: k.y,
            };
            match *v {
                '#' => [(new_point_1, '#'), (new_point_2, '#')],
                'O' => [(new_point_1, '['), (new_point_2, ']')],
                '.' => [(new_point_1, '.'), (new_point_2, '.')],
                '@' => [(new_point_1, '@'), (new_point_2, '.')],
                _ => panic!(),
            }
        })
        .collect()
}

fn parse_input(input: &str) -> (HashMap<Point2<isize>, char>, Vec<Vector2<isize>>) {
    let (grid_input, directions_input) = input.split("\n\n").collect_tuple().unwrap();

    let grid = parse_to_char_map::<isize>(grid_input);
    let directions = directions_input
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Vector2 { x: 0, y: -1 }),
            '>' => Some(Vector2 { x: 1, y: 0 }),
            'v' => Some(Vector2 { x: 0, y: 1 }),
            '<' => Some(Vector2 { x: -1, y: 0 }),
            _ => None,
        })
        .collect_vec();
    (grid, directions)
}
