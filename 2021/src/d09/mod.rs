use itertools::Itertools;
use std::collections::{HashMap, LinkedList};

pub fn solve(problem: &[&str]) -> (u64, u64) {
    let num_rows = problem.len();
    let num_cols = problem[0].len();
    let heights: HashMap<_, _> = (0..num_rows)
        .cartesian_product(0..num_cols)
        .map(|pos| {
            let h = problem[pos.0]
                .chars()
                .nth(pos.1)
                .unwrap()
                .to_digit(10)
                .unwrap() as u64;
            (pos, h)
        })
        .collect();
    (solve1(&heights), solve2(heights))
}

fn solve1(heights: &HashMap<(usize, usize), u64>) -> u64 {
    basin_points(heights)
        .iter()
        .map(|p| heights.get(p).unwrap() + 1)
        .sum()
}

fn solve2(mut heights: HashMap<(usize, usize), u64>) -> u64 {
    basin_points(&heights)
        .iter()
        .map(|p| {
            let mut next_points = LinkedList::new();
            next_points.push_back(*p);
            heights.remove(p);

            let mut qty_in_basin = 1;
            while let Some(next) = next_points.pop_front() {
                for (neighbor_p, h) in get_neighbors(&heights, &next) {
                    heights.remove(&neighbor_p);
                    if h >= 9 {
                        continue;
                    }
                    qty_in_basin += 1;
                    next_points.push_back(neighbor_p);
                }
            }
            qty_in_basin
        })
        // take the top three
        .sorted()
        .rev()
        .take(3)
        // and multiply them
        .product()
}

fn basin_points(heights: &HashMap<(usize, usize), u64>) -> Vec<(usize, usize)> {
    heights
        .keys()
        .filter_map(|pos| {
            let height = heights.get(pos).unwrap();
            let neighbor_heights = get_neighbors(heights, pos);
            if neighbor_heights
                .iter()
                .all(|(_, neighbor_height)| neighbor_height > height)
            {
                Some(*pos)
            } else {
                None
            }
        })
        .collect()
}

fn get_neighbors(
    heights: &HashMap<(usize, usize), u64>,
    point: &(usize, usize),
) -> Vec<((usize, usize), u64)> {
    // coord, height
    let point = (point.0 as i64, point.1 as i64);
    let neighbors = [-1, 0, 1, 0]
        .iter()
        .zip([0, -1, 0, 1])
        .filter_map(|diff| {
            let row_new = usize::try_from(point.0 + diff.0);
            let col_new = usize::try_from(point.1 + diff.1);
            if let (Ok(row_new), Ok(col_new)) = (row_new, col_new) {
                heights
                    .get(&(row_new, col_new))
                    .map(|h| ((row_new, col_new), *h))
            } else {
                None
            }
        })
        .collect_vec();
    neighbors
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
