use itertools::Itertools;
use num::cast::AsPrimitive;
use std::collections::HashMap;

pub type SolverFn = fn(&str) -> (usize, usize);

#[allow(dead_code)]
pub fn basic_test(input: &str, test: SolverFn) {
    let examples = parse_example_testcases(input);
    for (idx, example) in examples.iter().enumerate() {
        println!("Example {}", idx + 1);
        let (answer1, answer2) = test(&example.problem);
        assert_eq!(example.answer1, answer1);
        assert_eq!(example.answer2, answer2);
    }
}

pub fn median_round_down(input: Vec<usize>) -> usize {
    let mut input = input.to_vec();
    input.sort();
    input[input.len() / 2]
}

pub fn parse_numbers(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect_vec()
}

pub fn parse_all_numbers(input: &[&str]) -> Vec<Vec<usize>> {
    input.iter().map(|l| parse_numbers(l)).collect()
}

pub fn parse_example_testcases(input: &str) -> Vec<TestCase> {
    // find indices of test case boundaries
    let indices_vec = input
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            if line.starts_with("-------------------------------") {
                Some(idx)
            } else {
                None
            }
        })
        .collect_vec();

    // assumes the format "answer_x: i"
    fn parse_usize_from_string(input: &str) -> usize {
        let maybe_num = input.split(':').last().unwrap().trim();

        if maybe_num == "-" {
            return 0;
        }
        maybe_num.parse().unwrap()
    }

    indices_vec
        .chunks(3)
        .map(|indices| {
            let (start, middle) = (indices[0], indices[1]);
            let problem = input
                .split('\n')
                .skip(start + 1)
                .take(middle - start - 1)
                .collect_vec();

            let answer1: usize = input
                .split('\n')
                .nth(middle + 1)
                .map(parse_usize_from_string)
                .unwrap();
            let answer2: usize = input
                .split('\n')
                .nth(middle + 2)
                .map(parse_usize_from_string)
                .unwrap();

            TestCase {
                problem: problem.join("\n"),
                answer1,
                answer2,
            }
        })
        .collect_vec()
}

pub fn transposed<T: Clone>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| matrix[row][col].clone()).collect_vec())
        .collect_vec()
}

#[derive(Debug, PartialEq)]
pub struct TestCase {
    pub problem: String,
    pub answer1: usize,
    pub answer2: usize,
}

#[test]
fn parses_single_test_case() {
    let input = r#"
some text
-------------------------------
1
2
3
-------------------------------
hello: 1
world: 2
-------------------------------
"#;
    let test_cases = parse_example_testcases(input);

    assert_eq!(
        vec![TestCase {
            problem: "1\n2\n3".into(),
            answer1: 1,
            answer2: 2,
        }],
        test_cases
    );
}

#[test]
fn parses_two_test_cases() {
    let input = r#"
some text
-------------------------------
1
2
3
-------------------------------
hello: 1
world: 2
-------------------------------
more text

-------------------------------
greetings...
yo
sup

-------------------------------
hello: -
world: 100
-------------------------------
"#;
    let test_cases = parse_example_testcases(input);

    assert_eq!(
        vec![
            TestCase {
                problem: "1\n2\n3".into(),
                answer1: 1,
                answer2: 2,
            },
            TestCase {
                problem: "greetings...\nyo\nsup\n".into(),
                answer1: 0,
                answer2: 100,
            }
        ],
        test_cases
    );
}

#[test]
fn transpose_valid_matrix() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let matrix_t = transposed(&matrix);
    assert_eq!(vec![vec![1, 4], vec![2, 5], vec![3, 6]], matrix_t)
}

#[test]
fn even_median() {
    let input = vec![1, 2, 3, 0];
    let median = median_round_down(input);
    assert_eq!(2, median);
}

#[test]
fn odd_median() {
    let input = vec![1, 5, 2, 3, 0];
    let median = median_round_down(input);
    assert_eq!(2, median);
}

