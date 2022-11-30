use std::fs::File;
use std::io::Read;
use std::num::NonZeroU8;
use std::path::PathBuf;
use std::time::Instant;

use anyhow::Result;
use clap::Parser;

use aoc_2022::get_implementation;

/// Advent of Code 2022 runner
#[derive(Parser)]
struct Opts {
    /// Which day to run
    day: NonZeroU8,

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

impl Opts {
    fn input_data(&self) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();

        if let Some(input) = &self.input {
            File::open(input)?.read_to_end(&mut buffer)?;
        } else {
            std::io::stdin().read_to_end(&mut buffer)?;
        }

        Ok(buffer)
    }
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let input = opts.input_data()?;

    let implementation = get_implementation(opts.day.get(), opts.part2)?;

    let begin = Instant::now();
    let result = implementation(&input)?;

    if opts.time {
        eprintln!("Execution time: {:?}", Instant::now().duration_since(begin));
    }

    println!("{}", result);
    Ok(())
}
