use std::io;
use std::io::prelude::*;
use regex;
use common;

#[derive(Default)]
pub struct Day15 {
    disks: Vec<(i32, i32)>,
}

impl Day15 {
    pub fn new() -> Day15 {
        Default::default()
    }

    fn read_disks(&mut self, input: &mut io::Read) {
        // Note: this implementation assumes the input is sorted.
        let re = regex::Regex::new(r"Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+).")
            .unwrap();
        let reader = io::BufReader::new(input);
        for line in reader.lines() {
            let contents = line.unwrap();
            let groups = re.captures(&contents).unwrap();
            let disk_size: i32 = groups.get(2).unwrap().as_str().parse().unwrap();
            let start_pos: i32 = groups.get(3).unwrap().as_str().parse().unwrap();
            self.disks.push((disk_size, start_pos));
        }
    }

    fn first_pass(&self) -> i32 {
        let mut to_wait = 0;
        let mut multiplier = 1;
        for (i, (size, start)) in self.disks.iter().enumerate() {
            while (i as i32 + start + to_wait + 1) % size != 0 {
                to_wait += multiplier;
            }
            multiplier = common::lcm(*size, multiplier);
        }
        to_wait
    }
}

impl common::Solution for Day15 {
    fn part1(&mut self, input: &mut io::Read) -> String {
        self.read_disks(input);
        format!("{}", self.first_pass())
    }

    fn part2(&mut self, input: &mut io::Read) -> String {
        self.read_disks(input);
        self.disks.push((11, 0));
        format!("{}", self.first_pass())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::Solution;

    const SAMPLE: &str = "Disc #1 has 5 positions; at time=0, it is at position 4.\n\
                          Disc #2 has 2 positions; at time=0, it is at position 1.";

    #[test]
    fn sample_part1() {
        let mut instance = Day15::new();
        assert_eq!("5", instance.part1(&mut SAMPLE.as_bytes()));
    }
}
