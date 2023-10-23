use itertools::Itertools;

pub fn solve(problem: &[&str]) -> (u64, u64) {
    let lines = problem.iter().map(|s| Line::parse(s)).collect_vec();
    (solve1(&lines), solve2(&lines))
}

fn solve1(lines: &[Line]) -> u64 {
    count_overlap(lines, |l| !l.is_diagonal())
}

fn solve2(lines: &[Line]) -> u64 {
    count_overlap(lines, |_| true)
}

fn count_overlap(lines: &[Line], filter: impl Fn(&&Line) -> bool) -> u64 {
    let coords = lines
        .iter()
        .filter(filter)
        .flat_map(|l| l.points.clone())
        .counts();
    let n_overlap = coords.iter().filter(|c| c.1 >= &2).count();
    u64::try_from(n_overlap).unwrap()
}

#[derive(Debug, PartialEq)]
struct Line {
    start: (u64, u64),
    end: (u64, u64),
    points: Vec<(u64, u64)>,
}

impl Line {
    fn parse(input: &str) -> Self {
        let (start, end) = input
            .split(" -> ")
            .map(Self::parse_point)
            .collect_tuple()
            .unwrap();

        let x =  if start.0 > end.0 {
            (end.0..=start.0).rev().collect_vec() // this is wrong for certain diagonals
        } else {
            (start.0..=end.0).collect_vec()
        };

        let y = if start.1 > end.1 {
            (end.1..=start.1).rev().collect_vec()
        } else {
            (start.1..=end.1).collect_vec()
        };

        let points = if x.len() < y.len() {
            x.into_iter().cycle().zip(y).collect_vec()
        } else {
            x.into_iter().zip(y.into_iter().cycle()).collect_vec()
        };

        Self { start, end, points }
    }

    fn parse_point(input: &str) -> (u64, u64) {
        input
            .split(',')
            .map(|n| n.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap()
    }

    fn is_diagonal(&self) -> bool {
        self.start.0 != self.end.0 && self.start.1 != self.end.1
    }
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}

#[test]
fn parse_single_point_line() {
    let input = "0,0 -> 0,0";
    let line = Line::parse(input);
    assert_eq!(
        Line {
            start: (0, 0),
            end: (0, 0),
            points: vec![(0, 0)],
        },
        line
    );
}

#[test]
fn parse_horizontal_line() {
    let input = "5,0 -> 0,0";
    let line = Line::parse(input);
    assert_eq!(
        Line {
            start: (5, 0),
            end: (0, 0),
            points: vec![(5, 0), (4, 0), (3, 0), (2, 0), (1, 0), (0, 0)],
        },
        line
    );
}
