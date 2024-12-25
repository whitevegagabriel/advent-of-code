use crate::common::{test, test_with_params};
use itertools::Itertools;
use std::collections::HashMap;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, 2024);
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, 49520947122770);
}

#[test]
fn p2_input() {
    test_with_params(
        "input",
        MODULE,
        p2,
        4,
        String::from("gjc,gvm,qjj,qsb,wmp,z17,z26,z39"),
    );
}

fn p1(input: &str) -> usize {
    let (initializer, graph, mut gate_map) = parse_input(input);
    let mut outputs = HashMap::new();

    for (key, value) in initializer {
        trigger_switch(key, value, &mut outputs, &mut gate_map, &graph)
    }

    let mut ans = 0;
    let mut mult = 1_usize;
    for i in 0.. {
        let key = format!("z{i:02}");
        let Some(value) = outputs.get(key.as_str()) else {
            break;
        };

        ans += value * mult;
        mult *= 2;
    }
    ans
}

fn p2(input: &str, _pairs_needing_swapped: usize) -> String {
    let (_, graph, gate_map) = parse_input(input);

    let mut swapped = vec![];
    for i in 1.. {
        let x = format!("x{i:02}");
        let z = format!("z{i:02}");

        if !graph.contains_key(x.as_str()) {
            break;
        }

        let xor1 = graph[x.as_str()]
            .iter()
            .find(|out| gate_map[*out].logic_operator == LogicOperator::Xor)
            .unwrap();

        let and1 = graph[x.as_str()]
            .iter()
            .find(|out| gate_map[*out].logic_operator == LogicOperator::And)
            .unwrap();

        let Some(xor2) = graph[xor1].iter().find(|s| {
            let gate = &gate_map[*s];
            (gate.left == *xor1 || gate.right == *xor1) && gate.logic_operator == LogicOperator::Xor
        }) else {
            swapped.push(xor1.to_string());
            swapped.push(and1.to_string());
            continue;
        };

        if *xor2 != z {
            swapped.push(z);
            swapped.push(xor2.to_string());
        }
    }

    swapped.sort();
    swapped.iter().join(",")
}

fn parse_input(input: &str) -> (HashMap<&str, usize>, Graph, HashMap<&str, LogicGate>) {
    let (init_str, gate_str) = input.split("\n\n").collect_tuple().unwrap();

    let initializer = init_str
        .lines()
        .map(|line| {
            let (key, value_str) = line.split(": ").collect_tuple().unwrap();
            (key, value_str.parse::<usize>().unwrap())
        })
        .collect();

    let mut graph = HashMap::new();
    let mut gate_map = HashMap::new();

    for line in gate_str.lines() {
        let (left, logic_str, right, _, result) = line.split(" ").collect_tuple().unwrap();
        let logic_operator = match logic_str {
            "AND" => LogicOperator::And,
            "OR" => LogicOperator::Or,
            "XOR" => LogicOperator::Xor,
            _ => panic!(),
        };
        let gate = LogicGate {
            logic_operator,
            left,
            left_value: None,
            right,
            right_value: None,
        };
        graph.entry(left).or_insert(vec![]).push(result);
        graph.entry(right).or_insert(vec![]).push(result);
        gate_map.insert(result, gate);
    }
    (initializer, graph, gate_map)
}

fn compact(prefix: &str, outputs: &HashMap<&str, usize>) -> usize {
    let mut ans = 0;
    let mut mult = 1_usize;
    for i in 0.. {
        let key = format!("{prefix}{i:02}");
        let Some(value) = outputs.get(key.as_str()) else {
            break;
        };

        ans += value * mult;
        mult *= 2;
    }
    ans
}

fn trigger_switch<'b, 'a: 'b>(
    source: &'a str,
    value: usize,
    outputs: &'b mut HashMap<&'a str, usize>,
    gate_map: &'b mut HashMap<&'a str, LogicGate>,
    graph: &Graph<'a>,
) {
    let Some(destinations) = graph.get(source) else {
        return;
    };
    for dest in destinations {
        let gate = gate_map.get_mut(dest).unwrap();
        if gate.left == source {
            gate.left_value = Some(value);
        } else if gate.right == source {
            gate.right_value = Some(value);
        } else {
            panic!();
        }

        if let Some(result) = gate.evaluate() {
            outputs.insert(dest, result);
            trigger_switch(dest, result, outputs, gate_map, graph)
        }
    }
}

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

#[derive(Debug)]
struct LogicGate<'a> {
    logic_operator: LogicOperator,
    left: &'a str,
    left_value: Option<usize>,
    right: &'a str,
    right_value: Option<usize>,
}

impl LogicGate<'_> {
    fn evaluate(&self) -> Option<usize> {
        if let Some(left) = self.left_value
            && let Some(right) = self.right_value
        {
            Some(match self.logic_operator {
                LogicOperator::And => left & right,
                LogicOperator::Or => left | right,
                LogicOperator::Xor => left ^ right,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum LogicOperator {
    And,
    Or,
    Xor,
}
