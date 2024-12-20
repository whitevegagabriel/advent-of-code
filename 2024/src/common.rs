use crate::common::Direction::{Down, Left, Right, Up};
use itertools::Itertools;
use num::{integer::gcd, Integer};
use num_traits::Num;
use std::{
    cmp::Eq,
    collections::HashMap,
    env,
    fmt::Debug,
    fs::read_to_string,
    hash::Hash,
    ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign},
    time,
};

pub fn test<T: Debug + Eq, F: Fn(&str) -> T>(
    file_name: &str,
    module_path: &str,
    f: F,
    expected: T,
) {
    test_with_params(file_name, module_path, |s: &str, _: ()| f(s), (), expected);
}

pub fn test_with_params<P, T: Debug + Eq, F: Fn(&str, P) -> T>(
    file_name: &str,
    module_path: &str,
    f: F,
    params: P,
    expected: T,
) {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let module_name = module_path.split("::").last().unwrap();
    let input_file = format!("{manifest_dir}/src/{module_name}/{file_name}.txt");
    let start = time::Instant::now();
    {
        let input = read_to_string(input_file).unwrap();
        let actual = f(&input, params);

        assert_eq!(expected, actual);
    }
    let elapsed = start.elapsed();

    let (time, units) = if elapsed.as_secs() >= 1 {
        (elapsed.as_millis(), "ms")
    } else {
        (elapsed.as_micros(), "μs")
    };

    println!("Elapsed: {time} {units}");
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Point2<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T: Num + Copy> Add<Vector2<T>> for Point2<T> {
    type Output = Point2<T>;
    fn add(self, rhs: Vector2<T>) -> Self::Output {
        Point2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Num + Copy> AddAssign<Vector2<T>> for Point2<T> {
    fn add_assign(&mut self, rhs: Vector2<T>) {
        *self = *self + rhs;
    }
}

impl<T: Num + Sub<Output = T>> Sub<Vector2<T>> for Point2<T> {
    type Output = Self;

    fn sub(self, rhs: Vector2<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Num + Copy> SubAssign<Vector2<T>> for Point2<T> {
    fn sub_assign(&mut self, rhs: Vector2<T>) {
        *self = *self - rhs;
    }
}

impl<T: Num + Sub<Output = T>> Sub<Point2<T>> for Point2<T> {
    type Output = Vector2<T>;

    fn sub(self, rhs: Point2<T>) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Vector2<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T: Copy + Neg<Output = T> + Integer> Vector2<T> {
    pub fn rotate_90(&mut self, rotation_direction: RotationDirection) {
        // note that positive is down and right
        match rotation_direction {
            RotationDirection::Counterclockwise => {
                let x_prev = self.x;
                self.x = self.y;
                self.y = -x_prev;
            }
            RotationDirection::Clockwise => {
                let x_prev = self.x;
                self.x = -self.y;
                self.y = x_prev;
            }
        }
    }

    pub fn rotated_90(&self, rotation_direction: RotationDirection) -> Self {
        let mut new_self = *self;
        new_self.rotate_90(rotation_direction);
        new_self
    }

    pub fn simplify(&mut self) {
        let divisor = gcd(self.x, self.y);
        self.x = self.x / divisor;
        self.y = self.y / divisor;
    }
}

impl<T: Neg<Output = T>> Neg for Vector2<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Num + Copy> Mul<T> for Vector2<T> {
    type Output = Self;
    fn mul(self, num: T) -> Self::Output {
        Self {
            x: self.x * num,
            y: self.y * num,
        }
    }
}

pub enum RotationDirection {
    Clockwise,
    Counterclockwise,
}

pub fn parse_to_usize_map<T: TryFrom<usize> + Eq + Hash>(input: &str) -> HashMap<Point2<T>, usize> {
    parse_to_map(input, |c| c as usize - '0' as usize)
}

pub fn parse_to_char_map<T: TryFrom<usize> + Eq + Hash>(input: &str) -> HashMap<Point2<T>, char> {
    parse_to_map(input, |c| c)
}

fn parse_to_map<T: TryFrom<usize> + Eq + Hash, V, F: Fn(char) -> V>(
    input: &str,
    mapper: F,
) -> HashMap<Point2<T>, V> {
    input
        .lines()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(c_idx, c)| {
                    (
                        Point2 {
                            x: c_idx.try_into().ok().unwrap(),
                            y: line_idx.try_into().ok().unwrap(),
                        },
                        mapper(c),
                    )
                })
                .collect_vec()
        })
        .collect()
}

pub fn get_cross_neighbors<T: Integer + Neg<Output = T> + Copy>(curr: Point2<T>) -> Vec<Point2<T>> {
    [
        Vector2 {
            x: -T::one(),
            y: T::zero(),
        },
        Vector2 {
            x: T::one(),
            y: T::zero(),
        },
        Vector2 {
            x: T::zero(),
            y: -T::one(),
        },
        Vector2 {
            x: T::zero(),
            y: T::one(),
        },
    ]
    .iter()
    .map(|dir| curr + *dir)
    .collect()
}

pub fn get_cross_neighbors_with_direction<T: Integer + Neg<Output = T> + Copy>(
    curr: Point2<T>,
) -> Vec<(Point2<T>, Direction)> {
    [
        (
            Vector2 {
                x: -T::one(),
                y: T::zero(),
            },
            Left,
        ),
        (
            Vector2 {
                x: T::one(),
                y: T::zero(),
            },
            Right,
        ),
        (
            Vector2 {
                x: T::zero(),
                y: -T::one(),
            },
            Down,
        ),
        (
            Vector2 {
                x: T::zero(),
                y: T::one(),
            },
            Up,
        ),
    ]
    .into_iter()
    .map(|(diff, dir)| (curr + diff, dir))
    .collect()
}

pub fn count_digits(num: usize) -> usize {
    (num.checked_ilog10().unwrap_or(0) + 1) as usize
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub trait AsPoint2<'a, T> {
    fn as_point2(&'a self) -> &'a Point2<T>;
}

pub fn visualize_points<'a, T: Copy + TryInto<usize> + 'a, P: AsPoint2<'a, T>>(
    points: &'a [P],
    width: usize,
    height: usize,
) -> String {
    let mut canvas = vec![vec![' '; width]; height];
    for point in points {
        let y_idx = point.as_point2().y.try_into().ok().unwrap();
        let x_idx = point.as_point2().x.try_into().ok().unwrap();
        canvas[y_idx][x_idx] = '*';
    }
    canvas.iter().map(|chars| chars.iter().join("")).join("\n")
}

pub fn visualize<T: Num + Copy + Ord + TryInto<usize>>(
    points: &HashMap<Point2<T>, char>,
) -> String {
    let x_points = points.keys().map(|point| point.x).collect_vec();
    let min_x = (*x_points.iter().min().unwrap()).try_into().ok().unwrap();
    let max_x = (*x_points.iter().max().unwrap()).try_into().ok().unwrap();

    let y_points = points.keys().map(|point| point.y).collect_vec();
    let min_y = (*y_points.iter().min().unwrap()).try_into().ok().unwrap();
    let max_y = (*y_points.iter().max().unwrap()).try_into().ok().unwrap();

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    let mut canvas = vec![vec![' '; width]; height];

    for (point, c) in points {
        let y_idx = point.y.try_into().ok().unwrap() - min_x;
        let x_idx = point.x.try_into().ok().unwrap() - min_y;
        canvas[y_idx][x_idx] = *c;
    }

    canvas.iter().map(|chars| chars.iter().join("")).join("\n")
}

pub fn manhattan_dist<T: Num + Ord + Copy>(p1: &Point2<T>, p2: &Point2<T>) -> T {
    let dist_x = if p1.x > p2.x {
        p1.x - p2.x
    } else {
        p2.x - p1.x
    };

    let dist_y = if p1.y > p2.y {
        p1.y - p2.y
    } else {
        p2.y - p1.y
    };

    dist_x + dist_y
}
