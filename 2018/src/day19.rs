use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use common::Solution;
use cpu::OpCode;
use cpu::CPU;

#[derive(Default)]
pub struct Day19 {
    program: Vec<(OpCode, [i32; 3])>,
    ip: usize,
}

impl Day19 {
    pub fn new() -> Self {
        Default::default()
    }

    fn read_input(&mut self, input: &mut dyn Read) {
        let reader = BufReader::new(input);

        for line in reader.lines() {
            let line = line.unwrap();
            if line.chars().next().unwrap() == '#' {
                self.ip = line.split(' ').last().unwrap().parse().unwrap();
            } else {
                let mut parts = line.split(' ');
                let opcode = OpCode::from(parts.next().unwrap());
                let mut operands = [0; 3];
                for (target, part) in operands.iter_mut().zip(parts) {
                    *target = part.parse().unwrap();
                }

                self.program.push((opcode, operands));
            }
        }
    }
}

impl Solution for Day19 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        self.read_input(input);

        let mut cpu = CPU::new();

        while (cpu.registers[self.ip] as usize) < self.program.len() {
            let (opcode, operands) = &self.program[cpu.registers[self.ip] as usize];
            cpu.execute(*opcode, operands).unwrap();
            cpu.registers[self.ip] += 1;
        }

        cpu.registers[0].to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        self.read_input(input);

        let mut cpu = CPU::new();
        cpu.registers[0] = 1;

        // This is optimized for my input.
        assert_eq!(self.ip, 3);

        while (cpu.registers[3] as usize) < self.program.len() {
            if cpu.registers[3] == 1 {
                let reg = &mut cpu.registers;
                reg[0] = (1..=reg[5]).filter(|&x| reg[5] % x == 0).sum();
                reg[3] = 16;
            }
            let (opcode, operands) = &self.program[cpu.registers[self.ip] as usize];
            cpu.execute(*opcode, operands).unwrap();
            cpu.registers[self.ip] += 1;
        }

        cpu.registers[0].to_string()
    }
}

#[cfg(test)]
mod tests {
    use common::Solution;
    use day19::Day19;

    const SAMPLE_INPUT: &[u8] = include_bytes!("samples/19.txt");

    #[test]
    fn sample_part1() {
        let mut instance = Day19::new();
        assert_eq!("7", instance.part1(&mut SAMPLE_INPUT));
    }
}
