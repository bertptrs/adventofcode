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
    let a = -1.0;
    let b = time as f64;
    let c = -(distance as f64);
    let d = b * b - 4.0 * a * c;
    if d < 0.0 {
        0
    } else {
        // Note: can leave out quite a bit of the quadratic formula because things cancel out nicely
        let solution = ((b - d.sqrt()) / 2.0 + 1.0) as u64;
        let half = time / 2;
        let make_it = half - solution + 1;

        if time % 2 == 0 {
            2 * make_it - 1
        } else {
            2 * make_it
        }
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
    fn individual_samples() {
        assert_eq!(ways(7, 9), 4);
        assert_eq!(ways(15, 40), 8);
        assert_eq!(ways(30, 200), 9);
    }

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "288");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "71503");
    }
}
