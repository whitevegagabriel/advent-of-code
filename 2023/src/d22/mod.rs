use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(problem: &str) -> (usize, usize) {
    let sand_map = problem
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let sand = Sand::parse(idx, line);
            (idx, sand)
        })
        .collect::<HashMap<usize, Sand>>();
    (solve1(&sand_map), solve2(&sand_map))
}

fn solve1(sand_map: &HashMap<usize, Sand>) -> usize {
    let processed_sand = process_sand(sand_map);

    let num_can_disintegrate = processed_sand
        .values()
        .filter(|sand| {
            let is_not_the_sole_support = sand.supporting.is_empty()
                || sand
                    .supporting
                    .iter()
                    .all(|id| processed_sand[id].supported_by.len() > 1);
            is_not_the_sole_support
        })
        .count();

    num_can_disintegrate
}

fn solve2(sand_map: &HashMap<usize, Sand>) -> usize {
    let processed_sand = process_sand(sand_map);

    let total_sand_would_fall = processed_sand
        .values()
        .map(|sand| {
            let mut all_supported_sands = HashSet::from([sand.id]);
            add_supported_sand_ids(sand, &processed_sand, &mut all_supported_sands);
            all_supported_sands.len() - 1
        })
        .sum::<usize>();

    total_sand_would_fall
}

fn add_supported_sand_ids(
    sand: &Sand,
    sand_map: &HashMap<usize, Sand>,
    all_supported_sands: &mut HashSet<usize>,
) {
    let mut to_process = vec![];
    for sand_id in &sand.supporting {
        let supported_by = &sand_map[sand_id].supported_by;
        if supported_by
            .iter()
            .any(|id| !all_supported_sands.contains(id))
        {
            continue;
        }
        all_supported_sands.insert(*sand_id);
        to_process.push(*sand_id);
    }

    for sand_id in to_process {
        add_supported_sand_ids(&sand_map[&sand_id], sand_map, all_supported_sands);
    }
}

fn process_sand(sand_to_be_processed: &HashMap<usize, Sand>) -> HashMap<usize, Sand> {
    let mut sand_map = sand_to_be_processed.clone();
    let sand_ids_in_order_to_be_processed = sand_map
        .values()
        .sorted_by_key(|sand| sand.min_z)
        .map(|sand| sand.id)
        .collect_vec();

    let mut all_sand = sand_map
        .values()
        .flat_map(|sand| sand.blocks.clone())
        .collect::<HashSet<_>>();

    for curr_sand_id in sand_ids_in_order_to_be_processed {
        for point3 in &sand_map[&curr_sand_id].blocks {
            all_sand.remove(point3);
        }

        while sand_map[&curr_sand_id].min_z > 0
            && all_sand
                .intersection(&sand_map[&curr_sand_id].blocks)
                .count()
                == 0
        {
            sand_map.get_mut(&curr_sand_id).unwrap().move_down();
        }

        // find all intersections
        let supporting_sand_ids = sand_map
            .values()
            .filter(|s| {
                if *s == &sand_map[&curr_sand_id] {
                    return false;
                }
                s.blocks
                    .intersection(&sand_map[&curr_sand_id].blocks)
                    .count()
                    > 0
            })
            .map(|sand| sand.id)
            .collect_vec();
        for supporting_sand_id in supporting_sand_ids {
            sand_map
                .get_mut(&supporting_sand_id)
                .unwrap()
                .supporting
                .push(curr_sand_id);
            sand_map
                .get_mut(&curr_sand_id)
                .unwrap()
                .supported_by
                .push(supporting_sand_id)
        }

        sand_map.get_mut(&curr_sand_id).unwrap().move_up();

        for point3 in &sand_map[&curr_sand_id].blocks {
            all_sand.insert(point3.clone());
        }
    }
    sand_map
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Sand {
    id: usize,
    blocks: HashSet<Point3>,
    min_z: usize,
    supporting: Vec<usize>,
    supported_by: Vec<usize>,
}

impl Sand {
    fn parse(id: usize, input: &str) -> Self {
        let (start, end) = input.split('~').map(Point3::parse).collect_tuple().unwrap();

        let x_range = if start.x <= end.x {
            start.x..=end.x
        } else {
            end.x..=start.x
        };
        let y_range = if start.y <= end.y {
            start.y..=end.y
        } else {
            end.y..=start.y
        };
        let z_range = if start.z <= end.z {
            start.z..=end.z
        } else {
            end.z..=start.z
        };

        let blocks = x_range
            .cartesian_product(y_range)
            .cartesian_product(z_range.clone())
            .map(|((x, y), z)| Point3 { x, y, z })
            .collect::<HashSet<Point3>>();

        Sand {
            id,
            blocks,
            min_z: z_range.min().unwrap(),
            supporting: vec![],
            supported_by: vec![],
        }
    }

    fn move_down(&mut self) {
        self.blocks = self
            .blocks
            .iter()
            .map(|point3| point3.moved_down())
            .collect::<HashSet<_>>();
        self.min_z -= 1;
    }

    fn move_up(&mut self) {
        self.blocks = self
            .blocks
            .iter()
            .map(|point3| point3.moved_up())
            .collect::<HashSet<_>>();
        self.min_z += 1;
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point3 {
    x: usize,
    y: usize,
    z: usize,
}

impl Point3 {
    fn parse(input: &str) -> Self {
        let (x, y, z) = input
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        Point3 { x, y, z }
    }

    fn moved_down(&self) -> Self {
        let mut point3 = self.clone();
        point3.z -= 1;
        point3
    }

    fn moved_up(&self) -> Self {
        let mut point3 = self.clone();
        point3.z += 1;
        point3
    }
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
