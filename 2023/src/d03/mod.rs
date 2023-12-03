use std::collections::{HashMap, HashSet};
use crate::utils::get_square_neighbors;

pub fn solve(problem: &str) -> (u64, u64) {
    let (schematic, height, width) = parse_schematic(problem);
    (solve1(&schematic, height, width), solve2(&schematic, height, width))
}

fn solve1(schematic: &HashMap<Point, SchematicEntry>, height: usize, width: usize) -> u64 {
    let mut part_numbers = Vec::new();
    for row_idx in 0..height {
        let mut part_number = 0;
        let mut connected_to_symbol = false;
        // must investigate one past the width to complete the row investigation
        for col_idx in 0..=width {
            let pos = (row_idx, col_idx);
            match schematic.get(&pos) {
                Some(SchematicEntry::Number(n)) => {
                    part_number = part_number * 10 + n;
                    connected_to_symbol |= any_neighbor_is_symbol(schematic, &pos);
                },
                // may be empty or contain symbol
                _ => {
                    // analyzing the input reveals that "0" is never a part number
                    if part_number > 0 {
                        if connected_to_symbol {
                            part_numbers.push(part_number);
                            connected_to_symbol = false;
                        }
                        part_number = 0;
                    }
                },
            }
        }
    }
    part_numbers.iter().sum()
}

fn solve2(schematic: &HashMap<Point, SchematicEntry>, height: usize, width: usize) -> u64 {
    let mut gear_part_numbers = HashMap::<_, Vec<u64>>::new();
    for row_idx in 0..height {
        let mut part_number = 0;
        let mut connected_gears = HashSet::new();
        // must investigate one past the width to complete the row investigation
        for col_idx in 0..=width {
            let pos = (row_idx, col_idx);
            match schematic.get(&pos) {
                Some(SchematicEntry::Number(n)) => {
                    part_number = part_number * 10 + n;
                    let gears = get_connected_gears(schematic, &pos);
                    for gear in gears {
                        connected_gears.insert(gear);
                    }
                },
                // may be empty or contain symbol
                _ => {
                    // analyzing the input reveals that "0" is never a part number
                    if part_number > 0 {
                        for gear in &connected_gears {
                            gear_part_numbers.entry(*gear).or_default().push(part_number)
                        }
                        connected_gears.clear();
                        part_number = 0;
                    }
                },
            }
        }
    }
    gear_part_numbers.values().map(|part_numbers| {
        match part_numbers.len() {
            2 => part_numbers.iter().product::<u64>(),
            _ => 0,
        }
    }).sum()
}

fn any_neighbor_is_symbol(schematic: &HashMap<Point, SchematicEntry>, point: &Point) -> bool {
    let neighbors = get_square_neighbors(point);
    neighbors.iter().any(|p| {
        if let Some(SchematicEntry::Symbol(_)) = schematic.get(p) {
            return true
        }
        false
    })
}

fn get_connected_gears(schematic: &HashMap<Point, SchematicEntry>, point: &Point) -> Vec<Point> {
    let mut neighbors = get_square_neighbors(point);
    neighbors.retain(|p| {
        if let Some(SchematicEntry::Symbol('*')) = schematic.get(p) {
            return true
        }
        false
    });
    neighbors
}

type Point = (usize, usize);

enum SchematicEntry {
    Number(u64),
    Symbol(char),
}

fn parse_schematic(input: &str) -> (HashMap<Point, SchematicEntry>, usize, usize) {
    let height = input.lines().count();
    let width = input.chars().take_while(|c| c != &'\n').count();
    let schematic = input.lines().enumerate().flat_map(|(row_idx, line)| {
        line.chars().enumerate().filter_map(move |(col_idx, c)| {
            let point = (row_idx, col_idx);
            if let Some(d) = c.to_digit(10) {
                Some((point, SchematicEntry::Number(d as u64)))
            } else if c == '.' {
                None
            } else {
                Some((point, SchematicEntry::Symbol(c)))
            }
        })
    }).collect();
    (schematic, height, width)
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
