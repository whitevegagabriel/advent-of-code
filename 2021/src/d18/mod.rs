use itertools::Itertools;
use std::{collections::LinkedList, rc::Rc};

pub fn solve(problem: &[&str]) -> (u64, u64) {
    // The addition section is a bit easier to solve in a flat structure because travelling up and
    // down trees arbitrarily is hard
    // I originally solved this using Vec, then switched to LinkedList to see what happened with
    // performance. Performance is slightly worse. Oh well!
    let snailfish_numbers = problem
        .iter()
        .map(|number| {
            number
                .chars()
                .filter_map(|c| match c {
                    '[' => Some(SnailItem::Open),
                    ']' => Some(SnailItem::Close),
                    ',' => None,
                    _ => Some(SnailItem::Value(c.to_digit(10).unwrap() as u64)),
                })
                .collect::<LinkedList<_>>()
        })
        .collect_vec();
    (solve1(snailfish_numbers.clone()), solve2(snailfish_numbers))
}

fn solve1(snailfish_numbers: Vec<LinkedList<SnailItem>>) -> u64 {
    let mut final_number = snailfish_numbers
        .into_iter()
        .reduce(add_snailfish_numbers)
        .unwrap();

    let tree = SnailFishNumberTree::parse(&mut final_number);
    tree.magnitude()
}

fn solve2(snailfish_numbers: Vec<LinkedList<SnailItem>>) -> u64 {
    snailfish_numbers
        .into_iter()
        .tuple_combinations()
        .map(|(num1, num2)| {
            let mut addition1 = add_snailfish_numbers(num1.clone(), num2.clone());
            let sum1 = SnailFishNumberTree::parse(&mut addition1).magnitude();
            let mut addition2 = add_snailfish_numbers(num2, num1);
            let sum2 = SnailFishNumberTree::parse(&mut addition2).magnitude();
            sum1.max(sum2)
        })
        .max()
        .unwrap()
}

#[derive(Debug)]
enum SnailFishNumberTree {
    Leaf(u64),
    Branch {
        left: Rc<SnailFishNumberTree>,
        right: Rc<SnailFishNumberTree>,
    },
}

impl SnailFishNumberTree {
    fn parse(num: &mut LinkedList<SnailItem>) -> Self {
        match num.pop_front().unwrap() {
            SnailItem::Open => {
                let left = Self::parse(num);
                let right = Self::parse(num);

                let close = num.pop_front().unwrap();
                assert_eq!(SnailItem::Close, close);

                SnailFishNumberTree::Branch {
                    left: Rc::new(left),
                    right: Rc::new(right),
                }
            }
            SnailItem::Value(v) => SnailFishNumberTree::Leaf(v),
            SnailItem::Close => panic!("expected something else, I guess"),
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            SnailFishNumberTree::Leaf(v) => *v,
            SnailFishNumberTree::Branch { left, right } => {
                left.magnitude() * 3 + right.magnitude() * 2
            }
        }
    }
}

#[allow(unused)]
fn display(num: &[SnailItem]) {
    let num = num
        .iter()
        .map(|item| match item {
            SnailItem::Open => "[".to_string(),
            SnailItem::Close => "]".to_string(),
            SnailItem::Value(v) => v.to_string(),
        })
        .join("");
    println!("{num}");
}

fn add_snailfish_numbers(
    mut num1: LinkedList<SnailItem>,
    mut num2: LinkedList<SnailItem>,
) -> LinkedList<SnailItem> {
    num1.push_front(SnailItem::Open);
    num1.append(&mut num2);
    num1.push_back(SnailItem::Close);

    let mut combined = num1;
    while let Some(action) = next_action(&combined) {
        match action {
            SnailAction::Explode(idx) => explode_at(&mut combined, idx),
            SnailAction::Split(idx) => split_at(&mut combined, idx),
        }
    }
    combined
}

fn explode_at(snailfish_num: &mut LinkedList<SnailItem>, idx: usize) {
    // |   |    idx     |             |   |
    // | [ | left_value | right_value | ] |
    let left = snailfish_num;
    let mut middle = left.split_off(idx - 1);
    let mut right = middle.split_off(4);

    middle.pop_front();
    let left_value = match middle.pop_front().unwrap() {
        SnailItem::Value(v) => v,
        _ => panic!("this was supposed to be a raw value"),
    };
    let right_value = match middle.pop_front().unwrap() {
        SnailItem::Value(v) => v,
        _ => panic!("this was supposed to be a raw value"),
    };

    for item in left.iter_mut().rev() {
        if let SnailItem::Value(v) = item {
            *v += left_value;
            break;
        }
    }
    for item in right.iter_mut() {
        if let SnailItem::Value(v) = item {
            *v += right_value;
            break;
        }
    }

    left.push_back(SnailItem::Value(0));
    left.append(&mut right);
}

fn split_at(snailfish_num: &mut LinkedList<SnailItem>, idx: usize) {
    let left = snailfish_num;
    let mut right = left.split_off(idx);

    let value = match right.pop_front().unwrap() {
        SnailItem::Value(v) => v,
        _ => panic!("this was supposed to be a raw value"),
    };

    let new_left_value = value / 2;
    let new_right_value = value - new_left_value;

    left.push_back(SnailItem::Open);
    left.push_back(SnailItem::Value(new_left_value));
    left.push_back(SnailItem::Value(new_right_value));
    left.push_back(SnailItem::Close);
    left.append(&mut right);
}

fn next_action(snailfish_num: &LinkedList<SnailItem>) -> Option<SnailAction> {
    let mut open_count = 0;
    let mut maybe_split = None;
    for (idx, item) in snailfish_num.iter().enumerate() {
        match item {
            SnailItem::Open => {
                open_count += 1;
                if open_count > 4 {
                    return Some(SnailAction::Explode(idx + 1));
                }
            }
            SnailItem::Close => open_count -= 1,
            SnailItem::Value(v) => {
                if v > &9 && maybe_split.is_none() {
                    maybe_split = Some(SnailAction::Split(idx))
                }
            }
        };
    }
    maybe_split
}

#[derive(Debug)]
enum SnailAction {
    Explode(usize),
    Split(usize),
}

#[derive(Debug, Clone, PartialEq)]
enum SnailItem {
    Open,
    Close,
    Value(u64),
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
