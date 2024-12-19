use crate::common::test;
use itertools::Itertools;
use regex::Regex;

const MODULE: &str = module_path!();

#[test]
fn p1_example() {
    test("example", MODULE, p1, String::from("4,6,3,5,6,3,5,2,1,0"));
}

#[test]
fn p1_input() {
    test("input", MODULE, p1, String::from("6,2,7,2,3,1,6,0,5"));
}

#[test]
fn p2_example() {
    test("example2", MODULE, p2, 117440);
}

#[test]
fn p2_input() {
    test("input", MODULE, p2, 236548287712877);
}

fn p1(input: &str) -> String {
    let (mut computer, instructions, _) = parse_input(input);

    while let Some(instruction) = instructions.get(computer.instruction_pointer) {
        computer.execute(instruction);
    }

    computer.output.iter().map(|v| v.to_string()).join(",")
}

fn p2(input: &str) -> usize {
    let (computer, instructions, raw_instructions) = parse_input(input);
    let raw_instructions_rev = raw_instructions.into_iter().rev().collect_vec();

    value_for_register_a_that_outputs_self_instruction(
        0,
        &raw_instructions_rev,
        &instructions,
        &computer,
    )
    .unwrap()
}

fn value_for_register_a_that_outputs_self_instruction(
    prev_a_register: usize,
    raw_instructions: &[usize],
    instructions: &[Instruction],
    original_computer: &Computer,
) -> Option<usize> {
    let Some(target_output) = raw_instructions.first() else {
        return Some(prev_a_register);
    };
    let curr_a_register_shifted_3_bits = prev_a_register * 8;
    for lower_3_bits in 0_usize..8 {
        let mut computer = original_computer.clone();
        computer.register_a = curr_a_register_shifted_3_bits + lower_3_bits;
        while let Some(instruction) = instructions.get(computer.instruction_pointer) {
            computer.execute(instruction);
            if !computer.output.is_empty() {
                break;
            }
        }
        if &computer.output[0] == target_output {
            let maybe_register_a = value_for_register_a_that_outputs_self_instruction(
                curr_a_register_shifted_3_bits + lower_3_bits,
                &raw_instructions[1..],
                instructions,
                original_computer,
            );
            if maybe_register_a.is_some() {
                return maybe_register_a;
            }
        }
    }
    None
}

fn parse_input(input: &str) -> (Computer, Vec<Instruction>, Vec<usize>) {
    let re = Regex::new(
        r"Register A: (\d+)
Register B: (\d+)
Register C: (\d+)

Program: (.+)",
    )
    .unwrap();

    let caps = re.captures(input).unwrap();
    let register_a = caps[1].parse().unwrap();
    let register_b = caps[2].parse().unwrap();
    let register_c = caps[3].parse().unwrap();
    let raw_instructions = caps[4]
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();
    let instructions = raw_instructions
        .iter()
        .tuples()
        .map(|(instruction_code, operand_code)| Instruction::new(*instruction_code, *operand_code))
        .collect();

    (
        Computer {
            register_a,
            register_b,
            register_c,
            output: vec![],
            instruction_pointer: 0,
        },
        instructions,
        raw_instructions,
    )
}

#[derive(Debug, Clone)]
struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    output: Vec<usize>,
    instruction_pointer: usize,
}

impl Computer {
    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Adv(combo_operand) => {
                let value = self.evaluate(combo_operand);
                self.register_a /= 2_usize.pow(value as u32);
            }
            Instruction::Bxl(LiteralOperand(value)) => {
                self.register_b ^= *value;
            }
            Instruction::Bst(combo_operand) => {
                let value = self.evaluate(combo_operand);
                self.register_b = value % 8;
            }
            Instruction::Jnz(LiteralOperand(value)) => {
                if self.register_a != 0 {
                    self.instruction_pointer = *value;

                    // do not increase pointer
                    return;
                }
            }
            Instruction::Bxc => self.register_b ^= self.register_c,
            Instruction::Out(combo_operand) => {
                let value = self.evaluate(combo_operand);

                self.output.push(value % 8);
            }
            Instruction::Bdv(combo_operand) => {
                let value = self.evaluate(combo_operand);
                self.register_b = self.register_a / 2_usize.pow(value as u32);
            }
            Instruction::Cdv(combo_operand) => {
                let value = self.evaluate(combo_operand);
                self.register_c = self.register_a / 2_usize.pow(value as u32);
            }
        }

        self.instruction_pointer += 1;
    }

    fn evaluate(&self, combo_operand: &ComboOperand) -> usize {
        match combo_operand {
            ComboOperand::Literal(value) => *value,
            ComboOperand::RegisterA => self.register_a,
            ComboOperand::RegisterB => self.register_b,
            ComboOperand::RegisterC => self.register_c,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Adv(ComboOperand),
    Bxl(LiteralOperand),
    Bst(ComboOperand),
    Jnz(LiteralOperand),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

impl Instruction {
    fn new(instruction_code: usize, operand_code: usize) -> Self {
        match instruction_code {
            0 => Self::Adv(ComboOperand::new(operand_code)),
            1 => Self::Bxl(LiteralOperand(operand_code)),
            2 => Self::Bst(ComboOperand::new(operand_code)),
            3 => Self::Jnz(LiteralOperand(operand_code)),
            4 => Self::Bxc,
            5 => Self::Out(ComboOperand::new(operand_code)),
            6 => Self::Bdv(ComboOperand::new(operand_code)),
            7 => Self::Cdv(ComboOperand::new(operand_code)),
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct LiteralOperand(usize);

#[derive(Debug)]
enum ComboOperand {
    Literal(usize),
    RegisterA,
    RegisterB,
    RegisterC,
}

impl ComboOperand {
    fn new(operand_code: usize) -> Self {
        match operand_code {
            0_usize..=3_usize => ComboOperand::Literal(operand_code),
            4 => ComboOperand::RegisterA,
            5 => ComboOperand::RegisterB,
            6 => ComboOperand::RegisterC,
            _ => panic!(),
        }
    }
}
