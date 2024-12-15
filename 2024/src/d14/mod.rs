use crate::common::{test_with_params, visualize_points, AsPoint2, Point2, Vector2};
use average::Variance;
use itertools::Itertools;
use regex::Regex;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test_with_params("example", MODULE, p1, (7, 11), 12);
}

#[test]
fn p1_input() {
    test_with_params("input", MODULE, p1, (103, 101), 229980828);
}

#[test]
fn p2_input() {
    test_with_params("input", MODULE, p2, (103, 101), 7132);
}

fn p1(input: &str, dimensions: (isize, isize)) -> usize {
    let (height, width) = dimensions;
    let mut robots = parse_input(input, width, height);

    for robot in &mut robots {
        robot.step(100);
    }

    let quadrant_counts = robots
        .iter()
        .filter_map(|robot| {
            let shifted_x = robot.position.x - width / 2;
            let shifted_y = robot.position.y - height / 2;

            if shifted_x == 0 || shifted_y == 0 {
                None
            } else if shifted_x > 0 && shifted_y > 0 {
                Some(1)
            } else if shifted_x < 0 && shifted_y > 0 {
                Some(2)
            } else if shifted_x < 0 && shifted_y < 0 {
                Some(3)
            } else {
                Some(4)
            }
        })
        .counts();

    quadrant_counts.values().product()
}

fn p2(input: &str, dimensions: (isize, isize)) -> usize {
    let (height, width) = dimensions;
    let mut robots = parse_input(input, width, height);

    let mut min_var_x = (f64::MAX, 0);
    let mut min_var_y = (f64::MAX, 0);

    for i in 0..width.max(height) {
        let var_x = robots
            .iter()
            .map(|robot| robot.position.x as f64)
            .collect::<Variance>()
            .population_variance();
        let var_y = robots
            .iter()
            .map(|robot| robot.position.y as f64)
            .collect::<Variance>()
            .population_variance();

        if var_x < min_var_x.0 {
            min_var_x = (var_x, i);
        }

        if var_y < min_var_y.0 {
            min_var_y = (var_y, i);
        }

        for robot in &mut robots {
            robot.step(1);
        }
    }

    // patterns repeat every {width} in the x, and every {height} in the y
    // when the pattern in the x with minimum variance overlaps with the pattern in the y, then
    // we have likely found the Christmas tree, which is more clustered than a rand dist
    // the state of the graph repeats every width * height
    let repetition_length = width * height;
    let min_var_x_idx = min_var_x.1;
    let min_var_y_idx = min_var_y.1;
    let steps = (0..repetition_length / width)
        .cartesian_product(0..repetition_length / height)
        .find_map(|(x_repetition, y_repetition)| {
            if min_var_x_idx + width * x_repetition == min_var_y_idx + height * y_repetition {
                Some(min_var_x_idx + width * x_repetition)
            } else {
                None
            }
        })
        .unwrap();

    let mut robots = parse_input(input, width, height);
    for robot in &mut robots {
        robot.step(steps);
    }
    let visualized = visualize_points(&robots, width as usize, height as usize);
    println!("{visualized}");

    steps as usize
}

fn parse_input(input: &str, x_bound: isize, y_bound: isize) -> Vec<Robot> {
    let pattern = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    pattern
        .captures_iter(input)
        .map(|captures| {
            let (_, [px, py, vx, vy]) = captures.extract();
            Robot {
                position: Point2 {
                    x: px.parse().unwrap(),
                    y: py.parse().unwrap(),
                },
                velocity: Vector2 {
                    x: vx.parse().unwrap(),
                    y: vy.parse().unwrap(),
                },
                x_bound,
                y_bound,
            }
        })
        .collect()
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
struct Robot {
    position: Point2<isize>,
    velocity: Vector2<isize>,
    x_bound: isize,
    y_bound: isize,
}

impl Robot {
    fn step(&mut self, times: isize) {
        self.position.x = (self.position.x + self.velocity.x * times) % self.x_bound;
        if self.position.x < 0 {
            self.position.x += self.x_bound;
        }
        self.position.y = (self.position.y + self.velocity.y * times) % self.y_bound;
        if self.position.y < 0 {
            self.position.y += self.y_bound;
        }
    }
}

impl AsPoint2<'_, isize> for Robot {
    fn as_point2(&self) -> &Point2<isize> {
        &self.position
    }
}
