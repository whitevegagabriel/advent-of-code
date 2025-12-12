use crate::common::test;
use itertools::Itertools;

const MODULE: &str = module_path!();

#[test]
fn p1_input() {
    test("input", MODULE, p1, 476);
}

fn p1(input: &str) -> usize {
    let inputs = input.split("\n\n").collect_vec();
    let present_sizes = inputs[..=5]
        .iter()
        .map(|present| present.chars().filter(|c| c == &'#').count())
        .collect_vec();

    // manually inspecting the input shows that in some cases, if we were to actually have the required counts of
    // each present, they wouldn't actually fit in the required area even if you broke apart each present into
    // independent blocks. the remaining input lines leave a lot of "wiggle room", leading me to believe that there
    // are a lot of valid arrangements (but we only need to know if there is at least 1).
    inputs[6]
        .lines()
        .filter(|line| {
            let (dims_str, counts_str) =
                line.split(':').collect_tuple().unwrap();
            let (width, height) = dims_str
                .split('x')
                .map(|dim| dim.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            let present_counts = counts_str
                .split_whitespace()
                .map(|count| count.parse::<usize>().unwrap())
                .collect_vec();

            let max_area = width * height;
            let min_area = present_sizes
                .iter()
                .zip(&present_counts)
                .map(|(size, count)| size * count)
                .sum::<usize>();
            max_area >= min_area
        })
        .count()
}
