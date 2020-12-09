use aoc_utils::day_runner::run_day;
use std::str::FromStr;
use crate::InstructionType::{Nop, Jmp, Acc};
use aoc_utils::input_helper::read_vector_from_file;

#[derive(Debug, PartialEq, Clone)]
enum InstructionType {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Clone)]
struct Instruction {
    instruction: InstructionType,
    data: i64,
    execution_count: u32,
}

impl Instruction {
    fn mutate(&mut self) {
        match self.instruction {
            Acc => {}
            Jmp => {self.instruction = Nop}
            Nop => {self.instruction = Jmp}
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();

        let itype: InstructionType = match parts[0] {
            "acc" => Acc,
            "jmp" => Jmp,
            "nop" => Nop,
            _ => return Err(())
        };

        let data: i64 = match parts[1].parse() {
            Ok(x) => x,
            Err(..) => return Err(())
        };

        Ok(Instruction {
            instruction: itype,
            data: data,
            execution_count: 0,
        })
    }
}

fn main() {
    run_day(part1, part2)
}

fn part1() {
    let program = load_program();
    let program_length = program.len();

    let (terminated, acc) = program_terminates(program);

    if terminated {
        println!("Somehow the program terminated with {}", acc)
    } else {
        println!("Found infinite loop at with acc {}", acc);
    }
}

fn part2() {
    let program = load_program();

    let mut combinations = 0;
    let mut mutations = Vec::new();
    {
        for pc in 0..program.len() {
            let instruction = &program[pc];

            if instruction.instruction == Jmp || instruction.instruction == Nop  {
                combinations += 1;
                mutations.push(pc);
            }
        }
    }

    for i in mutations {
        let mut clone = program.clone();

        clone[i].mutate();

        let (t,a) = program_terminates(clone);
        if t {
            println!("I terminated by changing {} with acc {}", i, a);
            break;
        }
    }
}

fn load_program() -> Vec<Instruction> {
    let program: Vec<Instruction> = read_vector_from_file("../input/day8/input.txt", "\r\n").unwrap();

    return program;
}

fn program_terminates(mut program: Vec<Instruction>) -> (bool, i64) {
    let mut acc: i64 = 0;
    let mut program_counter: i64 = 0;
    let program_length = program.len() as i64;

    loop {
        if program_counter >= program_length {
            return (true, acc);
        }

        let instruction = &mut program[program_counter as usize];

        if instruction.execution_count == 1 {
            return (false, acc);
        }

        instruction.execution_count += 1;

        match instruction.instruction {
            Acc => acc += instruction.data,
            Jmp => program_counter = (program_counter + instruction.data) - 1,
            Nop => ()
        }

        program_counter += 1;
    }
}