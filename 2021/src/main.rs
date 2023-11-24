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
#[allow(dead_code)]
mod d19;
#[allow(dead_code)]
mod d20;
#[allow(dead_code)]
mod d21;
#[allow(dead_code)]
mod d22;
#[allow(dead_code)]
mod d23;
#[allow(dead_code)]
mod d24;
#[allow(dead_code)]
mod d25;
mod utils;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let cli = Cli::parse();
    // let cli = Cli::parse_from(["", "19", "--example"]);

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
            answer1: 1665,
            answer2: 1702,
        }],
        2 => vec![TestCase {
            problem,
            answer1: 1524750,
            answer2: 1592426537,
        }],
        3 => vec![TestCase {
            problem,
            answer1: 738234,
            answer2: 3969126,
        }],
        4 => vec![TestCase {
            problem,
            answer1: 71708,
            answer2: 34726,
        }],
        5 => vec![TestCase {
            problem,
            answer1: 7269,
            answer2: 21140,
        }],
        6 => vec![TestCase {
            problem,
            answer1: 388739,
            answer2: 1741362314973,
        }],
        7 => vec![TestCase {
            problem,
            answer1: 354129,
            answer2: 98905973,
        }],
        8 => vec![TestCase {
            problem,
            answer1: 532,
            answer2: 1011284,
        }],
        9 => vec![TestCase {
            problem,
            answer1: 528,
            answer2: 920448,
        }],
        10 => vec![TestCase {
            problem,
            answer1: 462693,
            answer2: 3094671161,
        }],
        11 => vec![TestCase {
            problem,
            answer1: 1739,
            answer2: 324,
        }],
        12 => vec![TestCase {
            problem,
            answer1: 4749,
            answer2: 123054,
        }],
        13 => vec![TestCase {
            problem,
            answer1: 693,
            answer2: 5989000983004226491,
        }],
        14 => vec![TestCase {
            problem,
            answer1: 3342,
            answer2: 3776553567525,
        }],
        15 => vec![TestCase {
            problem,
            answer1: 745,
            answer2: 3002,
        }],
        16 => vec![TestCase {
            problem,
            answer1: 889,
            answer2: 739303923668,
        }],
        17 => vec![TestCase {
            problem,
            answer1: 10585,
            answer2: 5247,
        }],
        18 => vec![TestCase {
            problem,
            answer1: 3699,
            answer2: 4735,
        }],
        19 => vec![TestCase {
            problem,
            answer1: 323,
            answer2: 10685,
        }],
        20 => vec![TestCase {
            problem,
            answer1: 5573,
            answer2: 20097,
        }],
        21 => vec![TestCase {
            problem,
            answer1: 576600,
            answer2: 131888061854776,
        }],
        22 => vec![TestCase {
            problem,
            answer1: 576028,
            answer2: 1387966280636636,
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
