use crate::utils::get_cross_neighbors;
use itertools::Itertools;
use num::Integer;
use std::collections::HashSet;

pub fn solve(problem: &str) -> (usize, usize) {
    let problem = problem
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let height = problem.len();
    let width = problem[0].len();

    let rock_coords = (0..height)
        .cartesian_product(0..width)
        .filter_map(|(row, col)| {
            if problem[row][col] == '#' {
                Some((row, col))
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();

    let start = ((height - 1) / 2, (width - 1) / 2);

    (
        solve1(&rock_coords, start, width, height),
        solve2(&rock_coords, start, width, height),
    )
}

fn solve1(
    rock_coords: &HashSet<(usize, usize)>,
    start: (usize, usize),
    width: usize,
    height: usize,
) -> usize {
    solve_base(rock_coords, start, width, height, 64)
}

fn solve2(
    rock_coords: &HashSet<(usize, usize)>,
    start: (usize, usize),
    width: usize,
    height: usize,
) -> usize {
    let max_steps = 26501365;

    // explicit calculation for the number of reachable positions given a number of steps to take
    let num_reachable = |max_steps| solve_base(rock_coords, start, width, height, max_steps);

    // how long it takes for the growth pattern to repeat
    let period_length = width * 2;

    // how much the previous iteration's growth increases by
    let growth_acceleration = num_reachable(period_length * 2 + 1)
        - 2 * num_reachable(period_length + 1)
        + num_reachable(1);

    // how many times the period repeats, rounded down
    let period_repetitions = max_steps / period_length;
    // the smallest iteration for which num_reachable must be calculated
    let base_iteration = max_steps % period_length;

    let num_reachable_base = num_reachable(base_iteration);
    let num_reachable_base_plus_period = num_reachable(base_iteration + period_length);

    // wizardry that is apparently true according to my paper/pencil calculations
    num_reachable_base
        + (period_repetitions)
            * (num_reachable_base_plus_period - num_reachable_base
                + (period_repetitions - 1) * growth_acceleration / 2)
}

fn solve_base(
    rock_coords: &HashSet<(usize, usize)>,
    start: (usize, usize),
    width: usize,
    height: usize,
    max_steps: usize,
) -> usize {
    let rock_coords = rock_coords
        .iter()
        .map(|(row, col)| (*row as isize, *col as isize))
        .collect::<HashSet<_>>();
    let start = (start.0 as isize, start.1 as isize);
    let width = width as isize;
    let height = height as isize;

    let mut to_visit = vec![start];

    let mut visited_odd = HashSet::new();
    let mut visited_even = HashSet::from([start]);

    let remap = |coord: (isize, isize)| -> (isize, isize) {
        let row = coord.0 % height;
        let row = if row >= 0 { row } else { height - row.abs() };

        let col = coord.1 % width;
        let col = if col >= 0 { col } else { width - col.abs() };

        (row, col)
    };

    for steps_taken in 1..=max_steps {
        let mut to_visit_next = vec![];
        for p in to_visit {
            for n in get_cross_neighbors(&p) {
                let n_remapped = remap(n);
                if rock_coords.contains(&n_remapped) {
                    continue;
                }

                if steps_taken.is_odd() {
                    if visited_odd.insert(n) {
                        to_visit_next.push(n);
                    }
                } else if visited_even.insert(n) {
                    to_visit_next.push(n);
                }
            }
        }

        to_visit = to_visit_next;
    }

    if max_steps.is_even() {
        visited_even.len()
    } else {
        visited_odd.len()
    }
}

#[allow(dead_code)]
fn display(hashtags: &HashSet<(isize, isize)>, os: &HashSet<(isize, isize)>) {
    let rows = hashtags.iter().chain(os).map(|p| p.0).collect_vec();
    let cols = hashtags.iter().chain(os).map(|p| p.1).collect_vec();
    let min_row = rows.iter().min().unwrap();
    let max_row = rows.iter().max().unwrap();
    let min_col = cols.iter().min().unwrap();
    let max_col = cols.iter().max().unwrap();

    let mut buf =
        vec![vec!['.'; (max_col - min_col + 1) as usize]; (max_row - min_row + 1) as usize];
    for (row, col) in hashtags.iter().map(|(r, c)| (r - min_row, c - min_col)) {
        buf[row as usize][col as usize] = '#';
    }
    for (row, col) in os.iter().map(|(r, c)| (r - min_row, c - min_col)) {
        buf[row as usize][col as usize] = 'O';
    }
    let graph = buf
        .into_iter()
        .map(|line| line.iter().collect::<String>())
        .join("\n");
    println!("{graph}");
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
