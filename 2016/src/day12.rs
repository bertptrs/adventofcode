use std::io;
use common;

#[derive(Default)]
pub struct Day12 {
    cpu: common::AssemBunnyCPU,
}

impl Day12 {
    pub fn new() -> Day12 {
        Default::default()
    }
}

impl common::Solution for Day12 {

    fn part1(&mut self, input: &mut io::Read) -> String {
        self.cpu.read_instructions(input);
        format!("{}", self.cpu.run())
    }

    fn part2(&mut self, input: &mut io::Read) -> String {
        self.cpu.read_instructions(input);
        self.cpu.registers[2] = 1;
        format!("{}", self.cpu.run())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::Solution;

    const SAMPLE_INPUT: &str = "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

    #[test]
    fn sample_part1() {
        let mut instance = Day12::new();
        assert_eq!("42", instance.part1(&mut SAMPLE_INPUT.as_bytes()))
    }

}
