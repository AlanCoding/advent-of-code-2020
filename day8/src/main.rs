use std::env;
use std::fs;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    JMP(isize),
    ACC(isize),
    NOP(isize),
}

#[derive(Debug, Clone)]
struct Program {
    instructions: Vec<Instruction>,
    visited_instructions: Vec<usize>,
    pc: isize,
    acc: isize,
}

impl Program {
    pub fn from_instructions(instructions: Vec<Instruction>) -> Self {
        Program {
            instructions,
            visited_instructions: Vec::new(),
            pc: 0,
            acc: 0,
        }
    }

    pub fn next(&mut self) {
        let i = self.instructions[self.pc as usize];
        match i {
            Instruction::ACC(x) => self.acc += x,
            Instruction::JMP(x) => {
                self.pc += x;
                return;
            }
            Instruction::NOP(_) => {}
        }
        self.visited_instructions.push(self.pc as usize);
        self.pc += 1;
    }

    pub fn run(&mut self) {
        while self.pc >= 0
            && (self.pc as usize) < self.instructions.len()
            && !self.visited_instructions.contains(&(self.pc as usize))
        {
            self.next();
        }
    }

    pub fn change_instr(&mut self, i: usize) -> bool {
        match self.instructions[i] {
            Instruction::JMP(x) => {
                self.instructions[i] = Instruction::NOP(x);
            }
            Instruction::NOP(x) if x == 0 => {
                return false;
            }
            Instruction::NOP(x) if x > 0 => {
                self.instructions[i] = Instruction::JMP(x);
            }
            _ => {
                return false;
            }
        }
        true
    }
}

fn load_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Reading file failed")
}

fn parse_input(data: &str) -> Program {
    Program::from_instructions(
        data.lines()
            .map(|line| {
                let mut split = line.split(" ");
                let i = split.next().unwrap();
                let num = split.next().unwrap().parse::<isize>().unwrap();
                match i {
                    "nop" => Instruction::NOP(num),
                    "jmp" => Instruction::JMP(num),
                    "acc" => Instruction::ACC(num),
                    _ => unreachable!(),
                }
            })
            .collect(),
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() >= 2 {
        args[1].clone()
    } else {
        "input".to_string()
    };
    let input = load_input(&filename);
    let mut p = parse_input(&input);
    p.run();
    println!("part 1: {}", p.acc);

    let p = parse_input(&input);
    let mut x = 0;
    let mut final_p = p.clone();
    while !(final_p.pc as usize == final_p.instructions.len()) {
        final_p = p.clone();
        if !final_p.change_instr(x) {
            x += 1;
            continue;
        };
        final_p.run();
        x += 1;
    }
    println!("part 2: {}", final_p.acc);
}
