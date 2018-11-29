extern crate clap;
extern crate regex;
extern crate itertools;
extern crate permutohedron;
use clap::{Arg, App};
use std::fs;
use std::io;

pub mod common;
pub mod day1;
pub mod day12;
pub mod day15;
pub mod day16;
pub mod day23;
pub mod day24;
pub mod day25;

fn get_impl(day: &str) -> Box<common::Solution> {
    match day.parse() {
        Ok(1) => { Box::new(day1::Day1::new()) }
        Ok(12) => { Box::new(day12::Day12::new()) }
        Ok(15) => { Box::new(day15::Day15::new()) }
        Ok(16) => { Box::new(day16::Day16::new()) }
        Ok(23) => { Box::new(day23::Day23::new()) }
        Ok(24) => { Box::new(day24::Day24::new()) }
        Ok(25) => { Box::new(day25::Day25::new()) }
        Ok(val) => {
            panic!("Unimplemented day {}", val)
        },
        Err(_) => {
            panic!("Invalid day");
        }
    }
}

fn main() {
    let matches = App::new("Advent of Code")
        .version("2016")
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
        .get_matches();

    let mut implementation = get_impl(&matches.value_of("day").unwrap());
    let mut data: Box<io::Read> = match matches.value_of("input") {
        Some(filename) => { Box::new(fs::File::open(filename).unwrap()) }
        None => { Box::new(io::stdin()) }
    };

    if matches.is_present("part2") {
        println!("{}", implementation.part2(&mut data));
    } else {
        println!("{}", implementation.part1(&mut data));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_impl() {
        // Verify that we can load all days
        let last_implemented = 1;
        for d in 1..(last_implemented + 1) {
            get_impl(&format!("{}", d));
        }
    }
}
