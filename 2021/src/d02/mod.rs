use itertools::Itertools;

pub fn solve(problem: &str) -> (u64, u64) {
    let problem = &problem.lines().collect_vec();
    let directions = problem
        .iter()
        .map(|line| {
            let mut split_line = line.split(' ');
            let dir = split_line.next().unwrap();
            let qty = split_line.next().unwrap().parse::<u64>().unwrap();
            match dir {
                "up" => Movement::Up(qty),
                "down" => Movement::Down(qty),
                "forward" => Movement::Forward(qty),
                _ => panic!("bad direction"),
            }
        })
        .collect_vec();
    (solve1(&directions), solve2(&directions))
}

fn solve1(movements: &[Movement]) -> u64 {
    let mut down = 0;
    let mut forward = 0;
    for m in movements {
        match m {
            Movement::Up(qty) => down -= qty,
            Movement::Down(qty) => down += qty,
            Movement::Forward(qty) => forward += qty,
        }
    }
    down * forward
}

fn solve2(movements: &[Movement]) -> u64 {
    let mut depth = 0;
    let mut forward = 0;
    let mut aim = 0;
    for m in movements {
        match m {
            Movement::Up(qty) => aim -= qty,
            Movement::Down(qty) => aim += qty,
            Movement::Forward(qty) => {
                forward += qty;
                depth += aim * qty;
            }
        }
    }
    depth * forward
}

enum Movement {
    Up(u64),
    Down(u64),
    Forward(u64),
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
