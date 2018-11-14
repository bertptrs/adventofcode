use std::io;
use common;

#[derive(Default)]
pub struct Day23 {
    cpu: common::AssemBunnyCPU,
}

impl Day23 {
    pub fn new() -> Day23 {
        Default::default()
    }
}

impl common::Solution for Day23 {

    fn part1(&mut self, input: &mut io::Read) -> String {
        self.cpu.read_instructions(input);
        self.cpu.registers[0] = 7;
        format!("{}", self.cpu.run())
    }

    fn part2(&mut self, input: &mut io::Read) -> String {
        self.cpu.read_instructions(input);
        self.cpu.registers[0] = 12;
        format!("{}", self.cpu.run())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::Solution;

    const SAMPLE_INPUT: &str = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";

    #[test]
    fn sample_part1() {
        let mut instance = Day23::new();
        assert_eq!("3", instance.part1(&mut SAMPLE_INPUT.as_bytes()))
    }

}
