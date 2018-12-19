use std::collections::HashSet;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use regex::Regex;

use common::Solution;
use cpu::OpCode;
use cpu::CPU;

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

    fn read(&mut self, reader: &mut BufRead, target: &mut [i32]) -> bool {
        self.buf.clear();
        if reader.read_line(&mut self.buf).is_err() {
            return false;
        }

        if let Some(captures) = self.matcher.captures(&self.buf) {
            for (target, cap) in target.iter_mut().zip(captures.iter().skip(1)) {
                *target = cap.unwrap().as_str().parse().unwrap();
            }

            true
        } else {
            false
        }
    }

    fn determine_options(&mut self, mut reader: &mut BufReader<&mut Read>) -> [HashSet<OpCode>; 16] {
        let mut mappings: [HashSet<OpCode>; 16] = [
            HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new(),
            HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new(),
            HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new(),
            HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new(),
        ];
        let mut before = [0; 6];
        let mut op = [0; 4];
        let mut after = [0; 6];
        while self.read(&mut reader, &mut before[..4]) {
            self.read(&mut reader, &mut op);
            self.read(&mut reader, &mut after[..4]);
            reader.read_line(&mut self.buf).unwrap_or(0);

            if mappings[op[0] as usize].is_empty() {
                mappings[op[0] as usize].extend(OpCode::values()
                    .filter(|x| x.is_valid(&op, &before, &after)));
            } else {
                for option in OpCode::values()
                    .filter(|x| !x.is_valid(&op, &before, &after)) {
                    mappings[op[0] as usize].remove(&option);
                }
                continue;
            }
        }
        mappings
    }

    fn determine_mapping(&self, mut options: [HashSet<OpCode>; 16]) -> [OpCode; 16] {
        let mut mapping: [Option<OpCode>; 16] = [None; 16];
        let mut determined = 0;

        while determined < mapping.len() {
            for op in mapping.iter() {
                if let Some(op) = op {
                    for option in options.iter_mut() {
                        option.remove(op);
                    }
                }
            }

            for (idx, option) in options.iter_mut().enumerate() {
                if option.len() == 1 {
                    mapping[idx] = option.drain().next();
                    option.clear();
                    determined += 1;
                }
            }
        }

        let mut actual_mapping = [OpCode::ADDI; 16];
        for (i, op) in mapping.iter().enumerate() {
            actual_mapping[i] = op.unwrap();
        }

        actual_mapping
    }
}

impl Default for Day16 {
    fn default() -> Self {
        Self::new()
    }
}

impl Solution for Day16 {
    fn part1(&mut self, input: &mut Read) -> String {
        let mut reader = BufReader::new(input);

        let mut before = [0; 6];
        let mut op = [0; 4];
        let mut after = [0; 6];
        let mut counter = 0;

        while self.read(&mut reader, &mut before[..4]) {
            self.read(&mut reader, &mut op);
            self.read(&mut reader, &mut after[..4]);
            reader.read_line(&mut self.buf).unwrap_or(0);

            let valid = OpCode::values()
                .filter(|x| x.is_valid(&op, &before, &after))
                .count();

            if valid >= 3 {
                counter += 1;
            }
        }
        format!("{}", counter)
    }

    fn part2(&mut self, input: &mut Read) -> String {
        let mut reader = BufReader::new(input);

        let mappings = self.determine_options(&mut reader);
        let mapping = self.determine_mapping(mappings);

        let mut op = [0; 4];
        // Skip a line
        reader.read_line(&mut self.buf).unwrap();

        let mut cpu = CPU::new();

        while self.read(&mut reader, &mut op) {
            cpu.execute(mapping[op[0] as usize], &op[1..4]).unwrap();
        }

        format!("{}", cpu.registers[0])
    }
}


#[cfg(test)]
mod tests {
    use common::Solution;
    use day16::Day16;

    #[test]
    fn sample_part1() {
        let input: &[u8] = include_bytes!("samples/16.txt");
        let mut instance = Day16::new();
        assert_eq!("1", instance.part1(&mut input.as_ref()));
    }
}
