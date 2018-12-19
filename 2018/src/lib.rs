extern crate chrono;
#[macro_use]
extern crate itertools;
extern crate regex;

pub mod common;
pub mod cpu;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub fn get_impl(day: u32) -> Box<common::Solution> {
    match day {
        1 => Box::new(day01::Day01::new()),
        2 => Box::new(day02::Day02::new()),
        3 => Box::new(day03::Day03::new()),
        4 => Box::new(day04::Day04::new()),
        5 => Box::new(day05::Day05::new()),
        6 => Box::new(day06::Day06::new()),
        7 => Box::new(day07::Day07::new()),
        8 => Box::new(day08::Day08::new()),
        9 => Box::new(day09::Day09::new()),
        10 => Box::new(day10::Day10::new()),
        11 => Box::new(day11::Day11::new()),
        12 => Box::new(day12::Day12::new()),
        13 => Box::new(day13::Day13::new()),
        14 => Box::new(day14::Day14::new()),
        15 => Box::new(day15::Day15::new()),
        16 => Box::new(day16::Day16::new()),
        17 => Box::new(day17::Day17::new()),
        18 => Box::new(day18::Day18::new()),
        19 => Box::new(day19::Day19::new()),
        20 => Box::new(day20::Day20::new()),
        21 => Box::new(day21::Day21::new()),
        22 => Box::new(day22::Day22::new()),
        23 => Box::new(day23::Day23::new()),
        24 => Box::new(day24::Day24::new()),
        25 => Box::new(day25::Day25::new()),
        val => panic!("Unimplemented day {}", val),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_impl() {
        // Verify that we can load all days
        let last_implemented = 25;
        for d in 1..=last_implemented {
            get_impl(d);
        }
    }
}
