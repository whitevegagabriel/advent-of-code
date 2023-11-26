use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

pub fn solve(problem: &str) -> (u64, u64) {
    let (_, cucumber_state) = CucumberState::parse(problem).unwrap();
    (solve1(cucumber_state.clone()), solve2())
}

fn solve1(mut cucumber_state: CucumberState) -> u64 {
    let mut cucumbers_moved_last_iteration = true;

    let mut time_steps = 0;
    while cucumbers_moved_last_iteration {
        cucumber_state.time_step();
        cucumbers_moved_last_iteration = cucumber_state.cucumbers_moved_last_iteration;
        time_steps += 1;
    }

    time_steps
}

fn solve2() -> u64 {
    0
}

#[derive(Clone, Debug)]
struct CucumberState {
    state: Vec<Vec<CucumberSlot>>,
    cucumbers_moved_last_iteration: bool,
    curr_step: usize,
}

impl CucumberState {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_list1(
                tag("\n"),
                many1(map(
                    alt((tag("."), tag(">"), tag("v"))),
                    |tag: &str| match tag {
                        "." => CucumberSlot::Empty,
                        ">" => CucumberSlot::FacingEast(0),
                        "v" => CucumberSlot::FacingSouth(0),
                        &_ => {
                            panic!("nooooo")
                        }
                    },
                )),
            ),
            |mut state| {
                // insert phantom row and col
                state.push(state[0].clone());
                for row in &mut state {
                    row.push(row[0].clone());
                }

                Self {
                    state,
                    cucumbers_moved_last_iteration: true,
                    curr_step: 0,
                }
            },
        )(input)
    }

    fn time_step(&mut self) {
        self.curr_step += 1;
        let mut updated = false;

        let height = self.state.len() - 1;
        let width = self.state[0].len() - 1;

        for row in &mut self.state {
            row[width] = row[0].clone();
        }

        for (row, col) in (0..height).cartesian_product(0..width) {
            if let CucumberSlot::FacingEast(step) = self.state[row][col] {
                assert!(step <= self.curr_step);

                let east_col = (col + 1) % width;
                let east_col_phantom = col + 1;
                if step == self.curr_step
                    || self.state[row][east_col_phantom] != CucumberSlot::Empty
                {
                    continue;
                }

                updated = true;
                self.state[row][col] = CucumberSlot::Empty;
                self.state[row][east_col] = CucumberSlot::FacingEast(self.curr_step);
            }
        }

        self.state[height] = self.state[0].clone();

        for (row, col) in (0..height).cartesian_product(0..width) {
            if let CucumberSlot::FacingSouth(step) = self.state[row][col] {
                assert!(step <= self.curr_step);

                let south_row = (row + 1) % height;
                let south_row_phantom = row + 1;
                if step == self.curr_step
                    || self.state[south_row_phantom][col] != CucumberSlot::Empty
                {
                    continue;
                }

                updated = true;
                self.state[row][col] = CucumberSlot::Empty;
                self.state[south_row][col] = CucumberSlot::FacingSouth(self.curr_step);
            }
        }

        self.cucumbers_moved_last_iteration = updated;
    }

    fn print_state(&self) {
        let printable_state = self
            .state
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cucumber| match cucumber {
                        CucumberSlot::FacingEast(_) => '>',
                        CucumberSlot::FacingSouth(_) => 'v',
                        CucumberSlot::Empty => '.',
                    })
                    .collect::<String>()
            })
            .join("\n");
        println!("{printable_state}");
    }
}

#[derive(Clone, Debug, PartialEq)]
enum CucumberSlot {
    FacingEast(usize),
    FacingSouth(usize),
    Empty,
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
