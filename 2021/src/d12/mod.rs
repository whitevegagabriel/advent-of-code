use itertools::Itertools;
use std::collections::{HashMap, HashSet, LinkedList};

pub fn solve(problem: &str) -> (u64, u64) {
    let problem = &problem.lines().collect_vec();
    let mut cave_map = HashMap::<&str, Vec<&str>>::new();
    for line in problem {
        // assume every pairing is only listed once
        let (l, r) = line.split('-').collect_tuple().unwrap();
        cave_map.entry(l).or_default().push(r);
        cave_map.entry(r).or_default().push(l);
    }
    (solve1(&cave_map), solve2(&cave_map))
}

fn solve1(cave: &HashMap<&str, Vec<&str>>) -> u64 {
    fn filter(curr_state: &State, next_location: &str) -> Option<State> {
        if !curr_state.visited.contains(next_location) {
            let mut next = curr_state.clone();
            next.location = next_location.into();

            if next_location.chars().next().unwrap().is_ascii_lowercase() {
                next.visited.insert(next_location.into());
            }

            Some(next)
        } else {
            None
        }
    }

    unique_paths(cave, filter)
}

fn solve2(cave: &HashMap<&str, Vec<&str>>) -> u64 {
    fn filter(curr_state: &State, next_location: &str) -> Option<State> {
        let mut next_state = curr_state.clone();
        next_state.location = String::from(next_location);

        if !curr_state.visited.contains(next_location) {
            // we haven't visited yet
            if next_location.chars().next().unwrap().is_ascii_lowercase() {
                next_state.visited.insert(String::from(next_location));
            }
            Some(next_state)
        } else if next_location != "start" && !curr_state.visited_twice {
            // we haven't visited any location twice yet (though "start" is banned)
            next_state.visited_twice = true;
            Some(next_state)
        } else {
            None
        }
    }

    unique_paths(cave, filter)
}

fn unique_paths(
    cave: &HashMap<&str, Vec<&str>>,
    next_state_filter: impl Fn(&State, &str) -> Option<State>,
) -> u64 {
    let start = State {
        location: "start".into(),
        visited: HashSet::from(["start".into()]),
        visited_twice: false,
    };
    let mut to_visit = LinkedList::new();
    to_visit.push_back(start);

    let mut num_paths = 0;
    while let Some(curr) = to_visit.pop_front() {
        if curr.location == "end" {
            num_paths += 1;
            continue;
        }

        let next_locations = cave.get(curr.location.as_str()).unwrap();

        next_locations
            .iter()
            .filter_map(|next_location| next_state_filter(&curr, next_location))
            .for_each(|state| to_visit.push_back(state));
    }
    num_paths
}

#[derive(Clone)]
struct State {
    location: String,
    visited: HashSet<String>,
    visited_twice: bool,
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
