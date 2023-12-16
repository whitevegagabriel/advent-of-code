use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(problem: &str) -> (usize, usize) {
    let problem = problem
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    (solve1(problem.clone()), solve2(problem))
}

fn solve1(mut problem: Vec<Vec<char>>) -> usize {
    problem.insert(0, vec!['#'; problem[0].len()]);
    let width = problem[0].len();
    let height = problem.len();
    (0..width)
        .map(|col| {
            let mut num_rocks = 0;
            let mut row_weight = 0;
            for row in (0..height).rev() {
                match problem[row][col] {
                    'O' => num_rocks += 1,
                    '#' => {
                        let row = height - 1 - row;
                        let below = row - num_rocks;
                        row_weight += (row * (row + 1)) / 2 - (below * (below + 1)) / 2;
                        num_rocks = 0;
                    }
                    _ => (),
                }
            }
            row_weight
        })
        .sum()
}

fn solve2(mut problem: Vec<Vec<char>>) -> usize {
    let cycles_req = 1_000_000_000;
    let mut memory = HashMap::new();
    let mut cycles_taken = 0;
    while !memory.contains_key(&problem) {
        memory.insert(problem.clone(), cycles_taken);

        tilt_north(&mut problem);
        tilt_west(&mut problem);
        tilt_south(&mut problem);
        tilt_east(&mut problem);

        cycles_taken += 1;
    }

    let meta_cycle_repeat_start = memory[&problem];
    let meta_cycle_len = cycles_taken - meta_cycle_repeat_start;
    let required_cycle_pos_within_meta_cycle =
        (cycles_req - meta_cycle_repeat_start) % meta_cycle_len;

    let problem_at_req_cycle = memory
        .iter()
        .find(|(_, cycle)| {
            **cycle == required_cycle_pos_within_meta_cycle + meta_cycle_repeat_start
        })
        .map(|(problem, _)| problem.clone())
        .unwrap();

    let height = problem_at_req_cycle.len();
    let width = problem_at_req_cycle[0].len();
    (0..height)
        .cartesian_product(0..width)
        .map(|(row, col)| match problem_at_req_cycle[row][col] {
            'O' => height - row,
            _ => 0,
        })
        .sum()
}

fn tilt_north(problem: &mut [Vec<char>]) {
    let height = problem.len();
    let width = problem[0].len();
    for col in 0..width {
        let mut insert_into = 0;
        let mut insert_from = 0;
        while insert_from < height {
            if problem[insert_from][col] == 'O' {
                let at_from = problem[insert_from][col];
                let at_into = problem[insert_into][col];
                problem[insert_from][col] = at_into;
                problem[insert_into][col] = at_from;
                insert_from += 1;
                insert_into += 1;
            } else if problem[insert_from][col] == '#' {
                insert_from += 1;
                insert_into = insert_from;
            } else {
                insert_from += 1;
            }
        }
    }
}

fn tilt_east(problem: &mut [Vec<char>]) {
    let width = problem[0].len();
    for p_row in problem {
        let mut insert_into = 0;
        let mut insert_from = 0;
        while insert_from < width {
            if p_row[width - 1 - insert_from] == 'O' {
                let at_from = p_row[width - 1 - insert_from];
                let at_into = p_row[width - 1 - insert_into];
                p_row[width - 1 - insert_from] = at_into;
                p_row[width - 1 - insert_into] = at_from;
                insert_from += 1;
                insert_into += 1;
            } else if p_row[width - 1 - insert_from] == '#' {
                insert_from += 1;
                insert_into = insert_from;
            } else {
                insert_from += 1;
            }
        }
    }
}

fn tilt_south(problem: &mut [Vec<char>]) {
    let height = problem.len();
    let width = problem[0].len();
    for col in 0..width {
        let mut insert_into = 0;
        let mut insert_from = 0;
        while insert_from < height {
            if problem[height - 1 - insert_from][col] == 'O' {
                let at_from = problem[height - 1 - insert_from][col];
                let at_into = problem[height - 1 - insert_into][col];
                problem[height - 1 - insert_from][col] = at_into;
                problem[height - 1 - insert_into][col] = at_from;
                insert_from += 1;
                insert_into += 1;
            } else if problem[height - 1 - insert_from][col] == '#' {
                insert_from += 1;
                insert_into = insert_from;
            } else {
                insert_from += 1;
            }
        }
    }
}

fn tilt_west(problem: &mut [Vec<char>]) {
    let width = problem[0].len();
    for p_row in problem {
        let mut insert_into = 0;
        let mut insert_from = 0;
        while insert_from < width {
            if p_row[insert_from] == 'O' {
                let at_from = p_row[insert_from];
                let at_into = p_row[insert_into];
                p_row[insert_from] = at_into;
                p_row[insert_into] = at_from;
                insert_from += 1;
                insert_into += 1;
            } else if p_row[insert_from] == '#' {
                insert_from += 1;
                insert_into = insert_from;
            } else {
                insert_from += 1;
            }
        }
    }
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
