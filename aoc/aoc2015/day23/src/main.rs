use std::char;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    HLF(char),        // half
    TPL(char),        // triple
    INC(char),        // increment
    JMP(isize),       // jump
    JIE(char, isize), // jump if even
    JIO(char, isize), // jump if one
}

#[derive(Debug, PartialEq, Eq)]
struct ExecutionError(Instruction, String);

impl Error for ExecutionError {}

impl Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

type ExecutionResult<T> = Result<T, ExecutionError>;

struct Computer {
    reg_a: usize,
    reg_b: usize,
}

impl Computer {
    fn new(reg_a: usize, reg_b: usize) -> Self {
        Self { reg_a, reg_b }
    }

    fn target_register(&mut self, reg: char) -> Option<&mut usize> {
        match reg {
            'a' => Some(&mut self.reg_a),
            'b' => Some(&mut self.reg_b),
            _ => None,
        }
    }

    fn run(&mut self, program: &Vec<Instruction>) -> ExecutionResult<(usize, usize)> {
        let mut pc = 0;

        while let Some(instruction) = program.get(pc) {
            let mut step = 1;
            match instruction {
                Instruction::HLF(reg) => {
                    let target = self.target_register(*reg).unwrap();
                    *target /= 2;
                }
                Instruction::TPL(reg) => {
                    let target = self.target_register(*reg).unwrap();
                    *target *= 3;
                }
                Instruction::INC(reg) => {
                    let target = self.target_register(*reg).unwrap();
                    *target += 1;
                }
                Instruction::JMP(count) => {
                    step = *count;
                }
                Instruction::JIE(reg, count) => {
                    let target = self.target_register(*reg).unwrap();
                    if *target % 2 == 0 {
                        step = *count;
                    }
                }
                Instruction::JIO(reg, count) => {
                    let target = self.target_register(*reg).unwrap();
                    if *target == 1 {
                        step = *count;
                    }
                }
            }

            if pc as isize + step < 0 {
                return Err(ExecutionError(
                    instruction.clone(),
                    String::from("invalid step"),
                ));
            }
            pc = (pc as isize + step) as usize;
        }

        // Ok((a, b))

        Ok((self.reg_a, self.reg_b))
    }
}

fn main() {
    let file = File::open("input.txt").expect("Failed to open input.txt");
    let buffer = BufReader::new(file);

    let mut program = Vec::new();
    for line in buffer.lines() {
        let line = line.expect("Failed to read line");

        if let Some(instruction) = parse_line(&line) {
            program.push(instruction)
        }
    }

    let mut computer = Computer::new(0, 0);
    let result = computer.run(&program);
    println!("result = {:?}", result);

    let mut computer = Computer::new(1, 0);
    let result = computer.run(&program);
    println!("result = {:?}", result);
}

fn parse_line(line: &str) -> Option<Instruction> {
    let mut parts = line.trim().split(' ');
    let cmd = parts.next()?.trim();
    let reg: Option<char> = match cmd {
        "hlf" | "tpl" | "inc" => {
            let reg = parts.next()?.trim().parse().unwrap();
            Some(reg)
        }
        "jie" | "jio" => {
            let reg = parts.next()?.replace(",", "").trim().parse().unwrap();
            Some(reg)
        }
        _ => None,
    };

    match cmd {
        "hlf" => Some(Instruction::HLF(reg?)),
        "tpl" => Some(Instruction::TPL(reg?)),
        "inc" => Some(Instruction::INC(reg?)),
        "jmp" => {
            let count: isize = parts.next()?.trim().parse().unwrap();
            Some(Instruction::JMP(count))
        }
        "jie" => {
            let count: isize = parts.next()?.trim().parse().unwrap();
            Some(Instruction::JIE(reg?, count))
        }
        "jio" => {
            let count = parts.next()?.trim().parse().unwrap();
            Some(Instruction::JIO(reg?, count))
        }
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parse_line("hlf a"), Some(Instruction::HLF('a')));
        assert_eq!(parse_line("tpl a"), Some(Instruction::TPL('a')));
        assert_eq!(parse_line("inc a"), Some(Instruction::INC('a')));
        assert_eq!(parse_line("jmp -2"), Some(Instruction::JMP(-2)));
        assert_eq!(parse_line("jio a, +2"), Some(Instruction::JIO('a', 2)));
        assert_eq!(parse_line("jie a, -2"), Some(Instruction::JIE('a', -2)));
    }

    #[test]
    fn test_run() {
        let program = vec![
            Instruction::INC('a'),
            Instruction::JIO('a', 2),
            Instruction::TPL('a'),
            Instruction::INC('a'),
        ];

        let mut computer = Computer::new(0, 0);
        let result = computer.run(&program);
        assert_eq!(result, Ok((2, 0)));
    }
}
