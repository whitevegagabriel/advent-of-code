use crate::common::{parse_lines_to_tuples, test};
use itertools::Itertools;
use std::collections::BinaryHeap;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 50);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 4715966250);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 24);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 1530527040);
}

fn p1(input: &str) -> usize {
    parse_lines_to_tuples(input, ',', |s| s.parse::<usize>().unwrap())
        .into_iter()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1))
        .max()
        .unwrap()
}

fn p2(input: &str) -> usize {
    let points = parse_lines_to_tuples(input, ',', |s| s.parse::<usize>().unwrap());
    let area_points = points
        .iter()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| {
            (
                (x1.abs_diff(*x2) + 1) * (y1.abs_diff(*y2) + 1),
                (*x1, *y1),
                (*x2, *y2),
            )
        })
        .collect_vec();
    let area_points_bh = BinaryHeap::from(area_points);
    let line_segments = points
        .iter()
        .cloned()
        .zip(points.iter().cycle().skip(1).cloned())
        .sorted_by(|(p1, p2), (p3, p4)| length(*p1, *p2).cmp(&length(*p3, *p4)))
        .rev()
        .collect_vec();

    area_points_bh.into_iter_sorted().find(|(area, (x1, y1), (x2, y2))| {
        !line_segments.iter().any(|((x3, y3), (x4, y4))| {
            let (start_1_2, end_1_2, start_3_4, end_3_4) = (x1.min(x2), x1.max(x2), x3.min(x4), x3.max(x4));
            let (bottom_1_2, top_1_2, bottom_3_4, top_3_4) = (y1.min(y2), y1.max(y2), y3.min(y4), y3.max(y4));

            // point 3 is in the square
            x3 > start_1_2 && x3 < end_1_2 && y3 > bottom_1_2 && y3 < top_1_2 ||
            // point 4 is in the square
            x4 > start_1_2 && x4 < end_1_2 && y4 > bottom_1_2 && y4 < top_1_2 ||
            // line 3-4 horizonally intersects the square
            y3 == y4 && y3 > bottom_1_2 && y3 < top_1_2 && start_3_4 <= start_1_2 && end_3_4 >= end_1_2 ||
            // line 3-4 vertically intersects the square
            x3 == x4 && x3 > start_1_2 && x3 < end_1_2 && bottom_3_4 <= bottom_1_2 && top_3_4 >= top_1_2
        })
    }).unwrap().0
}

fn length(p1: (usize, usize), p2: (usize, usize)) -> usize {
    if p1.0 == p2.0 {
        p1.1.abs_diff(p2.1) + 1
    } else {
        p1.0.abs_diff(p2.0) + 1
    }
}
