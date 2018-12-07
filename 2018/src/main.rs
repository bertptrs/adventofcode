extern crate chrono;
extern crate clap;
#[macro_use] extern crate itertools;
extern crate regex;

use std::fs;
use std::io;
use std::time::Instant;

use clap::{App, Arg};

pub mod common;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

fn get_impl(day: &str) -> Box<common::Solution> {
    match day.parse() {
        Ok(1) => Box::new(day01::Day01::new()),
        Ok(2) => Box::new(day02::Day02::new()),
        Ok(3) => Box::new(day03::Day03::new()),
        Ok(4) => Box::new(day04::Day04::new()),
        Ok(5) => Box::new(day05::Day05::new()),
        Ok(6) => Box::new(day06::Day06::new()),
        Ok(7) => Box::new(day07::Day07::new()),
        Ok(val) => panic!("Unimplemented day {}", val),
        _ => panic!("Invalid number"),
    }
}

fn main() {
    let matches = App::new("Advent of Code")
        .version("2018")
        .author("Bert Peters <bert@bertptrs.nl>")
        .arg(Arg::with_name("day")
             .value_name("DAY")
             .help("Number of the day to execute")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("part2")
             .short("2")
             .help("Whether to run part 2")
             .long("part2"))
        .arg(Arg::with_name("input")
             .short("i")
             .long("input")
             .help("Optional input file, stdin otherwise")
             .takes_value(true))
        .arg(Arg::with_name("time")
             .short("t")
             .long("time")
             .help("Print the time for the result"))
        .get_matches();

    let mut implementation = get_impl(matches.value_of("day").unwrap());
    let mut data: Box<io::Read> = match matches.value_of("input") {
        Some(filename) => { Box::new(fs::File::open(filename).unwrap()) }
        None => { Box::new(io::stdin()) }
    };

    let begin = Instant::now();
    let result = if matches.is_present("part2") {
        implementation.part2(&mut data)
    } else {
        implementation.part1(&mut data)
    };
    if matches.is_present("time") {
        eprintln!("Duration: {:?}", Instant::now().duration_since(begin));
    }
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_impl() {
        // Verify that we can load all days
        let last_implemented = 6;
        for d in 1..=last_implemented {
            get_impl(&format!("{}", d));
        }
    }
}
