use crate::d24::Register::{W, X, Y, Z};
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take, take_till},
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::collections::HashSet;

pub fn solve(problem: &str) -> (u64, u64) {
    let (_, instructions) = separated_list1(tag("\n"), Instruction::parse)(problem).unwrap();
    (solve1(&instructions), solve2(&instructions))
}

fn solve1(instructions: &[Instruction]) -> u64 {
    // find largest number
    let ordered_digits_to_try = (1..=9).rev().collect_vec();
    find_valid_number(instructions, &ordered_digits_to_try)
}

fn solve2(instructions: &[Instruction]) -> u64 {
    // find smallest number
    let ordered_digits_to_try = (1..=9).collect_vec();
    find_valid_number(instructions, &ordered_digits_to_try)
}

fn find_valid_number(instructions: &[Instruction], ordered_digits_to_try: &[i64]) -> u64 {
    let instruction_groups = instructions.chunks(18).collect_vec();
    let mut no_solution_set = HashSet::new();
    let mut valid_number_le = Vec::new();

    found_valid_num(
        (0, 0),
        &instruction_groups,
        ordered_digits_to_try,
        &mut no_solution_set,
        &mut valid_number_le,
        &mut Alu::new(vec![]),
    );

    valid_number_le
        .into_iter()
        .rev()
        .reduce(|acc, digit| acc * 10 + digit)
        .unwrap() as u64
}

fn found_valid_num(
    (depth, z_prev): (usize, i64),
    instruction_groups: &[&[Instruction]],
    ordered_digits_to_try: &[i64],
    no_solution_set: &mut HashSet<(usize, i64)>,
    current_number: &mut Vec<i64>,
    alu: &mut Alu,
) -> bool {
    // requires knowing that the input performs divisions by 26 that eventually need to make it to 0
    // and experimentally figuring out that this still provides the correct answer
    if 23_i64.saturating_pow((instruction_groups.len() - depth) as u32) < z_prev {
        return false;
    }

    if z_prev > 1_000_000_000_000 {
        println!("z_prev: {z_prev}");
    }

    if depth == 14 {
        return z_prev == 0;
    }

    let instructions = instruction_groups[depth];
    for digit in ordered_digits_to_try {
        alu.z = z_prev;
        alu.inputs.push(*digit);
        alu.execute_all(instructions);

        let new_state = (depth + 1, alu.z);
        if no_solution_set.contains(&new_state) {
            continue;
        }

        let found_solution = found_valid_num(
            new_state,
            instruction_groups,
            ordered_digits_to_try,
            no_solution_set,
            current_number,
            alu,
        );
        if found_solution {
            current_number.push(*digit);
            return true;
        }

        no_solution_set.insert(new_state);
    }
    false
}

#[derive(PartialEq, Debug, Clone)]
struct Alu {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    inputs: Vec<i64>,
}

impl Alu {
    fn new(inputs: Vec<i64>) -> Self {
        Self {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
            inputs: inputs.into_iter().rev().collect(),
        }
    }

    fn execute_all(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            self.execute(instruction)
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Inp(num) => {
                let to_store = self.inputs.pop().unwrap();
                let reg = self.register_for(num);
                *reg = to_store;
            }
            Instruction::Add(num1, num2) => {
                let (num1_literal, num2_literal) = self.two_nums(num1, num2);
                let reg = self.register_for(num1);
                *reg = num1_literal + num2_literal
            }
            Instruction::Mul(num1, num2) => {
                let (num1_literal, num2_literal) = self.two_nums(num1, num2);
                let reg = self.register_for(num1);
                *reg = num1_literal * num2_literal
            }
            Instruction::Div(num1, num2) => {
                let (num1_literal, num2_literal) = self.two_nums(num1, num2);
                let reg = self.register_for(num1);
                *reg = num1_literal / num2_literal
            }
            Instruction::Mod(num1, num2) => {
                let (num1_literal, num2_literal) = self.two_nums(num1, num2);
                let reg = self.register_for(num1);
                *reg = num1_literal % num2_literal
            }
            Instruction::Eql(num1, num2) => {
                let (num1_literal, num2_literal) = self.two_nums(num1, num2);
                let reg = self.register_for(num1);
                *reg = if num1_literal == num2_literal { 1 } else { 0 };
            }
        }
    }

    fn two_nums(&mut self, num1: &Register, num2: &InstructionInput) -> (i64, i64) {
        let num1 = *self.register_for(num1);

        let num2 = match num2 {
            InstructionInput::Register(r) => *self.register_for(r),
            InstructionInput::Number(n) => *n,
        };

        (num1, num2)
    }

    fn register_for(&mut self, num: &Register) -> &mut i64 {
        match num {
            W => &mut self.w,
            X => &mut self.x,
            Y => &mut self.y,
            Z => &mut self.z,
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Inp(Register),
    Add(Register, InstructionInput),
    Mul(Register, InstructionInput),
    Div(Register, InstructionInput),
    Mod(Register, InstructionInput),
    Eql(Register, InstructionInput),
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                take(4_usize),
                separated_list1(tag(" "), InstructionInput::parse),
            )),
            |(ins, args)| {
                let reg = match args[0] {
                    InstructionInput::Register(r) => r,
                    InstructionInput::Number(_) => panic!("this was supposed to be a register"),
                };
                match ins {
                    "inp " => Self::Inp(reg),
                    "add " => Self::Add(reg, args[1]),
                    "mul " => Self::Mul(reg, args[1]),
                    "div " => Self::Div(reg, args[1]),
                    "mod " => Self::Mod(reg, args[1]),
                    "eql " => Self::Eql(reg, args[1]),
                    _ => panic!("invalid instruction: {ins:?}"),
                }
            },
        )(input)
    }
}

#[derive(Debug, Copy, Clone)]
enum InstructionInput {
    Register(Register),
    Number(i64),
}

impl InstructionInput {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            take_till(|c: char| !c.is_alphanumeric() && c != '-'),
            |num| match num {
                "w" => Self::Register(W),
                "x" => Self::Register(X),
                "y" => Self::Register(Y),
                "z" => Self::Register(Z),
                literal => Self::Number(literal.parse().unwrap()),
            },
        )(input)
    }
}

#[derive(Debug, Copy, Clone)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl Register {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(take(1_usize), |reg| {
            println!("reg: {reg}");
            match reg {
                "w" => W,
                "x" => X,
                "y" => Y,
                "z" => Z,
                _ => panic!("not valid"),
            }
        })(input)
    }
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}

#[test]
fn test_execute_instructions() {
    let instructions = vec![
        Instruction::Inp(W),
        Instruction::Mul(W, InstructionInput::Number(5)),
    ];

    let mut alu = Alu::new(vec![1]);

    for instruction in instructions {
        alu.execute(&instruction)
    }

    assert_eq!(
        Alu {
            w: 5,
            x: 0,
            y: 0,
            z: 0,
            inputs: vec![],
        },
        alu
    )
}
