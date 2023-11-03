use core::panic;
use std::{env, fs::read_to_string};

use crate::utils::SolverFn;
use clap::Parser;
use itertools::Itertools;
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
        let res = solver(&test_case.problem);
        format!(
            "Expected: |{:^13}|{:^13}|\nActual:   |{:^13}|{:^13}|",
            test_case.answer1, test_case.answer2, res.0, res.1
        )
    });

    println!("{}", printable_results.join("\n---------------------------------------\n"))
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
        _ => {
            panic!("pick another day");
        }
    }
}

fn get_real_testcases(day: u8, problem: &str) -> Vec<TestCase> {
    match day {
        1 => vec![TestCase {
            problem: problem.lines().collect_vec(),
            answer1: 1665,
            answer2: 1702,
        }],
        2 => vec![TestCase {
            problem: problem.lines().collect_vec(),
            answer1: 1524750,
            answer2: 1592426537,
        }],
        3 => vec![TestCase {
            problem: problem.lines().collect_vec(),
            answer1: 738234,
            answer2: 3969126,
        }],
        4 => vec![TestCase {
            problem: problem.lines().collect_vec(),
            answer1: 71708,
            answer2: 34726,
        }],
        5 => vec![TestCase {
            problem: problem.lines().collect_vec(),
            answer1: 7269,
            answer2: 21140,
        }],
        6 => vec![TestCase {
            problem: problem.lines().collect_vec(),
            answer1: 388739,
            answer2: 1741362314973,
        }],
        7 => vec![TestCase {
            problem: problem.lines().collect_vec(),
            answer1: 354129,
            answer2: 98905973,
        }],
        8 => vec![TestCase {
            problem: problem.lines().collect_vec(),
            answer1: 532,
            answer2: 1011284,
        }],
        9 => vec![TestCase {
            problem: problem.lines().collect_vec(),
            answer1: 528,
            answer2: 920448,
        }],
        10 => vec![TestCase {
            problem: problem.lines().collect_vec(),
            answer1: 462693,
            answer2: 3094671161,
        }],
        11 => vec![TestCase {
            problem: problem.lines().collect_vec(),
            answer1: 1739,
            answer2: 324,
        }],
        12 => vec![TestCase {
            problem: problem.lines().collect_vec(),
            answer1: 4749,
            answer2: 123054,
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
