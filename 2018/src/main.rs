extern crate chrono;
#[macro_use] extern crate clap;
extern crate aoc_2018;

use std::fs;
use std::io;
use std::time::Instant;

use aoc_2018::*;

use clap::Arg;

fn get_impl(day: u32) -> Box<common::Solution> {
    match day {
        1 => Box::new(day01::Day01::new()),
        2 => Box::new(day02::Day02::new()),
        3 => Box::new(day03::Day03::new()),
        4 => Box::new(day04::Day04::new()),
        5 => Box::new(day05::Day05::new()),
        6 => Box::new(day06::Day06::new()),
        7 => Box::new(day07::Day07::new()),
        8 => Box::new(day08::Day08::new()),
        val => panic!("Unimplemented day {}", val),
    }
}

fn main() {
    let matches = app_from_crate!()
        .arg(Arg::with_name("day")
            .value_name("DAY")
            .help("Number of the day to execute")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("part2")
            .short("2")
            .help("Run part 2 instead of part 1")
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

    let mut implementation = get_impl(value_t_or_exit!(matches, "day", u32));
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
        let last_implemented = 8;
        for d in 1..=last_implemented {
            get_impl(d);
        }
    }
}
