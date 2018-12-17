use std::io::Read;

use common::Solution;
use std::collections::HashSet;
use regex::Regex;
use std::io::BufReader;
use std::io::BufRead;
use itertools::Itertools;
use itertools::MinMaxResult;

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

    fn read_input(&mut self, input: &mut Read) {
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
                },
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
            },
            _ => panic!(),
        };
    }

    fn descend(&mut self, pos: Coordinate) {
        let (x, y) = pos;

        if y > self.ymax || self.clays.contains(&pos) || self.flowing.contains(&pos) || self.contained.contains(&pos) {
            return;
        }

        let below = (x, y + 1);

        self.descend(below);

        if self.clays.contains(&below) || self.contained.contains(&below) {
            let mut contained = true;

            let mut layer = vec![pos];

            let mut nx = x + 1;
            loop {
                let npos = (nx, y);
                if self.clays.contains(&npos) {
                    break;
                }
                layer.push(npos);

                let nbelow = (nx, y + 1);
                self.descend(nbelow);
                if !self.clays.contains(&nbelow) && !self.contained.contains(&nbelow) {
                    contained = false;
                    break;
                }

                nx += 1;
            }
            let mut nx = x - 1;
            loop {
                let npos = (nx, y);
                if self.clays.contains(&npos) {
                    break;
                }
                layer.push(npos);

                let nbelow = (nx, y + 1);
                self.descend(nbelow);
                if !self.clays.contains(&nbelow) && !self.contained.contains(&nbelow) {
                    contained = false;
                    break;
                }
                nx -= 1;
            }

            if contained {
                self.contained.extend(layer);
            } else {
                self.flowing.extend(layer);
            }
        } else {
            self.flowing.insert(pos);
        }
    }
}

impl Solution for Day17 {
    fn part1(&mut self, input: &mut Read) -> String {
        self.read_input(input);

        self.descend((500, 0));

        let result = self.contained.iter()
            .chain(self.flowing.iter())
            .filter(|&&(_, y)| y >= self.ymin && y <= self.ymax).count();
        format!("{}", result)
    }

    fn part2(&mut self, input: &mut Read) -> String {
        self.read_input(input);

        self.descend((500, 0));

        let result = self.contained.iter()
            .filter(|&&(_, y)| y >= self.ymin && y <= self.ymax).count();
        format!("{}", result)
    }
}

#[cfg(test)]
mod tests {
    use day17::Day17;
    use common::Solution;

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