pub fn get_cross_neighbors<T: AsPrimitive<isize> + TryFrom<isize>>(point: &(T, T)) -> Vec<(T, T)> {
    let point = (point.0.as_(), point.1.as_());
    [-1, 0, 1, 0]
        .into_iter()
        .zip([0, -1, 0, 1])
        .filter_map(|diff| maybe_add(&point, &diff))
        .collect_vec()
}

pub fn get_square_neighbors<T: AsPrimitive<isize> + TryFrom<isize>>(point: &(T, T)) -> Vec<(T, T)> {
    let point: (isize, isize) = (point.0.as_(), point.1.as_());
    [-1, 0, 1, -1, 1, -1, 0, 1]
        .into_iter()
        .zip([-1, -1, -1, 0, 0, 1, 1, 1])
        .filter_map(|diff| maybe_add(&point, &diff))
        .collect_vec()
}

fn maybe_add<T: AsPrimitive<isize> + TryFrom<isize>>(
    p1: &(isize, isize),
    p2: &(isize, isize),
) -> Option<(T, T)> {
    let row_new = T::try_from(p1.0 + p2.0);
    let col_new = T::try_from(p1.1 + p2.1);
    if let (Ok(row_new), Ok(col_new)) = (row_new, col_new) {
        Some((row_new, col_new))
    } else {
        None
    }
}

#[test]
fn test_cross_neighbors() {
    let point = (0_usize, 0);
    let mut neighbors = get_cross_neighbors(&point);
    neighbors.sort();
    assert_eq!(vec![(0, 1), (1, 0)], neighbors);

    let point = (1, 1);
    let mut neighbors = get_cross_neighbors(&point);
    neighbors.sort();
    assert_eq!(vec![(0, 1), (1, 0), (1, 2), (2, 1)], neighbors);
}

#[test]
fn test_square_neighbors() {
    let point = (0_usize, 0);
    let mut neighbors = get_square_neighbors(&point);
    neighbors.sort();
    assert_eq!(vec![(0, 1), (1, 0), (1, 1)], neighbors);

    let point = (1, 1);
    let mut neighbors = get_square_neighbors(&point);
    neighbors.sort();
    assert_eq!(
        vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2)
        ],
        neighbors
    );
}

pub fn parse_matrix_of_nums(input: &[&str]) -> HashMap<(usize, usize), usize> {
    parse_and_map_matrix_of_nums(input, |n| n)
}

pub fn parse_and_map_matrix_of_nums<T>(
    input: &[&str],
    mapper: impl Fn(usize) -> T,
) -> HashMap<(usize, usize), T> {
    let num_rows = input.len();
    let num_cols = input[0].len();
    (0..num_rows)
        .cartesian_product(0..num_cols)
        .map(|pos| {
            let num = input[pos.0]
                .chars()
                .nth(pos.1)
                .unwrap()
                .to_digit(10)
                .unwrap() as usize;
            (pos, mapper(num))
        })
        .collect()
}

#[test]
fn test_parse_map_nums() {
    let input = ["12345", "74195", "44668"];
    let actual = parse_matrix_of_nums(&input);
    let expected: HashMap<_, _> = [
        ((0, 0), 1),
        ((0, 1), 2),
        ((0, 2), 3),
        ((0, 3), 4),
        ((0, 4), 5),
        ((1, 0), 7),
        ((1, 1), 4),
        ((1, 2), 1),
        ((1, 3), 9),
        ((1, 4), 5),
        ((2, 0), 4),
        ((2, 1), 4),
        ((2, 2), 6),
        ((2, 3), 6),
        ((2, 4), 8),
    ]
    .into();
    assert_eq!(expected, actual)
}

pub fn manhattan_distance(pos1: &(usize, usize), pos2: &(usize, usize)) -> usize {
    pos1.0.abs_diff(pos2.0) + pos1.1.abs_diff(pos2.1)
}

#[test]
fn test_cartesian_distance() {
    let pos1 = (0, 0);
    let pos2 = (1, 1);

    assert_eq!(2, manhattan_distance(&pos1, &pos2));

    let pos1 = (4, 3);
    let pos2 = (10, 1);

    assert_eq!(8, manhattan_distance(&pos1, &pos2));
}
