use std::io::Read;

use common::Solution;
use regex::Regex;
use std::io::BufReader;
use std::io::BufRead;
use common::Point;

type Coordinate = (i64, i64, i64);

#[derive(Default)]
pub struct Day23 {
    bots: Vec<(i64, Coordinate)>
}

impl Day23 {
    pub fn new() -> Self {
        Default::default()
    }

    fn read_input(&mut self, input: &mut Read) {
        let matcher = Regex::new(r"-?\d+").unwrap();
        let reader = BufReader::new(input);

        for line in reader.lines() {
            let line = line.unwrap();

            let mut ints = [0i64;4];
            for (c, i) in matcher.find_iter(&line).zip(ints.iter_mut()) {
                *i = c.as_str().parse().unwrap();
            }

            let pos = (ints[0], ints[1], ints[2]);

            self.bots.push((ints[3], pos));
        }
    }

    fn in_range(&self, pos: Coordinate) -> usize {
        self.bots.iter()
            .filter(|&&(range, other)| other.manhattan(pos) <= range)
            .count()
    }
}

impl Solution for Day23 {
    fn part1(&mut self, input: &mut Read) -> String {
        self.read_input(input);
        self.bots.sort_unstable();
        let (best_range, best_pos) = *self.bots.last().unwrap();

        let result = self.bots.iter().filter(|(_, pos)| pos.manhattan(best_pos) <= best_range)
            .count();

        result.to_string()
    }

    fn part2(&mut self, _input: &mut Read) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use day23::Day23;
    use common::Solution;

    const SAMPLE1_INPUT: &[u8] = include_bytes!("samples/23.1.txt");

    #[test]
    fn sample_part1() {
        let mut instance = Day23::new();
        assert_eq!("7", instance.part1(&mut SAMPLE1_INPUT));
    }
}
