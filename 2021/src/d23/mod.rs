use itertools::Itertools;
use nom::{
    bytes::complete::take_while, character::complete::anychar, combinator::map,
    multi::separated_list1, sequence::preceded, IResult,
};
use pathfinding::prelude::astar;

pub fn solve(problem: &str) -> (u64, u64) {
    let (_, game_state_1) = GameState::parse(problem).unwrap();

    let mut problem_2 = problem.to_string();
    problem_2.insert_str(41, "  #D#C#B#A#\n  #D#B#A#C#");
    let (_, game_state_2) = GameState::parse(&problem_2).unwrap();

    (solve1(&game_state_1), solve2(&game_state_2))
}

fn solve1(game_state: &GameState<2>) -> u64 {
    let winning_state = GameState {
        corridor: [None; 7],
        burrows: [
            [Some(Amphipod::Amber); 2],
            [Some(Amphipod::Bronze); 2],
            [Some(Amphipod::Copper); 2],
            [Some(Amphipod::Desert); 2],
        ],
    };
    let (_, dist) = astar(
        game_state,
        GameState::successors,
        |_| 0,
        |s| s == &winning_state,
    )
    .expect("there should be some winning path");
    dist
}

fn solve2(game_state: &GameState<4>) -> u64 {
    let winning_state = GameState {
        corridor: [None; 7],
        burrows: [
            [Some(Amphipod::Amber); 4],
            [Some(Amphipod::Bronze); 4],
            [Some(Amphipod::Copper); 4],
            [Some(Amphipod::Desert); 4],
        ],
    };
    let (_, dist) = astar(
        game_state,
        GameState::successors,
        GameState::heuristic,
        |s| s == &winning_state,
    )
    .expect("there should be some winning path");
    dist
}

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
struct GameState<const N: usize> {
    corridor: [Option<Amphipod>; 7],
    burrows: [[Option<Amphipod>; N]; 4],
}

