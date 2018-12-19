use std::io::Read;

use common::Solution;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum OpCode {
    ADDR,
    ADDI,
    MULR,
    MULI,
    BANR,
    BANI,
    BORR,
    BORI,
    SETR,
    SETI,
    GTIR,
    GTRI,
    GTRR,
    EQIR,
    EQRI,
    EQRR,
}

impl OpCode {
    fn valid(self, op: &[i32; 4], before: &[i32; 4], after: &[i32; 4]) -> bool {
        let mut cpu = CPU::new();

        cpu.registers.copy_from_slice(before);
        if let Ok(val) = cpu.execute(self, &op[1..4]) {
            if val == after[op[3] as usize] {
                return true;
            }
        }

        false
    }
}

impl From<&str> for OpCode {
    fn from(name: &str) -> Self {
        match name {
            "addr" => OpCode::ADDR,
            "addi" => OpCode::ADDI,
            "mulr" => OpCode::MULR,
            "muli" => OpCode::MULI,
            "banr" => OpCode::BANR,
            "bani" => OpCode::BANI,
            "borr" => OpCode::BORR,
            "bori" => OpCode::BORI,
            "setr" => OpCode::SETR,
            "seti" => OpCode::SETI,
            "gtir" => OpCode::GTIR,
            "gtri" => OpCode::GTRI,
            "gtrr" => OpCode::GTRR,
            "eqir" => OpCode::EQIR,
            "eqri" => OpCode::EQRI,
            "eqrr" => OpCode::EQRR,
            _ => panic!("Invalid opcode {}", name),
        }
    }
}

const OP_LIST: [OpCode; 16] = [
    OpCode::ADDR,
    OpCode::ADDI,
    OpCode::MULR,
    OpCode::MULI,
    OpCode::BANR,
    OpCode::BANI,
    OpCode::BORR,
    OpCode::BORI,
    OpCode::SETR,
    OpCode::SETI,
    OpCode::GTIR,
    OpCode::GTRI,
    OpCode::GTRR,
    OpCode::EQIR,
    OpCode::EQRI,
    OpCode::EQRR,
];

#[derive(Debug)]
enum CPUErr {
    InvalidRegister(i32),
}

#[derive(Default)]
struct CPU {
    registers: [i32; 6],
}

impl CPU {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn execute(&mut self, op: OpCode, var: &[i32]) -> Result<i32, CPUErr> {
        use self::OpCode::*;
        let res = match op {
            ADDR => self.reg(var[0])? + self.reg(var[1])?,
            ADDI => self.reg(var[0])? + var[1],
            MULR => self.reg(var[0])? * self.reg(var[1])?,
            MULI => self.reg(var[0])? * var[1],
            BANR => self.reg(var[0])? & self.reg(var[1])?,
            BANI => self.reg(var[0])? & var[1],
            BORR => self.reg(var[0])? | self.reg(var[1])?,
            BORI => self.reg(var[0])? | var[1],
            SETR => self.reg(var[0])?,
            SETI => var[0],
            GTRR => (self.reg(var[0])? > self.reg(var[1])?).into(),
            GTIR => (var[0] > self.reg(var[1])?).into(),
            GTRI => (self.reg(var[0])? > var[1]).into(),
            EQRR => (self.reg(var[0])? == self.reg(var[1])?).into(),
            EQIR => (var[0] == self.reg(var[1])?).into(),
            EQRI => (self.reg(var[0])? == var[1]).into(),
        };

        self.registers[var[2] as usize] = res;
        Ok(res)
    }

    fn reg(&self, index: i32) -> Result<i32, CPUErr> {
        if let Some(val) = self.registers.get(index as usize) {
            Ok(*val)
        } else {
            Err(CPUErr::InvalidRegister(index))
        }
    }
}

#[derive(Default)]
pub struct Day19 {
    program: Vec<(OpCode, [i32; 3])>,
    ip: usize,
}

impl Day19 {
    pub fn new() -> Self {
        Default::default()
    }

    fn read_input(&mut self, input: &mut Read) {
        let reader = BufReader::new(input);

        for line in reader.lines() {
            let line = line.unwrap();
            if line.chars().next().unwrap() == '#' {
                self.ip = line.split(' ').last().unwrap().parse().unwrap();
            } else {
                let mut parts = line.split(' ');
                let opcode = OpCode::from(parts.next().unwrap());
                let mut operands = [0; 3];
                for i in 0..3 {
                    operands[i] = parts.next().unwrap().parse().unwrap();
                }

                self.program.push((opcode, operands));
            }
        }
    }
}

impl Solution for Day19 {
    fn part1(&mut self, input: &mut Read) -> String {
        self.read_input(input);

        let mut cpu = CPU::new();

        while (cpu.registers[self.ip] as usize) < self.program.len() {
            let (opcode, operands) = &self.program[cpu.registers[self.ip] as usize];
            cpu.execute(*opcode, operands);
            cpu.registers[self.ip] += 1;
        }

        format!("{}", cpu.registers[0])
    }

    fn part2(&mut self, input: &mut Read) -> String {
        self.read_input(input);

        let mut cpu = CPU::new();
        cpu.registers[0] = 1;

        // This is optimized for my input.
        assert_eq!(self.ip, 3);

        while (cpu.registers[3] as usize) < self.program.len() {
            if cpu.registers[3] == 3 {
                let reg = &mut cpu.registers;
                if reg[5] % reg[1] == 0 {
                    reg[0] += reg[1];
                }
                reg[3] = 12;
            }
            let (opcode, operands) = &self.program[cpu.registers[self.ip] as usize];
            cpu.execute(*opcode, operands);
            cpu.registers[self.ip] += 1;
        }

        format!("{}", cpu.registers[0])
    }
}

#[cfg(test)]
mod tests {
    use day19::Day19;
    use common::Solution;

    const SAMPLE_INPUT: &[u8] = include_bytes!("samples/19.txt");

    #[test]
    fn sample_part1() {
        let mut instance = Day19::new();
        assert_eq!("6", instance.part1(&mut SAMPLE_INPUT));
    }
}
