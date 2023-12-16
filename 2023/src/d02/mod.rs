use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

pub fn solve(problem: &str) -> (usize, usize) {
    let (_, games) = separated_list1(tag("\n"), Game::parse)(problem).unwrap();
    (solve1(&games), solve2(games))
}

fn solve1(games: &[Game]) -> usize {
    games
        .iter()
        .filter_map(|game| {
            if game.cube_collections.iter().all(|collection| {
                collection.red <= 12 && collection.green <= 13 && collection.blue <= 14
            }) {
                return Some(game.id);
            }
            None
        })
        .sum()
}

fn solve2(games: Vec<Game>) -> usize {
    games
        .into_iter()
        .map(|game| {
            let min_set = game
                .cube_collections
                .into_iter()
                .reduce(|prev_collection, collection| CubeCollection {
                    red: prev_collection.red.max(collection.red),
                    green: prev_collection.green.max(collection.green),
                    blue: prev_collection.blue.max(collection.blue),
                })
                .unwrap();
            min_set.red * min_set.green * min_set.blue
        })
        .sum()
}

#[derive(PartialEq, Debug)]
struct Game {
    id: usize,
    cube_collections: Vec<CubeCollection>,
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        preceded(
            tag("Game "),
            map(
                separated_pair(
                    map(digit1, |id: &str| id.parse::<usize>().unwrap()),
                    tag(":"),
                    separated_list1(tag(";"), CubeCollection::parse),
                ),
                |(id, cube_collections)| Self {
                    id,
                    cube_collections,
                },
            ),
        )(input)
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
struct CubeCollection {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeCollection {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_list1(
                tag(","),
                preceded(
                    tag(" "),
                    separated_pair(
                        map(digit1, |id: &str| id.parse::<usize>().unwrap()),
                        tag(" "),
                        alpha1,
                    ),
                ),
            ),
            |blocks| {
                let mut collection = Self {
                    red: 0,
                    green: 0,
                    blue: 0,
                };
                for (qty, color) in blocks {
                    let property = match color {
                        "red" => &mut collection.red,
                        "green" => &mut collection.green,
                        "blue" => &mut collection.blue,
                        _ => panic!("that was not expected"),
                    };
                    *property = qty;
                }
                collection
            },
        )(input)
    }
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}

#[test]
fn test_parse_collection() {
    let input = " 3 blue, 4 red";
    let (_, collection) = CubeCollection::parse(input).unwrap();
    assert_eq!(
        CubeCollection {
            red: 4,
            green: 0,
            blue: 3,
        },
        collection
    );
}

#[test]
fn test_parse_game() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let (_, game) = Game::parse(input).unwrap();

    assert_eq!(
        Game {
            id: 1,
            cube_collections: vec![
                CubeCollection {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                CubeCollection {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                CubeCollection {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ]
        },
        game
    )
}