impl<const N: usize> GameState<N> {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            preceded(
                take_while(|c: char| !c.is_alphabetic()),
                separated_list1(take_while(|c: char| !c.is_alphabetic()), anychar),
            ),
            |amphipods: Vec<char>| {
                let amphipods: Vec<Amphipod> =
                    amphipods.into_iter().map(Amphipod::from).collect_vec();

                let mut burrows = [[None; N]; 4];

                for (burrow_idx, burrow) in burrows.iter_mut().enumerate() {
                    for (spot_idx, burrow_spot) in burrow.iter_mut().enumerate() {
                        let amphipod_idx = burrow_idx + (4 * spot_idx);
                        *burrow_spot = Some(amphipods[amphipod_idx])
                    }
                }

                GameState {
                    corridor: [None; 7],
                    burrows,
                }
            },
        )(input)
    }
    fn successors(&self) -> Vec<(Self, u64)> {
        for (idx, maybe_amphipod) in self.corridor.iter().enumerate() {
            if let Some(amphipod) = maybe_amphipod {
                let maybe_distance = self.distance_to_own_burrow(amphipod, idx);

                if let Some((blocks_traveled, dest_idx)) = maybe_distance {
                    let mut next_game = self.clone();
                    next_game.corridor[idx] = None;
                    next_game.burrows[amphipod.burrow_idx()][dest_idx] = Some(*amphipod);
                    let energy_consumed = blocks_traveled * amphipod.unit_energy_consumption();
                    return vec![(next_game, energy_consumed)];
                }
            }
        }

        let mut eligible_burrowed_amphipods = vec![];
        for (idx, burrow) in self.burrows.iter().enumerate() {
            // don't evaluate if all amphipods are already in position
            if burrow.iter().all(|a| match a {
                None => true,
                Some(a) => a.burrow_idx() == idx,
            }) {
                continue;
            }

            let (a_idx, amphipod) = burrow
                .iter()
                .enumerate()
                .find_or_first(|(_, a)| a.is_some())
                .unwrap();

            if let Some(amphipod) = amphipod {
                eligible_burrowed_amphipods.push((idx, a_idx, amphipod))
            }
        }

        let mut next_games = vec![];

        for (burrow_idx, amphipod_idx, amphipod) in eligible_burrowed_amphipods {
            let mut next_game_template = self.clone();
            next_game_template.burrows[burrow_idx][amphipod_idx] = None;

            let mut corridor_idx = burrow_idx + 1; // to the left
            let mut blocks_travelled = 2 + amphipod_idx as u64;
            while self.corridor[corridor_idx].is_none() {
                let mut next_game = next_game_template.clone();
                next_game.corridor[corridor_idx] = Some(*amphipod);
                let energy_consumed = blocks_travelled * amphipod.unit_energy_consumption();
                next_games.push((next_game, energy_consumed));

                if corridor_idx == 0 {
                    break;
                }

                corridor_idx -= 1;
                blocks_travelled += 2;
                if corridor_idx == 0 {
                    blocks_travelled -= 1;
                }
            }

            corridor_idx = burrow_idx + 2; // to the right
            blocks_travelled = 2 + amphipod_idx as u64;
            while let Some(None) = self.corridor.get(corridor_idx) {
                let mut next_game = next_game_template.clone();
                next_game.corridor[corridor_idx] = Some(*amphipod);
                let energy_consumed = blocks_travelled * amphipod.unit_energy_consumption();
                next_games.push((next_game, energy_consumed));

                corridor_idx += 1;
                blocks_travelled += 2;
                if corridor_idx == self.corridor.len() - 1 {
                    blocks_travelled -= 1;
                }
            }
        }
        next_games
    }

    fn distance_to_own_burrow(
        &self,
        amphipod: &Amphipod,
        starting_corridor_idx: usize,
    ) -> Option<(u64, usize)> {
        let burrow_idx = amphipod.burrow_idx();

        // verify that there are no foreign amphipods in burrow
        if !self.burrows[burrow_idx].iter().all(|maybe_a| {
            if let Some(a) = maybe_a {
                a.burrow_idx() == burrow_idx
            } else {
                true
            }
        }) {
            return None;
        }

        let (a_idx, maybe_amphipod) = self.burrows[burrow_idx]
            .iter()
            .enumerate()
            .rev()
            .find(|(_, a)| a.is_none())
            .unwrap();

        if maybe_amphipod.is_some() {
            panic!("burrow should have at least one empty spot")
        }

        let starting_idx_is_left_of_burrow = starting_corridor_idx < burrow_idx + 2;

        let mut corridor_idx = if starting_idx_is_left_of_burrow {
            // starting corridor position is to the left of burrow
            burrow_idx + 1
        } else {
            // its to the right
            burrow_idx + 2
        };

        let mut blocks_travelled = 2 + a_idx as u64;
        while self.corridor[corridor_idx].is_none() {
            if starting_idx_is_left_of_burrow {
                corridor_idx -= 1;
            } else {
                corridor_idx += 1;
            }

            blocks_travelled += 2;
            if corridor_idx == 0 || corridor_idx == self.corridor.len() - 1 {
                blocks_travelled -= 1;
            }
        }
        if corridor_idx == starting_corridor_idx {
            return Some((blocks_travelled, a_idx));
        }
        None
    }

    fn heuristic(&self) -> u64 {
        /*
        I tried heuristics like:
         - minimum cost to put all amphipods in their correct room
         - number of amphipods out of place

        But both resulted in slower execution times for me
        */
        0
    }

    fn absolute_corridor_horz_pos(relative_pos: &usize) -> usize {
        if (0..=1).contains(relative_pos) {
            *relative_pos
        } else if (2..=5).contains(relative_pos) {
            2 * relative_pos - 1
        } else {
            10
        }
    }

    fn absolute_burrow_horz_pos(relative_pos: &usize) -> usize {
        2 * relative_pos + 2
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn unit_energy_consumption(&self) -> u64 {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }

    fn burrow_idx(&self) -> usize {
        match self {
            Amphipod::Amber => 0,
            Amphipod::Bronze => 1,
            Amphipod::Copper => 2,
            Amphipod::Desert => 3,
        }
    }
}

