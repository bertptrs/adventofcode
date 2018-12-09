extern crate chrono;
#[macro_use] extern crate intrusive_collections;
#[macro_use] extern crate itertools;
extern crate regex;

pub mod common;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;

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
        val => panic!("Unimplemented day {}", val),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_impl() {
        // Verify that we can load all days
        let last_implemented = 8;
        for d in 1..=last_implemented {
            get_impl(d);
        }
    }
}
