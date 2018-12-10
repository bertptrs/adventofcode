use std::i32;
use std::collections::HashSet;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use itertools::Itertools;
use itertools::MinMaxResult;
use regex::Regex;

use common::Solution;

#[derive(Default)]
pub struct Day10 {
    points: Vec<(i32, i32)>,
    speeds: Vec<(i32, i32)>,
}

impl Day10 {
    pub fn new() -> Self {
        Default::default()
    }

    fn read_inputs(&mut self, input: &mut Read) {
        let reader = BufReader::new(input);
        let matcher = Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>").unwrap();

        for line in reader.lines() {
            let line = line.unwrap();
            let groups = matcher.captures(&line).unwrap();

            self.points.push((groups[1].parse().unwrap(), groups[2].parse().unwrap()));
            self.speeds.push((groups[3].parse().unwrap(), groups[4].parse().unwrap()));
        }
    }

    fn print_state(&self) -> String {
        let points: HashSet<_> = self.points.iter().collect();
        let (xmin, xmax) = match self.points.iter().map(|&(x, _)| x).minmax() {
            MinMaxResult::MinMax(x, y) => (x, y),
            _ => unreachable!(),
        };
        let (ymin, ymax) = match self.points.iter().map(|&(_, y)| y).minmax() {
            MinMaxResult::MinMax(x, y) => (x, y),
            _ => unreachable!(),
        };

        let mut buffer = String::with_capacity(((xmax + 1 - xmin) * (ymax + 1 - ymin)) as usize);

        for y in ymin..=ymax {
            for x in xmin..=xmax {
                let c = if points.contains(&(x, y)) { '#' } else { '.' };
                buffer.push(c);
            }

            buffer.push('\n');
        }

        buffer
    }

    fn run(&mut self, time_step: i32) {
        for ((x, y), (dx, dy)) in self.points.iter_mut().zip(self.speeds.iter()) {
            *x += dx * time_step;
            *y += dy * time_step;
        }
    }

    fn height(&self) -> i32 {
        match self.points.iter().map(|&(_, y)| y).minmax() {
            MinMaxResult::MinMax(x, y) => { y - x + 1 }
            _ => panic!("Input does not make sense."),
        }
    }
}

impl Solution for Day10 {
    fn part1(&mut self, input: &mut Read) -> String {
        self.read_inputs(input);
        let mut prev = i32::MAX;
        let mut height = self.height();
        while height < prev {
            prev = height;
            self.run(1);
            height = self.height();
        }
        self.run(-1);
        self.print_state()
    }

    fn part2(&mut self, input: &mut Read) -> String {
        self.read_inputs(input);
        let mut prev = i32::MAX;
        let mut height = self.height();
        let mut steps = -1;
        while height < prev {
            steps += 1;
            prev = height;
            self.run(1);
            height = self.height();
        }
        format!("{}", steps)
    }
}

#[cfg(test)]
mod tests {
    use common::Solution;
    use day10::Day10;

    const SAMPLE_INPUT: &[u8] = include_bytes!("samples/10.txt");

    #[test]
    fn sample_part1() {
        let mut instance = Day10::new();
        let result = instance.part1(&mut SAMPLE_INPUT);
        let correct = include_str!("samples/10.out.txt");
        assert_eq!(correct, &result);
    }

    #[test]
    fn sample_part2() {
        let mut instance = Day10::new();
        assert_eq!("3", instance.part2(&mut SAMPLE_INPUT));
    }
}
