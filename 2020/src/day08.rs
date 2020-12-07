use std::convert::Infallible;
use std::io::Read;
use std::str::FromStr;

use crate::common::from_lines;
use crate::Solution;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');

        let command = split.next().unwrap();
        let argument: i32 = split.next().unwrap().parse().unwrap();

        let instruction = match command {
            "nop" => Instruction::Nop(argument),
            "acc" => Instruction::Acc(argument),
            "jmp" => Instruction::Jmp(argument),
            _ => panic!("Unknown command '{}'", command),
        };

        Ok(instruction)
    }
}

fn run_program(program: &[Instruction]) -> (i32, usize) {
    let mut accumulator = 0;
    let mut pc = 0;
    let mut visited = vec![false; program.len()];

    while pc < program.len() {
        if visited[pc] {
            return (accumulator, pc);
        }

        visited[pc] = true;

        match program[pc] {
            Instruction::Acc(n) => accumulator += n,
            Instruction::Jmp(n) => {
                if n > 0 {
                    pc += n as usize;
                } else {
                    pc -= (-n) as usize;
                }
                continue;
            }
            Instruction::Nop(_) => {}
        }

        pc += 1;
    }

    (accumulator, pc)
}

#[derive(Default)]
pub struct Day08;

impl Solution for Day08 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let program: Vec<Instruction> = from_lines(input);
        run_program(&program).0.to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let mut program: Vec<Instruction> = from_lines(input);

        for i in 0..program.len() {
            if let Instruction::Nop(n) = program[i] {
                program[i] = Instruction::Jmp(n);

                let (acc, pc) = run_program(&program);

                if pc == program.len() {
                    return acc.to_string();
                }

                program[i] = Instruction::Nop(n);
            } else if let Instruction::Jmp(n) = program[i] {
                program[i] = Instruction::Nop(n);

                let (acc, pc) = run_program(&program);

                if pc == program.len() {
                    return acc.to_string();
                }

                program[i] = Instruction::Jmp(n);
            }
        }

        panic!("No solution found")
    }
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../samples/08.txt");

    #[test]
    fn sample_part1() {
        test_implementation!(Day08, 1, SAMPLE, 5);
    }

    #[test]
    fn sample_part2() {
        test_implementation!(Day08, 2, SAMPLE, 8);
    }
}