impl From<char> for Amphipod {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Amber,
            'B' => Self::Bronze,
            'C' => Self::Copper,
            'D' => Self::Desert,
            _ => {
                panic!("{value} is not valid")
            }
        }
    }
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}

#[test]
fn test_parse_game_state() {
    let input = r#"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
"#;
    let (_, game_state) = GameState::parse(input).unwrap();

    assert_eq!(
        GameState {
            corridor: [None; 7],
            burrows: [
                [Some(Amphipod::Bronze), Some(Amphipod::Amber)],
                [Some(Amphipod::Copper), Some(Amphipod::Desert)],
                [Some(Amphipod::Bronze), Some(Amphipod::Copper)],
                [Some(Amphipod::Desert), Some(Amphipod::Amber)]
            ],
        },
        game_state
    )
}

#[test]
fn test_successors_amphipod_to_burrow() {
    let game_state = GameState {
        corridor: [None, Some(Amphipod::Amber), None, None, None, None, None],
        burrows: [[None; 2]; 4],
    };

    let actual = game_state.successors();

    assert_eq!(
        vec![(
            GameState {
                corridor: [None, None, None, None, None, None, None],
                burrows: [
                    [None, Some(Amphipod::Amber)],
                    [None; 2],
                    [None; 2],
                    [None; 2]
                ],
            },
            3
        )],
        actual
    );
}

#[test]
fn test_successors_amphipod_to_burrow_with_obstacle() {
    let game_state = GameState {
        corridor: [
            Some(Amphipod::Amber),
            Some(Amphipod::Bronze),
            None,
            None,
            None,
            None,
            None,
        ],
        burrows: [[None; 2]; 4],
    };

    let actual = game_state.successors();

    assert_eq!(
        vec![(
            GameState {
                corridor: [Some(Amphipod::Amber), None, None, None, None, None, None],
                burrows: [
                    [None; 2],
                    [None, Some(Amphipod::Bronze)],
                    [None; 2],
                    [None; 2]
                ],
            },
            50
        )],
        actual
    );
}

#[test]
fn test_successors_burrow_to_corridor() {
    use std::collections::HashSet;

    let game_state = GameState {
        corridor: [None; 7],
        burrows: [
            [None, Some(Amphipod::Amber)],
            [None; 2],
            [None; 2],
            [None; 2],
        ],
    };

    let actual: HashSet<_> = game_state.successors().into_iter().collect();

    assert_eq!(
        HashSet::from([
            (
                GameState {
                    corridor: [Some(Amphipod::Amber), None, None, None, None, None, None],
                    burrows: [[None; 2]; 4],
                },
                4
            ),
            (
                GameState {
                    corridor: [None, Some(Amphipod::Amber), None, None, None, None, None],
                    burrows: [[None; 2]; 4],
                },
                3
            ),
            (
                GameState {
                    corridor: [None, None, Some(Amphipod::Amber), None, None, None, None],
                    burrows: [[None; 2]; 4],
                },
                3
            ),
            (
                GameState {
                    corridor: [None, None, None, Some(Amphipod::Amber), None, None, None],
                    burrows: [[None; 2]; 4],
                },
                5
            ),
            (
                GameState {
                    corridor: [None, None, None, None, Some(Amphipod::Amber), None, None],
                    burrows: [[None; 2]; 4],
                },
                7
            ),
            (
                GameState {
                    corridor: [None, None, None, None, None, Some(Amphipod::Amber), None],
                    burrows: [[None; 2]; 4],
                },
                9
            ),
            (
                GameState {
                    corridor: [None, None, None, None, None, None, Some(Amphipod::Amber)],
                    burrows: [[None; 2]; 4],
                },
                10
            ),
        ]),
        actual
    );
}

