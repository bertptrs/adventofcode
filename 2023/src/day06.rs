use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::character::complete::space1;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::IResult;

use crate::common::parse_input;

fn parse_race(i: &[u8]) -> IResult<&[u8], (Vec<u64>, Vec<u64>)> {
    use nom::character::complete::u64;

    pair(
        delimited(tag("Time:"), many1(preceded(space1, u64)), newline),
        delimited(tag("Distance:"), many1(preceded(space1, u64)), newline),
    )(i)
}

fn ways(time: u64, distance: u64) -> u64 {
    let make_it = (1..=time / 2)
        .filter(|&v| v * (time - v) > distance)
        .count() as u64;

    if time % 2 == 0 {
        2 * make_it - 1
    } else {
        2 * make_it
    }
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let (time, distance) = parse_input(input, parse_race)?;

    let total: u64 = time
        .iter()
        .zip(&distance)
        .map(|(&time, &distance)| ways(time, distance))
        .product();

    Ok(total.to_string())
}

pub fn part2(_input: &[u8]) -> anyhow::Result<String> {
    anyhow::bail!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/06.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "288");
    }
}
