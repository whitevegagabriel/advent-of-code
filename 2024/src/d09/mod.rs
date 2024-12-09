use crate::common::test;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 1928);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 6386640365805);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 2858);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 6423258376982);
}

fn p1(input: &str) -> usize {
    let mut disk = expand(parse_input(input));

    let mut left = 0;
    let mut right = disk.len() - 1;
    assert!(disk[right].is_some());

    while right > left {
        if disk[left].is_some() {
            left += 1;
            continue;
        }

        disk.swap(left, right);
        right -= 1;

        while disk[right].is_none() {
            right -= 1;
        }
    }

    checksum(disk)
}

fn p2(input: &str) -> usize {
    let mut compact_disk = parse_input(input);

    let mut right = compact_disk.len() - 1;

    while right >= 2 {
        for left in (1..right).step_by(2) {
            let left_len = compact_disk[left].1;
            let right_len = compact_disk[right].1;
            if left_len >= right_len {
                compact_disk.swap(left, right);
                compact_disk[right].1 = right_len;
                compact_disk.insert(left, (None, 0));
                compact_disk.insert(left + 2, (None, left_len - right_len));
                right += 2;
                break;
            }
        }
        right -= 2;
    }

    checksum(expand(compact_disk))
}

fn parse_input(input: &str) -> Vec<(Option<usize>, usize)> {
    input
        .chars()
        .enumerate()
        .map(|(idx, c)| {
            let len = c as usize - '0' as usize;
            if idx % 2 == 0 {
                (Some(idx / 2), len)
            } else {
                (None, len)
            }
        })
        .collect()
}

fn expand(compact_disk: Vec<(Option<usize>, usize)>) -> Vec<Option<usize>> {
    compact_disk
        .into_iter()
        .flat_map(|(maybe_id, len)| vec![maybe_id; len])
        .collect()
}

fn checksum(disk: Vec<Option<usize>>) -> usize {
    disk.iter()
        .enumerate()
        .map(|(idx, maybe_id)| match maybe_id {
            None => 0,
            Some(id) => id * idx,
        })
        .sum()
}
