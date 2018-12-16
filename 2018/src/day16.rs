use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use regex::Regex;

use common::Solution;

#[derive(Copy, Clone, Debug)]
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

enum CPUErr {
    InvalidRegister(i32),
}

#[derive(Default)]
struct CPU {
    registers: [i32; 4],
}

impl CPU {
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

pub struct Day16 {
    matcher: Regex,
    buf: String,
}

impl Day16 {
    pub fn new() -> Self {
        Day16 {
            matcher: Regex::new(r"(\d+),? (\d+),? (\d+),? (\d+)").unwrap(),
            buf: String::new(),
        }
    }

    fn read(&mut self, reader: &mut BufRead, target: &mut [i32; 4]) -> bool {
        self.buf.clear();
        if let Err(_) = reader.read_line(&mut self.buf) {
            return false;
        }

        if let Some(captures) = self.matcher.captures(&self.buf) {
            for i in 0..4 {
                target[i] = captures[i + 1].parse().unwrap();
            }

            true
        } else {
            false
        }
    }
}

impl Solution for Day16 {
    fn part1(&mut self, input: &mut Read) -> String {
        let mut reader = BufReader::new(input);

        let mut before = [0; 4];
        let mut op = [0; 4];
        let mut after = [0; 4];

        let mut cpu: CPU = Default::default();
        let mut counter = 0;

        while self.read(&mut reader, &mut before) {
            self.read(&mut reader, &mut op);
            self.read(&mut reader, &mut after);
            reader.read_line(&mut self.buf).unwrap_or(0);

            let mut valid = 0;

            for option in &OP_LIST {
                cpu.registers = before;
                if let Ok(val) = cpu.execute(*option, &op[1..4]) {
                    if val == after[op[3] as usize] {
                        valid += 1;
                    }
                }
            }

            if valid >= 3 {
                counter += 1;
            }
        }
        format!("{}", counter)
    }

    fn part2(&mut self, _input: &mut Read) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use common::Solution;
    use day16::Day16;

    #[test]
    fn sample_part1() {
        let input: &[u8] = b"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]
";
        let mut instance = Day16::new();
        assert_eq!("1", instance.part1(&mut input.as_ref()));
    }
}
