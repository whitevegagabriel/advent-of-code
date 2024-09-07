use crate::d19::{
    Checker::{GreaterThan, LessThan, Pass},
    SortResult::{Accepted, ReSort, Rejected},
    XmasProperty::{A, M, S, X},
};
use itertools::Itertools;
use std::{collections::HashMap, ops::RangeInclusive};

pub fn solve(problem: &str) -> (usize, usize) {
    let (sorters_input, xmas_input) = problem.split("\n\n").collect_tuple().unwrap();
    let sorters = sorters_input
        .lines()
        .map(|process| {
            let (label, criteria) = process.split("{").collect_tuple().unwrap();
            let criteria = &criteria[0..criteria.len() - 1];
            let sorter = Sorter::parse(criteria);
            (label, sorter)
        })
        .collect::<HashMap<_, _>>();

    let xmas_structs = xmas_input
        .lines()
        .map(|line| {
            let line = &line[1..line.len() - 1];
            Xmas::parse(line)
        })
        .collect_vec();

    (solve1(&sorters, &xmas_structs), solve2(&sorters))
}

fn solve1(sorters: &HashMap<&str, Sorter>, xmas_structs: &[Xmas]) -> usize {
    let first_sorter = &sorters["in"];
    xmas_structs
        .iter()
        .filter_map(|xmas| match first_sorter.sort(xmas, sorters) {
            Accepted => Some(xmas.sum()),
            Rejected => None,
            ReSort(_) => unreachable!(),
        })
        .sum()
}

fn solve2(sorters: &HashMap<&str, Sorter>) -> usize {
    let first_sorter = &sorters["in"];
    let start_range = XmasRange {
        x: 1..=4000,
        m: 1..=4000,
        a: 1..=4000,
        s: 1..=4000,
    };

    first_sorter
        .sort_range(start_range, sorters)
        .iter()
        .filter_map(|(range, result)| match result {
            Accepted => Some(range.num_combinations()),
            Rejected => None,
            ReSort(_) => unreachable!(),
        })
        .sum()
}

struct Sorter<'a> {
    checkers: Vec<Checker<'a>>,
}

impl<'a> Sorter<'a> {
    fn parse(input: &'a str) -> Self {
        let checkers = input.split(",").map(Checker::parse).collect_vec();
        Self { checkers }
    }

    fn sort(&'a self, xmas: &Xmas, sorters: &'a HashMap<&'a str, Sorter>) -> SortResult {
        let sort_result = self
            .checkers
            .iter()
            .filter_map(|checker| checker.check(xmas))
            .next()
            .unwrap();
        if let ReSort(label) = &sort_result {
            let new_sorter = &sorters[label];
            new_sorter.sort(xmas, sorters)
        } else {
            sort_result
        }
    }

    fn sort_range(
        &'a self,
        xmas_range: XmasRange,
        sorters: &'a HashMap<&'a str, Sorter>,
    ) -> Vec<(XmasRange, SortResult)> {
        let mut prev_range = xmas_range;
        let mut range_check_results = Vec::new();
        for checker in &self.checkers {
            let (maybe_pass_check, maybe_skip_check) = checker.check_range(&prev_range);

            if let Some((range, sort_result)) = maybe_pass_check {
                if let ReSort(label) = &sort_result {
                    let new_sorter = &sorters[label];
                    let mut sub_results = new_sorter.sort_range(range, sorters);
                    range_check_results.append(&mut sub_results);
                } else {
                    range_check_results.push((range, sort_result));
                }
            }

            match maybe_skip_check {
                None => break,
                Some(range) => prev_range = range,
            }
        }

        range_check_results
    }
}

enum XmasProperty {
    X,
    M,
    A,
    S,
}

impl XmasProperty {
    fn parse(input: &str) -> Self {
        match input {
            "x" => X,
            "m" => M,
            "a" => A,
            "s" => S,
            _ => unreachable!(),
        }
    }
}

enum Checker<'a> {
    LessThan {
        prop: XmasProperty,
        value: usize,
        sort_result: SortResult<'a>,
    },
    GreaterThan {
        prop: XmasProperty,
        value: usize,
        sort_result: SortResult<'a>,
    },
    Pass(SortResult<'a>),
}

impl<'a> Checker<'a> {
    fn parse(input: &'a str) -> Self {
        if !input.contains(":") {
            return Pass(SortResult::parse(input));
        } else {
            let (cmp, sort_result) = input.split(":").collect_tuple().unwrap();
            let prop = XmasProperty::parse(&cmp[0..1]);
            let value = cmp[2..].parse().unwrap();
            let sort_result = SortResult::parse(sort_result);
            match &cmp[1..2] {
                "<" => LessThan {
                    prop,
                    value,
                    sort_result,
                },
                ">" => GreaterThan {
                    prop,
                    value,
                    sort_result,
                },
                _ => unreachable!(),
            }
        }
    }

