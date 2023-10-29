use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, HashSet};

pub fn solve(problem: &[&str]) -> (u64, u64) {
    (solve1(problem), solve2(problem))
}

fn solve1(problem: &[&str]) -> u64 {
    let num_segments_unique_length = problem
        .iter()
        .flat_map(|l| l[61..].split_whitespace())
        .filter(|n| [2, 3, 4, 7].contains(&n.len()))
        .count();
    u64::try_from(num_segments_unique_length).unwrap()
}

fn solve2(problem: &[&str]) -> u64 {
    problem
        .iter()
        .map(|l| SegmentProblem::parse(l))
        .map(|p| {
            let charset_to_char_num_mapping = generate_mapping(&p.segments);
            let full_number = p
                .numbers
                .iter()
                .map(|n| {
                    let set = n.chars().collect::<BTreeSet<_>>();
                    charset_to_char_num_mapping.get(&set).unwrap()
                })
                .collect::<String>();
            full_number.parse::<u64>().unwrap()
        })
        .sum()
}

fn generate_mapping(numbers: &[String]) -> HashMap<BTreeSet<char>, char> {
    let mut numbers = numbers
        .iter()
        .map(|s| s.chars().collect::<BTreeSet<_>>())
        .collect::<HashSet<_>>();
    let mut mapping = HashMap::new();

    // order matters
    // able to narrow down the possibilities by removing them from another set

    let one = numbers.iter().find(|n| n.len() == 2).unwrap().clone();
    numbers.remove(&one);
    mapping.insert(one.clone(), '1');

    let four = numbers.iter().find(|n| n.len() == 4).unwrap().clone();
    numbers.remove(&four);
    mapping.insert(four, '4');

    let seven = numbers.iter().find(|n| n.len() == 3).unwrap().clone();
    numbers.remove(&seven);
    mapping.insert(seven, '7');

    let eight = numbers.iter().find(|n| n.len() == 7).unwrap().clone();
    numbers.remove(&eight);
    mapping.insert(eight, '8');

    let three = numbers
        .iter()
        .find(|n| n.len() == 5 && n.is_superset(&one))
        .unwrap()
        .clone();
    numbers.remove(&three);
    mapping.insert(three.clone(), '3');

    let nine = numbers
        .iter()
        .find(|n| n.is_superset(&three))
        .unwrap()
        .clone();
    numbers.remove(&nine);
    mapping.insert(nine, '9');

    let zero = numbers
        .iter()
        .find(|n| n.len() == 6 && n.is_superset(&one))
        .unwrap()
        .clone();
    numbers.remove(&zero);
    mapping.insert(zero, '0');

    let six = numbers.iter().find(|n| n.len() == 6).unwrap().clone();
    numbers.remove(&six);
    mapping.insert(six.clone(), '6');

    let five = numbers.iter().find(|n| n.is_subset(&six)).unwrap().clone();
    numbers.remove(&five);
    mapping.insert(five, '5');

    let two = numbers.into_iter().next().unwrap();
    mapping.insert(two, '2');

    mapping
}

struct SegmentProblem {
    segments: Vec<String>,
    numbers: Vec<String>,
}

impl SegmentProblem {
    fn parse(input: &str) -> Self {
        let (segments, number) = input.split(" | ").collect_tuple().unwrap();
        SegmentProblem {
            segments: segments.split_whitespace().map(String::from).collect_vec(),
            numbers: number.split_whitespace().map(String::from).collect_vec(),
        }
    }
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
