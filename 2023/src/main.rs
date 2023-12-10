#![feature(iter_array_chunks)]

extern crate core;

use crate::utils::SolverFn;
use clap::Parser;
use core::panic;
use itertools::Itertools;
use std::{env, fs::read_to_string, time};
use utils::TestCase;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;
mod d21;
mod d22;
mod d23;
mod d24;
mod d25;
#[allow(dead_code)]
mod utils;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let cli = Cli::parse();

    let file = match cli.example {
        true => "example.txt",
        false => "input.txt",
    };
    let path = format!("{manifest_dir}/src/d{:02}/{file}", cli.day);
    let input = read_to_string(path).unwrap();

    let test_cases = match cli.example {
        true => utils::parse_example_testcases(&input),
        false => get_real_testcases(cli.day, &input),
    };

    let solver = get_solver(cli.day);

    let mut printable_results = test_cases.iter().map(|test_case| {
        let start = time::Instant::now();
        let res = solver(&test_case.problem);
        let elapsed = start.elapsed();

        let (answer1, res0) = format_to_longest(test_case.answer1, res.0);
        let (answer2, res1) = format_to_longest(test_case.answer2, res.1);
        format!(
            "Elapsed: {} millis\nExpected: | {} | {} |\nActual:   | {} | {} |",
            elapsed.as_millis(),
            answer1,
            answer2,
            res0,
            res1
        )
    });

    println!(
        "{}",
        printable_results.join("\n---------------------------------------\n")
    )
}

fn format_to_longest(a1: u64, a2: u64) -> (String, String) {
    let mut a1 = a1.to_string();
    let mut a2 = a2.to_string();
    let a1_len = a1.len();
    let a2_len = a2.len();

    let shorter = if a1_len < a2_len { &mut a1 } else { &mut a2 };

    for _ in 0..a1_len.abs_diff(a2_len) {
        shorter.insert(0, ' ');
    }

    (a1, a2)
}

fn get_solver(day: u8) -> SolverFn {
    match day {
        1 => d01::solve,
        2 => d02::solve,
        3 => d03::solve,
        4 => d04::solve,
        5 => d05::solve,
        6 => d06::solve,
        7 => d07::solve,
        8 => d08::solve,
        9 => d09::solve,
        10 => d10::solve,
        11 => d11::solve,
        12 => d12::solve,
        13 => d13::solve,
        14 => d14::solve,
        15 => d15::solve,
        16 => d16::solve,
        17 => d17::solve,
        18 => d18::solve,
        19 => d19::solve,
        20 => d20::solve,
        21 => d21::solve,
        22 => d22::solve,
        23 => d23::solve,
        24 => d24::solve,
        25 => d25::solve,
        _ => {
            panic!("pick another day");
        }
    }
}

fn get_real_testcases(day: u8, problem: &str) -> Vec<TestCase> {
    let problem = problem.to_string();
    match day {
        1 => vec![TestCase {
            problem,
            answer1: 54634,
            answer2: 53855,
        }],
        2 => vec![TestCase {
            problem,
            answer1: 2541,
            answer2: 66016,
        }],
        3 => vec![TestCase {
            problem,
            answer1: 528799,
            answer2: 84907174,
        }],
        4 => vec![TestCase {
            problem,
            answer1: 23441,
            answer2: 5923918,
        }],
        5 => vec![TestCase {
            problem,
            answer1: 993500720,
            answer2: 4917124,
        }],
        6 => vec![TestCase {
            problem,
            answer1: 2374848,
            answer2: 39132886,
        }],
        7 => vec![TestCase {
            problem,
            answer1: 248422077,
            answer2: 249817836,
        }],
        8 => vec![TestCase {
            problem,
            answer1: 12361,
            answer2: 18215611419223,
        }],
        9 => vec![TestCase {
            problem,
            answer1: 2005352194,
            answer2: 1077,
        }],
        10 => vec![TestCase {
            problem,
            answer1: 6942,
            answer2: 297,
        }],
        11 => vec![TestCase {
            problem,
            answer1: 0,
            answer2: 0,
        }],
        12 => vec![TestCase {
            problem,
            answer1: 0,
            answer2: 0,
        }],
        13 => vec![TestCase {
            problem,
            answer1: 0,
            answer2: 0,
        }],
        14 => vec![TestCase {
            problem,
            answer1: 0,
            answer2: 0,
        }],
        15 => vec![TestCase {
            problem,
            answer1: 0,
            answer2: 0,
        }],
        16 => vec![TestCase {
            problem,
            answer1: 0,
            answer2: 0,
        }],
        17 => vec![TestCase {
            problem,
            answer1: 0,
            answer2: 0,
        }],
        18 => vec![TestCase {
            problem,
            answer1: 0,
            answer2: 0,
        }],
        19 => vec![TestCase {
            problem,
            answer1: 0,
            answer2: 0,
        }],
        20 => vec![TestCase {
            problem,
            answer1: 0,
            answer2: 0,
        }],
        21 => vec![TestCase {
            problem,
            answer1: 0,
            answer2: 0,
        }],
        22 => vec![TestCase {
            problem,
            answer1: 0,
            answer2: 0,
        }],
        23 => vec![TestCase {
            problem,
            answer1: 0,
            answer2: 0,
        }],
        24 => vec![TestCase {
            problem,
            answer1: 0,
            answer2: 0,
        }],
        25 => vec![TestCase {
            problem,
            answer1: 0,
            answer2: 0,
        }],
        _ => {
            panic!("pick another day");
        }
    }
}

#[derive(Parser)]
struct Cli {
    day: u8,
    #[arg(long)]
    example: bool,
}
