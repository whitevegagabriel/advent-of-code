use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_till},
    character::complete::digit1,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use ranges::{Arrangement, GenericRange, OperationResult};
use std::{collections::Bound, ops::RangeBounds};

pub fn solve(problem: &str) -> (u64, u64) {
    let (_, (nums, number_mappers)) = tuple((
        preceded(
            tag("seeds: "),
            separated_list1(tag(" "), map(digit1, |s: &str| s.parse::<u64>().unwrap())),
        ),
        separated_list1(tag("\n\n"), NumberMapper::parse),
    ))(problem)
    .unwrap();
    (
        solve1(&nums, &number_mappers),
        solve2(&nums, &number_mappers),
    )
}

fn solve1(nums: &[u64], mappers: &[NumberMapper]) -> u64 {
    nums.iter()
        .map(|num| {
            mappers
                .iter()
                .fold(*num, |prev_num, mapper| mapper.map(prev_num))
        })
        .min()
        .unwrap()
}

fn solve2(nums: &[u64], mappers: &[NumberMapper]) -> u64 {
    let sorted_input_ranges = nums
        .iter()
        .array_chunks()
        .sorted()
        .map(|[start, length]| GenericRange::from(*start..start + length))
        .collect_vec();

    let merged_mapper = mappers
        .iter()
        .cloned()
        .reduce(|prev_mapper, mapper| prev_mapper.merge(&mapper))
        .unwrap();

    merged_mapper
        .maps
        .iter()
        // these maps are sorted by lowest destination, so the first match is the correct answer
        .find_map(|map| {
            // if we were going to match on a destination range, then the lowest input range is the correct answer
            for range in &sorted_input_ranges {
                match range.arrangement(&map.source_range) {
                    Arrangement::Disjoint { self_less } => {
                        if self_less {
                            // next range might overlap
                            continue;
                        } else {
                            // remaining ranges won't overlap because they're sorted
                            return None;
                        }
                    }
                    // Idk why this crate makes these two enums
                    Arrangement::Touching { self_less } => {
                        if self_less {
                            // next range might overlap
                            continue;
                        } else {
                            // remaining ranges won't overlap because they're sorted
                            return None;
                        }
                    }
                    Arrangement::Empty { .. } => {
                        panic!("these empty ranges should have been removed earlier")
                    }
                    _ => {}
                }

                let min_source = match *range & map.source_range {
                    OperationResult::Single(intersection) => match intersection.start_bound() {
                        Bound::Included(start) => *start,
                        _ => panic!("unexpected"),
                    },
                    OperationResult::Empty => {
                        panic!("unexpected empty intersection when earlier check was not disjoint or touching")

                    }
                    OperationResult::Double( .. ) => {
                        unreachable!()
                    },
                };

                return Some(min_source - map.source + map.destination);
            }
            None
        })
        .unwrap()
}

#[derive(PartialEq, Debug, Clone)]
struct NumberMap {
    destination: u64,
    destination_range: GenericRange<u64>,
    source: u64,
    source_range: GenericRange<u64>,
    length: Option<u64>,
}

impl NumberMap {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_list1(tag(" "), map(digit1, |s: &str| s.parse::<u64>().unwrap())),
            |numbers| {
                let (destination, source, length) = numbers.into_iter().collect_tuple().unwrap();
                Self::new(destination, source, length)
            },
        )(input)
    }

    fn new(destination: u64, source: u64, length: u64) -> Self {
        Self {
            destination,
            destination_range: GenericRange::from(destination..destination.saturating_add(length)),
            source,
            source_range: GenericRange::from(source..source.saturating_add(length)),
            length: Some(length),
        }
    }

    fn remapped(&self, new_mapping: &Self) -> Option<Self> {
        let intersection = self.destination_range & new_mapping.source_range;

        let (start_inclusive, end_inclusive) = match intersection {
            OperationResult::Single(range) => (
                match range.start_bound() {
                    Bound::Included(i) => *i,
                    Bound::Excluded(i) => i + 1,
                    Bound::Unbounded => {
                        panic!()
                    }
                },
                match range.end_bound() {
                    Bound::Included(i) => *i,
                    Bound::Excluded(i) => i - 1,
                    Bound::Unbounded => {
                        panic!()
                    }
                },
            ),
            OperationResult::Empty => {
                return None;
            }
            OperationResult::Double(..) => {
                unreachable!()
            }
        };

        let new_source_start = self.reverse_map(start_inclusive);
        let new_dest_start = new_mapping.map(start_inclusive);

        Some(Self::new(
            new_dest_start,
            new_source_start,
            end_inclusive - start_inclusive + 1,
        ))
    }

    fn map(&self, num: u64) -> u64 {
        assert!(self.source_range.contains(&num));
        num - self.source + self.destination
    }

    fn reverse_map(&self, num: u64) -> u64 {
        assert!(self.destination_range.contains(&num));
        num - self.destination + self.source
    }
}

#[derive(PartialEq, Debug, Clone)]
struct NumberMapper {
    maps: Vec<NumberMap>,
}

impl NumberMapper {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            preceded(
                take_till(|c: char| c.is_ascii_digit()),
                separated_list1(tag("\n"), NumberMap::parse),
            ),
            |raw_maps| {
                let sorted_maps = raw_maps
                    .into_iter()
                    .sorted_by_key(|map| map.destination)
                    .collect_vec();
                let mut spaces = vec![NumberMap::new(0, 0, sorted_maps[0].destination)];

                for (map1, map2) in sorted_maps.iter().tuple_windows() {
                    let new_destination = map1.destination + map1.length.unwrap();
                    spaces.push(NumberMap::new(
                        new_destination,
                        new_destination,
                        map2.destination - new_destination,
                    ))
                }

                let final_map = sorted_maps.last().unwrap();
                let final_destination = final_map.destination + final_map.length.unwrap();
                spaces.push(NumberMap {
                    destination: final_destination,
                    destination_range: GenericRange::new_at_least(final_destination),
                    source: final_destination,
                    source_range: GenericRange::new_at_least(final_destination),
                    length: None,
                });

                let maps = spaces
                    .into_iter()
                    .interleave(sorted_maps)
                    .filter(|map| {
                        if let Some(len) = map.length {
                            len > 0
                        } else {
                            true // unbounded
                        }
                    })
                    .collect_vec();
                Self { maps }
            },
        )(input)
    }

    fn merge(&self, other: &NumberMapper) -> Self {
        let new_maps = self
            .maps
            .iter()
            .cartesian_product(&other.maps)
            .filter_map(|(m, next_m)| m.remapped(next_m))
            .sorted_by_key(|m| m.destination)
            .collect_vec();
        Self { maps: new_maps }
    }

    fn map(&self, num: u64) -> u64 {
        self.maps
            .iter()
            .find(|number_map| number_map.source_range.contains(&num))
            .map(|number_map| number_map.map(num))
            .unwrap()
    }
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
