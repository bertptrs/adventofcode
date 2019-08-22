use std::collections::HashSet;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use itertools::Itertools;
use itertools::MinMaxResult;
use regex::Regex;

use common::Solution;

type Coordinate = (usize, usize);

#[derive(Default)]
pub struct Day17 {
    clays: HashSet<Coordinate>,
    flowing: HashSet<Coordinate>,
    contained: HashSet<Coordinate>,
    ymin: usize,
    ymax: usize,
}

impl Day17 {
    pub fn new() -> Self {
        Default::default()
    }

    fn read_input(&mut self, input: &mut dyn Read) {
        let matcher = Regex::new(r"(.)=(\d+), (.)=(\d+)\.\.(\d+)").unwrap();
        let reader = BufReader::new(input);

        for line in reader.lines() {
            let line = line.unwrap();
            let captures = matcher.captures(&line).unwrap();

            let fixed: usize = captures[2].parse().unwrap();
            let a: usize = captures[4].parse().unwrap();
            let b: usize = captures[5].parse().unwrap();

            match &captures[1] {
                "x" => {
                    for y in a..=b {
                        self.clays.insert((fixed, y));
                    }
                }
                "y" => {
                    for x in a..=b {
                        self.clays.insert((x, fixed));
                    }
                }
                _ => panic!(),
            }
        }

        match self.clays.iter().map(|(_, y)| y).minmax() {
            MinMaxResult::MinMax(a, b) => {
                self.ymin = *a;
                self.ymax = *b;
            }
            _ => panic!(),
        };
    }

    fn support_end<T>(&mut self, center: usize, range: T, y: usize) -> (usize, bool)
    where
        T: Iterator<Item = usize>,
    {
        let mut prev = center;
        for x in range {
            let pos = (x, y);
            if self.clays.contains(&pos) {
                return (prev, true);
            }

            prev = x;
            let below = (x, y + 1);
            self.descend(below);
            if !self.is_supported(&below) {
                return (x, false);
            }
        }
        unreachable!();
    }

    fn is_supported(&self, pos: &Coordinate) -> bool {
        self.clays.contains(pos) || self.contained.contains(pos)
    }

    fn descend(&mut self, pos: Coordinate) {
        let (x, y) = pos;

        if y > self.ymax
            || self.clays.contains(&pos)
            || self.flowing.contains(&pos)
            || self.contained.contains(&pos)
        {
            return;
        }

        let below = (x, y + 1);

        self.descend(below);

        if self.is_supported(&below) {
            let (right, right_contained) = self.support_end(x, (x + 1).., y);
            let (left, left_contained) = self.support_end(x, (0..x).rev(), y);

            let range = (left..=right).map(|x| (x, y));

            if left_contained && right_contained {
                self.contained.extend(range);
            } else {
                self.flowing.extend(range);
            }
        } else {
            self.flowing.insert(pos);
        }
    }
}

impl Solution for Day17 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        self.read_input(input);

        self.descend((500, 0));

        let result = self.contained.len() + self.flowing.len() - self.ymin;
        result.to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        self.read_input(input);

        self.descend((500, 0));

        self.contained.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use common::Solution;
    use day17::Day17;

    const SAMPLE_INPUT: &[u8] = include_bytes!("samples/17.txt");

    #[test]
    fn sample_part1() {
        let mut instance = Day17::new();
        assert_eq!("57", instance.part1(&mut SAMPLE_INPUT));
    }

    #[test]
    fn sample_part2() {
        let mut instance = Day17::new();
        assert_eq!("29", instance.part2(&mut SAMPLE_INPUT));
    }
}
