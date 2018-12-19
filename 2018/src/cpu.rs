//! Shared CPU module
//!
//! Day 16 and day 19 use the same CPU, so that code is shared (and documented) here.

/// Representation an OpCode for the AoC 2018 CPU.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum OpCode {
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
    /// Check if the given before/after state is valid for a particular OpCode.
    ///
    /// For this, a possible op is represented as an array of integers, where the first
    /// integer (the possible op-code) is ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc_2018::cpu::OpCode;
    ///
    /// assert_eq!(true, OpCode::ADDI.is_valid(&[0, 0, 3, 0], &[0; 6], &[3, 0, 0, 0, 0, 0]))
    /// ```
    pub fn is_valid(self, op: &[i32; 4], before: &[i32; 6], after: &[i32; 6]) -> bool {
        let mut cpu = CPU::new();

        cpu.registers.copy_from_slice(before);
        if let Ok(val) = cpu.execute(self, &op[1..4]) {
            if val == after[op[3] as usize] {
                return true;
            }
        }

        false
    }

    /// Iterator over all possible OpCode values.
    ///
    /// This iterator is backed internally by a static array of all op codes.
    pub fn values() -> impl Iterator<Item=Self> {
        OP_LIST.iter().cloned()
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

/// CPU Error
///
/// This can be returned as an error when the CPU cannot execute
/// an instruction.
#[derive(Debug)]
pub enum CPUErr {
    /// An invalid register was requested, with this register number.
    InvalidRegister(i32),
}

#[derive(Default)]
pub struct CPU {
    /// Internal register state. Can be modified freely.
    pub registers: [i32; 6],
}

impl CPU {
    /// Construct a new CPU instance.
    pub fn new() -> Self {
        Default::default()
    }

    /// Execute an OpCode on the CPU
    ///
    /// This method will return the result of the operation executed, or a
    /// CPUErr when execution fails.
    ///
    /// # Panics
    ///
    /// This function does not check bounds on the operands slice (var) and the output register.
    /// Thus, when the operands slice has less than 3 elements or the target register does not
    /// exist, this function will panic.
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
