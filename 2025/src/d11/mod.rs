use crate::common::test;
use itertools::Itertools;
use std::{collections::HashMap, fmt::Debug, hash::Hash};

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 5);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 699);
}

#[test]
fn p2_example() {
    test("example2", MODULE, p2, 2);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 388893655378800);
}

fn p1(input: &str) -> usize {
    let device_map = input
        .lines()
        .map(|line| {
            let from = line[..3].to_string();
            let to = line[5..]
                .split_whitespace()
                .map(str::to_string)
                .collect_vec();
            (from, to)
        })
        .collect::<HashMap<_, _>>();

    count_all_paths(&device_map, &String::from("you"), &String::from("out"))
}

fn p2(input: &str) -> usize {
    let device_map = input
        .lines()
        .map(|line| {
            let from = line[..3].to_string();
            let to = line[5..]
                .split_whitespace()
                .map(str::to_string)
                .collect_vec();
            (from, to)
        })
        .collect::<HashMap<_, _>>();

    let dac = String::from("dac");
    let fft = String::from("fft");
    let (paths_dac_fft, first, second) = [(&dac, &fft), (&fft, &dac)]
        .into_iter()
        .find_map(|(start, end)| {
            let dist = count_all_paths(&device_map, start, end);
            if dist > 0 {
                Some((dist, start, end))
            } else {
                None
            }
        })
        .unwrap();

    let paths_to_first = count_all_paths(&device_map, &String::from("svr"), first);
    let paths_from_second = count_all_paths(&device_map, second, &String::from("out"));

    paths_to_first * paths_dac_fft * paths_from_second
}

fn count_all_paths<T: Eq + Hash + Debug>(
    device_map: &HashMap<T, Vec<T>>,
    start: &T,
    end: &T,
) -> usize {
    let mut to_visit = vec![start];

    let mut cache = HashMap::from([(end, 1_usize)]);
    while let Some(device) = to_visit.pop() {
        if cache.contains_key(device) {
            continue;
        }

        let maybe_next_devices = device_map.get(device);

        // is None if any dependencies have not been computed yet
        let maybe_paths = match maybe_next_devices {
            None => Some(0),
            Some(next_devices) => next_devices.iter().try_fold(0, |acc, next_device| {
                cache.get(next_device).map(|inner| inner + acc)
            }),
        };

        if let Some(paths) = maybe_paths {
            if device == start {
                return paths;
            }
            cache.insert(device, paths);
        } else if let Some(next_devices) = maybe_next_devices {
            to_visit.push(device);
            for next_device in next_devices {
                to_visit.push(next_device);
            }
        }
    }
    *cache.get(start).unwrap()
}