#[test]
fn test_winning_strategy_depth_2() {
    let game_state = GameState {
        corridor: [None; 7],
        burrows: [
            [Some(Amphipod::Bronze), Some(Amphipod::Amber)],
            [Some(Amphipod::Copper), Some(Amphipod::Desert)],
            [Some(Amphipod::Bronze), Some(Amphipod::Copper)],
            [Some(Amphipod::Desert), Some(Amphipod::Amber)],
        ],
    };

    let actual = game_state.successors();

    let expected = (
        GameState {
            corridor: [None, None, Some(Amphipod::Bronze), None, None, None, None],
            burrows: [
                [Some(Amphipod::Bronze), Some(Amphipod::Amber)],
                [Some(Amphipod::Copper), Some(Amphipod::Desert)],
                [None, Some(Amphipod::Copper)],
                [Some(Amphipod::Desert), Some(Amphipod::Amber)],
            ],
        },
        40,
    );

    assert!(actual.contains(&expected));

    let (game_state, _) = expected;

    let actual = game_state.successors();

    let expected = (
        GameState {
            corridor: [
                None,
                None,
                Some(Amphipod::Bronze),
                Some(Amphipod::Copper),
                None,
                None,
                None,
            ],
            burrows: [
                [Some(Amphipod::Bronze), Some(Amphipod::Amber)],
                [None, Some(Amphipod::Desert)],
                [None, Some(Amphipod::Copper)],
                [Some(Amphipod::Desert), Some(Amphipod::Amber)],
            ],
        },
        200,
    );

    assert!(actual.contains(&expected));

    let (game_state, _) = expected;

    let actual = game_state.successors();

    let expected = (
        GameState {
            corridor: [None, None, Some(Amphipod::Bronze), None, None, None, None],
            burrows: [
                [Some(Amphipod::Bronze), Some(Amphipod::Amber)],
                [None, Some(Amphipod::Desert)],
                [Some(Amphipod::Copper), Some(Amphipod::Copper)],
                [Some(Amphipod::Desert), Some(Amphipod::Amber)],
            ],
        },
        200,
    );

    assert!(actual.contains(&expected));
}

#[test]
fn test_winning_strategy_depth_4() {
    let game_state = GameState {
        corridor: [None; 7],
        burrows: [
            [Some(Amphipod::Bronze), Some(Amphipod::Amber)],
            [Some(Amphipod::Copper), Some(Amphipod::Desert)],
            [Some(Amphipod::Bronze), Some(Amphipod::Copper)],
            [Some(Amphipod::Desert), Some(Amphipod::Amber)],
        ],
    };

    let actual = game_state.successors();

    let expected = (
        GameState {
            corridor: [None, None, Some(Amphipod::Bronze), None, None, None, None],
            burrows: [
                [Some(Amphipod::Bronze), Some(Amphipod::Amber)],
                [Some(Amphipod::Copper), Some(Amphipod::Desert)],
                [None, Some(Amphipod::Copper)],
                [Some(Amphipod::Desert), Some(Amphipod::Amber)],
            ],
        },
        40,
    );

    assert!(actual.contains(&expected));

    let (game_state, _) = expected;

    let actual = game_state.successors();

    let expected = (
        GameState {
            corridor: [
                None,
                None,
                Some(Amphipod::Bronze),
                Some(Amphipod::Copper),
                None,
                None,
                None,
            ],
            burrows: [
                [Some(Amphipod::Bronze), Some(Amphipod::Amber)],
                [None, Some(Amphipod::Desert)],
                [None, Some(Amphipod::Copper)],
                [Some(Amphipod::Desert), Some(Amphipod::Amber)],
            ],
        },
        200,
    );

    assert!(actual.contains(&expected));

    let (game_state, _) = expected;

    let actual = game_state.successors();

    let expected = (
        GameState {
            corridor: [None, None, Some(Amphipod::Bronze), None, None, None, None],
            burrows: [
                [Some(Amphipod::Bronze), Some(Amphipod::Amber)],
                [None, Some(Amphipod::Desert)],
                [Some(Amphipod::Copper), Some(Amphipod::Copper)],
                [Some(Amphipod::Desert), Some(Amphipod::Amber)],
            ],
        },
        200,
    );

    assert!(actual.contains(&expected));
}
