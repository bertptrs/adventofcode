extern crate clap;
use clap::{Arg, App};
use std::fs;
use std::io;

pub mod common;
pub mod day1;

fn get_impl(day: i32) -> Box<common::Solution> {
    match day {
        1 => { Box::new(day1::Day1::new()) }
        _ => {
            panic!("Unimplemented day {}", day)
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

    let day: i32 = (&matches.value_of("day").unwrap()).parse()
        .expect("Invalid int");
    let mut implementation = get_impl(day);
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
            get_impl(d);
        }
    }
}
