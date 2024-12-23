use crate::common::test;
use itertools::Itertools;
use std::collections::HashMap;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 37327623);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 14119253575);
}

#[test]
fn p2_example() {
    test("example2", MODULE, p2, 23);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 1600);
}

fn p1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let secret = line.parse::<isize>().unwrap();
            let prng = PseudoRng { secret };

            prng.into_iter().nth(2000).unwrap()
        })
        .sum::<isize>() as usize
}

fn p2(input: &str) -> usize {
    let pattern_to_first_price_maps = input
        .lines()
        .map(|line| {
            let secret = line.parse::<isize>().unwrap();
            let prng = PseudoRng { secret };

            let change_pattern_and_price = prng
                .into_iter()
                .map(|secret| secret % 10)
                .tuple_windows()
                .map(|(p1, p2, p3, p4, p5)| ((p2 - p1, p3 - p2, p4 - p3, p5 - p4), p5))
                .take(2000) // take 2000 price changes, not 2000 prices
                .filter(|((c1, c2, c3, c4), _)| {
                    c1 + c2 + c3 + c4 >= 2 && c1 >= &-2 && c2 >= &-2 && c3 >= &-2 && c4 >= &-2
                })
                .collect_vec();

            change_pattern_and_price
                .into_iter()
                .rev() // first occurring pattern should override price value
                .collect::<HashMap<_, _>>()
        })
        .collect_vec();

    *pattern_to_first_price_maps
        .iter()
        .fold(HashMap::new(), |mut acc, curr| {
            for (key, value) in curr {
                *acc.entry(key).or_insert(0) += *value;
            }

            acc
        })
        .values()
        .max()
        .unwrap() as usize
}

struct PseudoRng {
    secret: isize,
}

impl PseudoRng {
    fn mix(&mut self, value: isize) {
        self.secret ^= value;
    }

    fn prune(&mut self) {
        self.secret %= 16777216;
    }

    fn gen_new_secret(&mut self) {
        self.mix(self.secret * 64);
        self.prune();

        self.mix(self.secret / 32);
        self.prune();

        self.mix(self.secret * 2048);
        self.prune();
    }
}

impl IntoIterator for PseudoRng {
    type Item = isize;
    type IntoIter = PseudoRngIterator;

    fn into_iter(self) -> Self::IntoIter {
        PseudoRngIterator { pseudo_rng: self }
    }
}

struct PseudoRngIterator {
    pseudo_rng: PseudoRng,
}

impl Iterator for PseudoRngIterator {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let secret = self.pseudo_rng.secret;
        self.pseudo_rng.gen_new_secret();

        Some(secret)
    }
}
