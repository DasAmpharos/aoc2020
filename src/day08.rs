use std::collections::HashSet;
use std::fs;

use substring::Substring;

use crate::day08::Sign::PLUS;
use std::borrow::BorrowMut;

pub fn find_solution() {
    let contents = fs::read_to_string("input/day08.txt")
        .expect("Something went wrong reading the file");

    let mut instructions = contents.lines()
        .map(|it| Instruction::parse(it))
        .collect::<Vec<Instruction>>();

    println!("{}", p1_execute(&instructions));
    println!("{:?}", p2_execute(&instructions));
}

fn p1_execute(instructions: &Vec<Instruction>) -> i32 {
    execute(instructions).0
}

fn p2_execute(instructions: &Vec<Instruction>) -> Option<i32> {
    let mut acc: Option<i32> = None;
    let corrupted = (0..instructions.len())
        .filter(|it| {
            let instruction = &instructions[*it];
            instruction.op == Op::NOP || instruction.op == Op::JMP
        })
        .collect::<Vec<usize>>();

    for idx in corrupted {
        let mut modified = instructions.iter()
            .cloned().collect::<Vec<Instruction>>();

        let instruction = &modified[idx];
        let modified_instruction = swap_corrupted(instruction);
        modified[idx] = modified_instruction;

        let result = execute(&modified);
        if !result.2 {
            acc = Some(result.0);
            break;
        }
    }
    acc
}

fn execute(instructions: &Vec<Instruction>) -> (i32, HashSet<usize>, bool) {
    let mut pc = 0;
    let mut acc = 0;
    let mut looped = false;
    let mut visited = HashSet::<usize>::new();
    let mut corrupted = HashSet::<usize>::new();

    loop {
        if pc >= instructions.len() { break; }
        if visited.contains(&pc) {
            looped = true;
            break;
        }
        let instruction = &instructions[pc];

        visited.insert(pc);
        if instruction.op == Op::NOP || instruction.op == Op::JMP {
            corrupted.insert(pc);
        }

        match instruction.op {
            Op::ACC => {
                match instruction.operand.0 {
                    Sign::PLUS => {
                        acc += instruction.operand.1
                    }
                    Sign::MINUS => {
                        acc -= instruction.operand.1
                    }
                }
            }
            Op::JMP => {
                match instruction.operand.0 {
                    Sign::PLUS => {
                        pc += (instruction.operand.1 as usize);
                        continue;
                    }
                    Sign::MINUS => {
                        pc -= (instruction.operand.1 as usize);
                        continue;
                    }
                }
            }
            Op::NOP => {}
        }
        pc += 1;
    }
    (acc, corrupted, looped)
}

fn parse_operand(operand: &str) -> (Sign, i32) {
    let sign = Sign::parse(operand.substring(0, 1));
    let value = operand.substring(1, operand.len()).parse::<i32>();
    if let Err(e) = value {
        panic!("{:?}", e);
    }
    (sign, value.unwrap())
}

fn swap_corrupted(instruction: &Instruction) -> Instruction {
    Instruction {
        op: match instruction.op {
            Op::JMP => Op::NOP,
            Op::NOP => Op::JMP,
            Op::ACC => panic!("ACC ops are never corrupted")
        },
        operand: (
            instruction.operand.0,
            instruction.operand.1
        ),
    }
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    op: Op,
    operand: (Sign, i32),
}

impl Instruction {
    fn parse(instruction: &str) -> Instruction {
        let split = instruction.split_whitespace()
            .collect::<Vec<&str>>();
        if split.len() != 2 {
            panic!("invalid instruction: {}", instruction);
        }

        Instruction {
            op: Op::parse(split[0]),
            operand: parse_operand(split[1]),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Op {
    ACC,
    JMP,
    NOP,
}

impl Op {
    fn parse(op: &str) -> Op {
        match op {
            "acc" => Op::ACC,
            "jmp" => Op::JMP,
            "nop" => Op::NOP,
            _ => panic!("invalid op: {}", op)
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Sign {
    PLUS,
    MINUS,
}

impl Sign {
    fn parse(sign: &str) -> Sign {
        match sign {
            "+" => Sign::PLUS,
            "-" => Sign::MINUS,
            _ => panic!("invalid sign: {}", sign)
        }
    }
}
