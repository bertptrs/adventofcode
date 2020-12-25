use std::io::Read;

mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub trait Solution {
    fn part1(&mut self, input: &mut dyn Read) -> String;

    fn part2(&mut self, _input: &mut dyn Read) -> String {
        unimplemented!("Still working on part 1");
    }
}

pub fn get_implementation(day: usize) -> Box<dyn Solution> {
    match day {
        1 => Box::new(day01::Day01::default()),
        2 => Box::new(day02::Day02::default()),
        3 => Box::new(day03::Day03::default()),
        4 => Box::new(day04::Day04::default()),
        5 => Box::new(day05::Day05::default()),
        6 => Box::new(day06::Day06::default()),
        7 => Box::new(day07::Day07::default()),
        8 => Box::new(day08::Day08::default()),
        9 => Box::new(day09::Day09::default()),
        10 => Box::new(day10::Day10::default()),
        11 => Box::new(day11::Day11::default()),
        12 => Box::new(day12::Day12::default()),
        13 => Box::new(day13::Day13::default()),
        14 => Box::new(day14::Day14::default()),
        15 => Box::new(day15::Day15::default()),
        16 => Box::new(day16::Day16::default()),
        17 => Box::new(day17::Day17::default()),
        18 => Box::new(day18::Day18::default()),
        19 => Box::new(day19::Day19::default()),
        20 => Box::new(day20::Day20::default()),
        21 => Box::new(day21::Day21::default()),
        22 => Box::new(day22::Day22::default()),
        23 => Box::new(day23::Day23::default()),
        24 => Box::new(day24::Day24::default()),
        25 => Box::new(day25::Day25::default()),
        _ => panic!("Unsupported day {}", day),
    }
}

#[cfg(test)]
fn test_implementation(mut day: impl Solution, part: u8, mut input: &[u8], answer: impl ToString) {
    let result = match part {
        1 => day.part1(&mut input),
        2 => day.part2(&mut input),
        _ => panic!("Invalid part: {}", part),
    };

    assert_eq!(answer.to_string(), result);
}
