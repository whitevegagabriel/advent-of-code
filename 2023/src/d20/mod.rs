use crate::d20::ModuleType::{Broadcaster, Conjunction, FlipFlop};
use itertools::Itertools;
use num::integer::lcm;
use std::collections::{HashMap, VecDeque};

pub fn solve(problem: &str) -> (usize, usize) {
    let mut modules = problem
        .lines()
        .map(|line| {
            let (parent, children) = line.split(" -> ").collect_tuple().unwrap();
            let children = children.split(", ").collect_vec();

            let (label, module_type) = match &parent[0..1] {
                "%" => (&parent[1..], FlipFlop { state: false }),
                "&" => (
                    &parent[1..],
                    Conjunction {
                        state: HashMap::new(),
                    },
                ),
                _ => (parent, Broadcaster),
            };

            (
                label,
                Module {
                    module_type,
                    children,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let keys = modules.keys().cloned().collect_vec();

    for parent in keys {
        let children = modules[parent].children.clone();
        for child in children {
            if let Some(Module {
                module_type: Conjunction { state },
                ..
            }) = modules.get_mut(child)
            {
                state.insert(parent, false);
            }
        }
    }
    (solve1(modules.clone()), solve2(&modules))
}

fn solve1(mut modules: HashMap<&str, Module>) -> usize {
    let mut pulses_count = HashMap::from([(false, 0_usize), (true, 0_usize)]);

    for _ in 0..1000 {
        let mut queue = VecDeque::from([("button", "broadcaster", false)]);
        while let Some((source, target, signal)) = queue.pop_front() {
            if let Some(count) = pulses_count.get_mut(&signal) {
                *count += 1;
            }

            let Some(target_module) = modules.get_mut(target) else {
                continue;
            };

            let new_value = match &mut target_module.module_type {
                Broadcaster => signal,
                FlipFlop { state } if !signal => {
                    *state = !*state;
                    *state
                }
                Conjunction { state } => {
                    let old_signal = state.get_mut(source).unwrap();
                    *old_signal = signal;
                    !state.values().all(|signal| *signal)
                }
                _ => continue,
            };

            let new_source = target;
            for new_target in &target_module.children {
                queue.push_back((new_source, *new_target, new_value));
            }
        }
    }

    pulses_count.values().product()
}

fn solve2(modules: &HashMap<&str, Module>) -> usize {
    let rx_parent = modules
        .iter()
        .find_map(|(id, module)| {
            if module.children.contains(&"rx") {
                Some(id)
            } else {
                None
            }
        })
        .unwrap();

    let rx_parent_inputs = modules
        .iter()
        .filter_map(|(id, module)| {
            if module.children.contains(rx_parent) {
                Some(id)
            } else {
                None
            }
        })
        .collect_vec();

    rx_parent_inputs
        .into_iter()
        .map(|input| least_steps_to_reach(input, rx_parent, true, modules.clone()) + 1)
        .reduce(lcm)
        .unwrap()
}

fn least_steps_to_reach(
    des_source: &str,
    des_target: &str,
    des_signal: bool,
    mut modules: HashMap<&str, Module>,
) -> usize {
    for i in 0_usize.. {
        let mut queue = vec![("button", "broadcaster", false)];
        while !queue.is_empty() {
            let mut next_queue = vec![];
            for (source, target, signal) in queue {
                if source == des_source && target == des_target && signal == des_signal {
                    return i;
                }

                let Some(target_module) = modules.get_mut(target) else {
                    continue;
                };

                let new_value = match &mut target_module.module_type {
                    Broadcaster => signal,
                    FlipFlop { state } if !signal => {
                        *state = !*state;
                        *state
                    }
                    Conjunction { state } => {
                        let old_signal = state.get_mut(source).unwrap();
                        *old_signal = signal;
                        !state.values().all(|signal| *signal)
                    }
                    _ => continue,
                };

                let new_source = target;
                for new_target in &target_module.children {
                    next_queue.push((new_source, *new_target, new_value));
                }
            }
            queue = next_queue;
        }
    }
    unreachable!()
}

#[derive(Debug, Clone)]
struct Module<'a> {
    module_type: ModuleType<'a>,
    children: Vec<&'a str>,
}

#[derive(Debug, Clone)]
enum ModuleType<'a> {
    Broadcaster,
    FlipFlop { state: bool },
    Conjunction { state: HashMap<&'a str, bool> },
}

/*

 @startuml

state nr: %

nr --> mr
sx --> zh
rk --> dc
rk --> bl
lx --> rs
hx --> bl
hp --> bj
dk --> mr
dk --> lf
hc --> xc
bj --> vv
bl --> rd
jt --> zh
bl --> ks
bl --> kn
bl --> dc
bl --> hc
bl --> zk
zh --> rx
sp --> hz
sp --> bl
rd --> vv
rd --> tp
cg --> dk
rg --> jl
rg --> pv
jl --> js
fb --> vv
fb --> zd
gv --> lx
lr --> vj
lr --> bl
vz --> hc
vz --> bl
kn --> bl
kn --> zk
rj --> mr
rj --> nr
cn --> pv
cn --> sb
rs --> vv
rs --> hp
mr --> qc
mr --> kb
mr --> gc
mr --> vl
mr --> bs
mr --> cg
mr --> lf
rb --> qj
sm --> bv
sm --> vv
dh --> rg
zk --> vz
qj --> xs
qj --> pv
ng --> ql
ng --> pv
vj --> bl
vj --> sp
kb --> zh
sb --> pv
vl --> mr
vl --> cz
dc --> lr
xc --> rk
xc --> bl
cz --> cg
cz --> mr
hz --> bl
hz --> hx
xs --> pv
xs --> cn
js --> ng
cb --> mr
cb --> nc
qb --> vv
gc --> qc
bv --> qb
bv --> vv
broadcaster --> kn
broadcaster --> fb
broadcaster --> ln
broadcaster --> vl
bs --> cb
lf --> gc
nc --> mr
nc --> rj
ln --> pv
ln --> dh
qc --> bs
vv --> zd
vv --> jt
vv --> fb
vv --> hp
vv --> gv
vv --> lx
ks --> zh
ql --> rb
tp --> sm
tb --> vv
pv --> sx
pv --> dh
pv --> jl
pv --> ln
pv --> js
pv --> rb
pv --> ql
zd --> gv

@enduml

 */
