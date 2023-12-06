use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::newline;
use nom::character::complete::space1;
use nom::multi::fold_many1;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::IResult;

use crate::common::parse_input;

fn parse_race(i: &[u8]) -> IResult<&[u8], (Vec<u64>, Vec<u64>)> {
    let line = |header| {
        delimited(
            tag(header),
            many1(preceded(space1, nom::character::complete::u64)),
            newline,
        )
    };

    pair(line("Time:"), line("Distance:"))(i)
}

fn parse_long_race(i: &[u8]) -> IResult<&[u8], (u64, u64)> {
    let line = |header| {
        delimited(
            tag(header),
            fold_many1(
                preceded(space1, digit1),
                || 0,
                |mut cur, sequence| {
                    for &c in sequence {
                        cur *= 10;
                        cur += u64::from(c - b'0');
                    }

                    cur
                },
            ),
            newline,
        )
    };

    pair(line("Time:"), line("Distance:"))(i)
}

fn ways(time: u64, distance: u64) -> u64 {
    let half = time / 2;
    let mut min = 1;
    let mut max = half;

    while min < max {
        let mid = min + (max - min) / 2;

        if mid * (time - mid) < distance {
            min = mid + 1;
        } else {
            max = mid;
        }
    }

    let make_it = half - min + 1;

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

pub fn part2(input: &[u8]) -> anyhow::Result<String> {
    let (time, distance) = parse_input(input, parse_long_race)?;

    Ok(ways(time, distance).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/06.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "288");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "71503");
    }
}
