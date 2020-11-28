use std::fs::File;
use std::io::Read;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::time::Instant;

use clap::Clap;

use aoc_2020::get_implementation;

/// Advent of Code 2020 runner
#[derive(Clap)]
struct Opts {
    /// Which day to run
    day: NonZeroUsize,

    /// Print time taken
    #[clap(short, long)]
    time: bool,

    /// Run part 2 instead of part 1
    #[clap(short = '2', long)]
    part2: bool,

    /// Read input from the given file instead of stdin
    #[clap(short, long)]
    input: Option<PathBuf>,
}

fn main() {
    let opts: Opts = Opts::parse();

    let mut implementation = get_implementation(opts.day.get());
    let mut input: Box<dyn Read> = if let Some(input) = opts.input {
        Box::new(File::open(&input).expect("Failed to open input"))
    } else {
        Box::new(std::io::stdin())
    };

    let begin = Instant::now();
    let result = if opts.part2 {
        implementation.part2(&mut input)
    } else {
        implementation.part1(&mut input)
    };

    if opts.time {
        eprintln!("Execution time: {:?}", Instant::now().duration_since(begin));
    }

    println!("{}", result);
}
