extern crate aoc_2018;
extern crate chrono;
#[macro_use]
extern crate clap;

use std::fs;
use std::io;
use std::time::Instant;

use clap::Arg;

use aoc_2018::get_impl;

fn main() {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("day")
                .value_name("DAY")
                .help("Number of the day to execute")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("part2")
                .short("2")
                .help("Run part 2 instead of part 1")
                .long("part2"),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .help("Optional input file, stdin otherwise")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("time")
                .short("t")
                .long("time")
                .help("Print the time for the result"),
        )
        .get_matches();

    let mut implementation = get_impl(value_t_or_exit!(matches, "day", u32));
    let mut data: Box<io::Read> = match matches.value_of("input") {
        Some(filename) => Box::new(fs::File::open(filename).unwrap()),
        None => Box::new(io::stdin()),
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
