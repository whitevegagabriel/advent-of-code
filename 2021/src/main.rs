use core::panic;
use std::{env, fs::read_to_string};

use clap::Parser;
use itertools::Itertools;
use utils::TestCase;

mod d01;
mod utils;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let cli = Cli::parse();

    let file = match cli.example {
        true => "example.txt",
        false => "input.txt",
    };
    let path = format!("{manifest_dir}/src/d{:02}/{file}", cli.day);
    let input = read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect_vec();

    let test_cases = match cli.example {
        true => utils::parse_example_testcases(&input),
        false => get_real_testcases(cli.day, &input),
    };

    let solver = get_solver(cli.day);

    for test_case in test_cases {
        let res = solver(&test_case.problem);
        println!("Expected: {:?}", (test_case.answer1, test_case.answer2));
        println!("Actual: {res:?}");
    }
}

fn get_solver(day: u8) -> impl Fn(&Vec<String>) -> (u32, u32) {
    match day {
        1 => d01::solve,
        _ => {
            panic!("pick another day");
        }
    }
}

fn get_real_testcases(day: u8, problem: &Vec<String>) -> Vec<TestCase> {
    match day {
        1 => vec![TestCase {
            problem: problem.clone(),
            answer1: 1665,
            answer2: 1702,
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
