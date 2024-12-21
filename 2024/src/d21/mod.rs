use crate::common::test;
use itertools::Itertools;
use std::{collections::HashMap, sync::LazyLock};

const MODULE: &str = module_path!();
static PATH_MAPPING: LazyLock<PathMapping> = LazyLock::new(generate_shortest_paths);

#[test]
fn p1_example() {
    test("example", MODULE, p1, 126384);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 203814);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, 154115708116294);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 248566068436630);
}

fn p1(input: &str) -> usize {
    complexities_sum(input, 3)
}

fn p2(input: &str) -> usize {
    complexities_sum(input, 26)
}

fn complexities_sum(input: &str, starting_layer: usize) -> usize {
    let mut cache = HashMap::new();
    input
        .lines()
        .map(|code| {
            let len = len_shortest_seq(code, starting_layer, &mut cache);
            let code_val: usize = code[..code.len() - 1].parse().unwrap();
            len * code_val
        })
        .sum()
}

fn len_shortest_seq<'b, 'a: 'b>(
    buttons: &'a str,
    layer: usize,
    cache: &'b mut HashMap<(&'a str, usize), usize>,
) -> usize {
    if layer == 0 {
        return buttons.len();
    }
    
    let key = (buttons, layer);
    if let Some(value) = cache.get(&key) {
        return *value;
    }

    let mut prev_button = 'A';
    let mut total_len = 0;
    for button in buttons.chars() {
        total_len += PATH_MAPPING[&(prev_button, button)]
            .iter()
            .map(|potential_path| len_shortest_seq(potential_path, layer - 1, cache))
            .min()
            .unwrap();
        prev_button = button;
    }

    cache.insert(key, total_len);
    total_len
}

type PathMapping = HashMap<(char, char), Vec<String>>;

fn generate_shortest_path_entry(
    from: &char,
    to: &char,
    row_mapping: &HashMap<char, usize>,
    col_mapping: &HashMap<char, usize>,
    bad_row: usize,
    bad_col: usize,
) -> ((char, char), Vec<String>) {
    let from_row = row_mapping[from];
    let to_row = row_mapping[to];
    let from_col = col_mapping[from];
    let to_col = col_mapping[to];

    let horz_dir = if from_col < to_col { '>' } else { '<' };

    let vert_dir = if from_row < to_row { 'v' } else { '^' };

    let horz_dist = from_col.abs_diff(to_col);

    let vert_dist = from_row.abs_diff(to_row);

    let vert_then_horz = [
        vec![vert_dir; vert_dist],
        vec![horz_dir; horz_dist],
        vec!['A'],
    ]
    .iter()
    .flatten()
    .collect::<String>();
    
    let horz_then_vert = [
        vec![horz_dir; horz_dist],
        vec![vert_dir; vert_dist],
        vec!['A'],
    ]
    .iter()
    .flatten()
    .collect::<String>();

    let key = (*from, *to);
    let value = if horz_dist == 0 || vert_dist == 0 || from_row == bad_row && to_col == bad_col {
        vec![vert_then_horz]
    } else if to_row == bad_row && from_col == bad_col {
        vec![horz_then_vert]
    } else {
        vec![vert_then_horz, horz_then_vert]
    };

    (key, value)
}

fn generate_shortest_paths() -> PathMapping {
    let nums = ['A', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let row_mapping = HashMap::from([
        ('7', 0_usize),
        ('8', 0),
        ('9', 0),
        ('4', 1),
        ('5', 1),
        ('6', 1),
        ('1', 2),
        ('2', 2),
        ('3', 2),
        ('0', 3),
        ('A', 3),
    ]);
    let col_mapping = HashMap::from([
        ('7', 0_usize),
        ('4', 0),
        ('1', 0),
        ('8', 1),
        ('5', 1),
        ('2', 1),
        ('0', 1),
        ('9', 2),
        ('6', 2),
        ('3', 2),
        ('A', 2),
    ]);

    let num_mapping_iter = nums
        .iter()
        .cartesian_product(&nums)
        .map(|(from, to)| generate_shortest_path_entry(from, to, &row_mapping, &col_mapping, 3, 0));

    let dirs = ['<', '>', '^', 'v', 'A'];
    let row_mapping = HashMap::from([('^', 0_usize), ('A', 0), ('<', 1), ('v', 1), ('>', 1)]);
    let col_mapping = HashMap::from([('<', 0_usize), ('^', 1), ('v', 1), ('A', 2), ('>', 2)]);

    let dir_mapping_iter = dirs
        .iter()
        .cartesian_product(&dirs)
        .map(|(from, to)| generate_shortest_path_entry(from, to, &row_mapping, &col_mapping, 0, 0));

    num_mapping_iter.chain(dir_mapping_iter).collect()
}
