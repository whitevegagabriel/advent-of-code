use crate::d18::Dir::{D, L, R, U};
use itertools::Itertools;

pub fn solve(problem: &str) -> (usize, usize) {
    let problem1 = problem
        .lines()
        .map(|line| {
            let (dir, qty, _) = line.split_whitespace().collect_tuple().unwrap();
            let qty = qty.parse().unwrap();
            let dir = match dir {
                "L" => L,
                "R" => R,
                "U" => U,
                "D" => D,
                _ => unreachable!(),
            };
            dir(qty)
        })
        .collect_vec();

    let problem2 = problem
        .lines()
        .map(|line| {
            let (_, _, h) = line.split_whitespace().collect_tuple().unwrap();
            let mut h = h.to_string();
            h.insert(2, '0');

            let qty = hex::decode(&h[2..8])
                .unwrap()
                .iter()
                .fold(0_isize, |acc, u| (acc << 8) + *u as isize);
            let dir = match h.as_bytes()[8] - b'0' {
                0 => R,
                1 => D,
                2 => L,
                3 => U,
                _ => unreachable!(),
            };

            dir(qty)
        })
        .collect_vec();

    (solve1(&problem1), solve2(&problem2))
}

fn solve1(directions: &[Dir]) -> usize {
    enclosed_area(directions)
}

fn solve2(directions: &[Dir]) -> usize {
    enclosed_area(directions)
}

fn enclosed_area(directions: &[Dir]) -> usize {
    let vertices = directions
        .iter()
        // convert directions into vertices
        .scan((0, 0), |prev, dir| {
            let (prev_row, prev_col) = *prev;
            let next = match *dir {
                L(qty) => (prev_row, prev_col - qty),
                R(qty) => (prev_row, prev_col + qty),
                U(qty) => (prev_row - qty, prev_col),
                D(qty) => (prev_row + qty, prev_col),
            };
            *prev = next;
            Some(next)
        })
        .collect_vec();

    // at this point, I may as well have used Pick's theorem, but I didn't know about it when
    // writing my original solution
    let mapping = if vertices
        .iter()
        .circular_tuple_windows()
        .map(|((y1, x1), (y2, x2))| (x1 * y2) - (y1 * x2))
        .sum::<isize>()
        < 0
    {
        anticlockwise_mapping
    } else {
        clockwise_mapping
    };

    let vertices = vertices
        .iter()
        // if we treat the above vertices as the centers of squares, then we must calculate the true
        // outer vertices
        .zip(directions.iter().circular_tuple_windows())
        .map(|((r, c), (d1, d2))| {
            let (r_mod, c_mod) = mapping(d1, d2);
            (r + r_mod, c + c_mod)
        })
        .collect_vec();

    vertices
        .iter()
        .circular_tuple_windows()
        // Huzzah for the the shoelace formula!
        .map(|((y1, x1), (y2, x2))| (x1 * y2) - (y1 * x2))
        .sum::<isize>()
        .unsigned_abs()
        / 2
}

fn clockwise_mapping(d1: &Dir, d2: &Dir) -> (isize, isize) {
    match (d1, d2) {
        (R(_), D(_)) | (D(_), R(_)) => (0, 1),
        (D(_), L(_)) | (L(_), D(_)) => (1, 1),
        (L(_), U(_)) | (U(_), L(_)) => (1, 0),
        (U(_), R(_)) | (R(_), U(_)) => (0, 0),
        _ => unreachable!(),
    }
}

fn anticlockwise_mapping(d1: &Dir, d2: &Dir) -> (isize, isize) {
    match (d1, d2) {
        (R(_), D(_)) | (D(_), R(_)) => (1, 0),
        (D(_), L(_)) | (L(_), D(_)) => (0, 0),
        (L(_), U(_)) | (U(_), L(_)) => (0, 1),
        (U(_), R(_)) | (R(_), U(_)) => (1, 1),
        _ => unreachable!(),
    }
}

#[allow(dead_code)]
fn display(points: &[(isize, isize)]) {
    let rows = points.iter().map(|p| p.0).collect_vec();
    let cols = points.iter().map(|p| p.1).collect_vec();
    let min_row = rows.iter().min().unwrap();
    let max_row = rows.iter().max().unwrap();
    let min_col = cols.iter().min().unwrap();
    let max_col = cols.iter().max().unwrap();

    let mut buf =
        vec![vec!['.'; (max_col - min_col + 1) as usize]; (max_row - min_row + 1) as usize];
    for (row, col) in points.iter().map(|(r, c)| (r - min_row, c - min_col)) {
        buf[row as usize][col as usize] = '#';
    }

    let graph = buf
        .into_iter()
        .map(|line| line.iter().collect::<String>())
        .join("\n");
    println!("{graph}");
}

enum Dir {
    L(isize),
    R(isize),
    U(isize),
    D(isize),
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
