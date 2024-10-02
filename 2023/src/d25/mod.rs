use std::collections::HashMap;

use itertools::Itertools;
use rand::{thread_rng, Rng};

pub fn solve(problem: &str) -> (usize, usize) {
    let mut node_map = HashMap::new();
    let mut edge_list = vec![];
    let mut initial_node_id_map = HashMap::new();


    for (id, name) in problem.lines().flat_map(|line| line.replace(':', "").split_whitespace().map(String::from).collect_vec()).unique().enumerate() {
        initial_node_id_map.insert(name.clone(), id);
        node_map.insert(id, Node {
            contains: vec![name],
            adjacent: vec![],
        });
    }

    for line in problem.lines() {
        let name= &line[0..3];
        let id = initial_node_id_map[name];
        for adj_name in line[5..].split(' ') {
            let adj_id = initial_node_id_map[adj_name];
            node_map.get_mut(&id).unwrap().adjacent.push(adj_id);
            node_map.get_mut(&adj_id).unwrap().adjacent.push(id);
            edge_list.push((id, adj_id));
        }
    }

    (solve1(&node_map, &edge_list), solve2())
}

fn solve1(initial_node_map: &HashMap<usize, Node>, initial_edge_list: &[(usize, usize)]) -> usize {
    let mut rng = thread_rng();

    let (left, right) = loop {
        let mut node_map = initial_node_map.clone();
        let mut edge_list = initial_edge_list.to_vec();

        while node_map.len() > 2 {
            let random_edge = rng.gen_range(0..edge_list.len());
            let edge = edge_list[random_edge];
            // remove all instances of the chosen edge
            edge_list.retain(|e| e != &edge && e != &(edge.1, edge.0));

            let (keep_id, merge_id) = edge;
            let mut merge = node_map.remove(&merge_id).unwrap();

            // make all nodes on the other side of "merge" point instead to "keep"
            for merge_adj_id in &merge.adjacent {
                if merge_adj_id == &keep_id {
                    continue;
                }

                let merge_adj = node_map.get_mut(merge_adj_id).unwrap();
                merge_adj.adjacent.retain(|id| id != &merge_id);
                if !merge_adj.adjacent.contains(&keep_id) {
                    merge_adj.adjacent.push(keep_id);
                }
            }

            // anything that used to be connected to "merge"
            for edge in edge_list.iter_mut() {
                if edge.0 == merge_id {
                    edge.0 = keep_id
                } else if edge.1 == merge_id {
                    edge.1 = keep_id
                }
            }

            let keep = node_map.get_mut(&keep_id).unwrap();

            // absorb "merge" into "keep"
            keep.contains.append(&mut merge.contains);
            keep.adjacent.append(&mut merge.adjacent);
            keep.adjacent.retain(|id| id != &merge_id && id != &keep_id);
            keep.adjacent.sort();
            keep.adjacent.dedup();
        }

        if edge_list.len() == 3 {
            break node_map.values().cloned().collect_tuple().unwrap();
        }
    };
    // repeat while the partition is greater than 3 edges
    // repeat while there are more than two nodes
    // pick a random edge
    // delete it
    // merge the nodes
    left.contains.len() * right.contains.len()
}

fn solve2() -> usize {
    0
}

#[derive(Hash, Clone, Debug)]
struct Node {
    contains: Vec<String>,
    adjacent: Vec<usize>,
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
