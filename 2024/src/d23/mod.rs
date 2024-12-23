use crate::common::test;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::{BTreeMap, BTreeSet};

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 7);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 1348);
}

#[test]
fn p2_example() {
    test("example", MODULE, p2, String::from("co,de,ka,ta"));
}

#[test]
fn p2_input() {
    test(
        "input",
        MODULE,
        p2,
        String::from("am,bv,ea,gh,is,iy,ml,nj,nl,no,om,tj,yv"),
    );
}

fn p1(input: &str) -> usize {
    let graph = parse_input(input);

    let mut component_groups = vec![];
    let mut visited = BTreeSet::new();
    for (curr, neighbors) in &graph {
        if visited.contains(curr) {
            continue;
        }
        visited.insert(*curr);

        for (a, b) in neighbors
            .iter()
            .filter(|n| !visited.contains(*n))
            .tuple_combinations()
        {
            if !graph[a].contains(b) {
                continue;
            }

            component_groups.push([*curr, *a, *b])
        }
    }

    component_groups
        .iter()
        .filter(|group| group.iter().any(|component| component.starts_with("t")))
        .count()
}

fn p2(input: &str) -> String {
    let graph = parse_input(input);

    let mut largest_cliques = graph
        .keys()
        .cloned()
        .tuple_combinations()
        .filter_map(|(a, b)| {
            if graph[a].contains(b) {
                Some(BTreeSet::from([a, b]))
            } else {
                None
            }
        })
        .collect();

    loop {
        let cliques = k_sized_cliques(&largest_cliques, &graph);
        if cliques.is_empty() {
            break;
        }
        largest_cliques = cliques;
    }

    largest_cliques
        .iter()
        .next()
        .unwrap()
        .iter()
        .sorted()
        .join(",")
}

fn k_sized_cliques<'a>(
    k_minus_1_cliques: &BTreeSet<BTreeSet<&'a str>>,
    graph: &BTreeMap<&'a str, BTreeSet<&'a str>>,
) -> BTreeSet<BTreeSet<&'a str>> {
    k_minus_1_cliques
        .par_iter()
        .flat_map(|k_minus_1_clique| {
            let mut all_neighbors = k_minus_1_clique
                .iter()
                .map(|member| &graph[member])
                .collect_vec();
            let mut k_clique_candidates = all_neighbors.pop().unwrap().clone();
            for neighbors in all_neighbors {
                k_clique_candidates.retain(|v| neighbors.contains(v));
            }

            k_clique_candidates
                .iter()
                .map(|candidate| {
                    let mut k_clique = k_minus_1_clique.clone();
                    k_clique.insert(*candidate);
                    k_clique
                })
                .collect_vec()
        })
        .collect()
}

fn parse_input(input: &str) -> BTreeMap<&str, BTreeSet<&str>> {
    let mut map: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for line in input.lines() {
        let (left, right) = line.split("-").collect_tuple().unwrap();
        map.entry(left).or_default().insert(right);
        map.entry(right).or_default().insert(left);
    }
    map
}
