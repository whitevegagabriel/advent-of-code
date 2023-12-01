use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_until},
    combinator::map,
    multi::many1,
    sequence::{terminated, tuple},
    IResult,
};
use std::collections::HashMap;

pub fn solve(problem: &str) -> (u64, u64) {
    let image = Image::parse(problem).unwrap().1;
    (solve1(image.clone()), solve2(image))
}

fn solve1(mut image: Image) -> u64 {
    for _ in 0..2 {
        image.enhance()
    }
    image.num_lit_pixels() as u64
}

fn solve2(mut image: Image) -> u64 {
    for _ in 0..50 {
        image.enhance()
    }
    image.num_lit_pixels() as u64
}

#[derive(Debug, Clone)]
struct Canvas {
    canvas: HashMap<(i32, i32), u8>,
    min_row: i32,
    max_row: i32,
    min_col: i32,
    max_col: i32,
}

impl Canvas {
    fn new(canvas: HashMap<(i32, i32), u8>) -> Self {
        let (min_row, max_row, min_col, max_col) = canvas.iter().fold(
            (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
            |(min_row, max_row, min_col, max_col), ((row, col), _)| {
                (
                    min_row.min(*row),
                    max_row.max(*row),
                    min_col.min(*col),
                    max_col.max(*col),
                )
            },
        );
        Canvas {
            canvas,
            min_row,
            max_row,
            min_col,
            max_col,
        }
    }

    fn insert(&mut self, key: (i32, i32), value: u8) {
        self.canvas.insert(key, value);
        self.min_row = self.min_row.min(key.0);
        self.max_row = self.max_row.max(key.0);
        self.min_col = self.min_col.min(key.1);
        self.max_col = self.max_col.max(key.1);
    }

    fn get_or_default(&self, key: (i32, i32), default: u8) -> u8 {
        *self.canvas.get(&key).unwrap_or(&default)
    }

    fn count_with_value(&self, value: u8) -> usize {
        self.canvas.values().filter(|u| u == &&value).count()
    }
}

#[derive(Debug, Clone)]
struct DefaultPixelGenerator {
    even: bool,
    even_value: u8,
    odd_value: u8,
}

impl DefaultPixelGenerator {
    fn rotate(&mut self) {
        self.even ^= true
    }

    fn default_pixel(&self) -> u8 {
        if self.even {
            self.even_value
        } else {
            self.odd_value
        }
    }
}

#[derive(Debug, Clone)]
struct Image {
    algorithm: Vec<u8>,
    canvases: [Canvas; 2],
    template_idx: usize,
    default_pixel_generator: DefaultPixelGenerator,
}

impl Image {
    fn new(algorithm: Vec<u8>, canvas: Canvas) -> Self {
        let generator = DefaultPixelGenerator {
            even: true,
            even_value: 0,
            odd_value: if algorithm[0] == 1 { 1 } else { 0 },
        };
        Self {
            algorithm,
            canvases: [canvas.clone(), canvas],
            template_idx: 0,
            default_pixel_generator: generator,
        }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let str_to_bools = |s: &str| -> Vec<u8> {
            s.chars()
                .map(|c| match c {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!("erm"),
                })
                .collect_vec()
        };
        let parse_line_of_bools = map(terminated(take_until("\n"), many1(tag("\n"))), str_to_bools);
        let parse_line_of_bools2 =
            map(terminated(take_until("\n"), many1(tag("\n"))), str_to_bools);

        map(
            tuple((parse_line_of_bools, many1(parse_line_of_bools2))),
            |(algorithm, image_vec)| {
                let mut image = HashMap::new();
                for (row, col) in (0..image_vec.len()).cartesian_product(0..image_vec[0].len()) {
                    image.insert((row as i32, col as i32), image_vec[row][col]);
                }
                Self::new(algorithm, Canvas::new(image))
            },
        )(input)
    }

    fn enhance(&mut self) {
        let template = &self.canvases[self.template_idx];
        let (min_row, max_row, min_col, max_col) = (
            template.min_row,
            template.max_row,
            template.min_col,
            template.max_col,
        );

        for (row, col) in (min_row - 1..=max_row + 1).cartesian_product(min_col - 1..=max_col + 1) {
            let algo_idx = (row - 1..=row + 1)
                .cartesian_product(col - 1..=col + 1)
                .map(|point| {
                    self.canvases[self.template_idx]
                        .get_or_default(point, self.default_pixel_generator.default_pixel())
                })
                .fold(0_usize, |accum, bit| (accum << 1) | (bit as usize));

            self.canvases[(self.template_idx + 1) % 2].insert((row, col), self.algorithm[algo_idx]);
        }

        self.template_idx = (self.template_idx + 1) % 2;
        self.default_pixel_generator.rotate()
    }

    fn num_lit_pixels(&self) -> usize {
        let prev_buffer = &self.canvases[self.template_idx];
        prev_buffer.count_with_value(1)
    }
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