    fn check(&self, xmas: &Xmas) -> Option<SortResult> {
        match self {
            LessThan {
                prop,
                value,
                sort_result,
            } => {
                if xmas.get_prop(prop) < *value {
                    Some(sort_result.clone())
                } else {
                    None
                }
            }
            GreaterThan {
                prop,
                value,
                sort_result,
            } => {
                if xmas.get_prop(prop) > *value {
                    Some(sort_result.clone())
                } else {
                    None
                }
            }
            Pass(sort_result) => Some(sort_result.clone()),
        }
    }

    fn check_range(
        &self,
        xmas_range: &XmasRange,
    ) -> (Option<(XmasRange, SortResult)>, Option<XmasRange>) {
        match self {
            LessThan {
                prop,
                value,
                sort_result,
            } => {
                let value = *value;
                let to_check = xmas_range.get_prop(prop);

                let start = *to_check.start();
                let end = *to_check.end();

                if end < value {
                    let matched = Some((xmas_range.clone(), sort_result.clone()));
                    let leftover = None;
                    (matched, leftover)
                } else if start > value {
                    let matched = None;
                    let leftover = Some(xmas_range.clone());
                    (matched, leftover)
                } else {
                    let mut range = xmas_range.clone();
                    *range.get_prop_mut(prop) = start..=value - 1;
                    let matched = Some((range, sort_result.clone()));

                    let mut range = xmas_range.clone();
                    *range.get_prop_mut(prop) = value..=end;
                    let leftover = Some(range);

                    (matched, leftover)
                }
            }
            GreaterThan {
                prop,
                value,
                sort_result,
            } => {
                let value = *value;
                let to_check = xmas_range.get_prop(prop);

                let start = *to_check.start();
                let end = *to_check.end();

                if start > value {
                    let matched = Some((xmas_range.clone(), sort_result.clone()));
                    let leftover = None;
                    (matched, leftover)
                } else if end < value {
                    let matched = None;
                    let leftover = Some(xmas_range.clone());
                    (matched, leftover)
                } else {
                    let mut range = xmas_range.clone();
                    *range.get_prop_mut(prop) = value + 1..=end;
                    let matched = Some((range, sort_result.clone()));

                    let mut range = xmas_range.clone();
                    *range.get_prop_mut(prop) = start..=value;
                    let leftover = Some(range);

                    (matched, leftover)
                }
            }
            Pass(sort_result) => (Some((xmas_range.clone(), sort_result.clone())), None),
        }
    }
}

#[derive(Clone)]
struct XmasRange {
    x: RangeInclusive<usize>,
    m: RangeInclusive<usize>,
    a: RangeInclusive<usize>,
    s: RangeInclusive<usize>,
}

impl XmasRange {
    fn num_combinations(&self) -> usize {
        range_len(&self.x) * range_len(&self.m) * range_len(&self.a) * range_len(&self.s)
    }

    fn get_prop_mut(&mut self, prop: &XmasProperty) -> &mut RangeInclusive<usize> {
        match prop {
            X => &mut self.x,
            M => &mut self.m,
            A => &mut self.a,
            S => &mut self.s,
        }
    }

    fn get_prop(&self, prop: &XmasProperty) -> &RangeInclusive<usize> {
        match prop {
            X => &self.x,
            M => &self.m,
            A => &self.a,
            S => &self.s,
        }
    }
}

fn range_len(range: &RangeInclusive<usize>) -> usize {
    range.end() - range.start() + 1
}

struct Xmas {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Xmas {
    fn parse(input: &str) -> Self {
        let (x, m, a, s) = input
            .split(",")
            .map(|eq| eq.split("=").skip(1).collect::<String>().parse().unwrap())
            .collect_tuple()
            .unwrap();
        Self { x, m, a, s }
    }

    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    fn get_prop(&self, prop: &XmasProperty) -> usize {
        match prop {
            X => self.x,
            M => self.m,
            A => self.a,
            S => self.s,
        }
    }
}

#[derive(Clone)]
enum SortResult<'a> {
    Accepted,
    Rejected,
    ReSort(&'a str),
}

impl<'a> SortResult<'a> {
    fn parse(input: &'a str) -> Self {
        match input {
            "A" => Accepted,
            "R" => Rejected,
            _ => ReSort(input),
        }
    }
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
