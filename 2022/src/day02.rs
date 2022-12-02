use std::ops::Add;

use anyhow::Result;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;

use crate::common::parse_input;
use crate::common::reduce_many1;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    /// Score we get by playing this move
    fn score(self) -> u32 {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }

    /// Score we get from the result from playing given other
    fn score_against(self, other: Self) -> u32 {
        match (self, other) {
            (a, b) if a == b => 3,
            (Rps::Rock, Rps::Paper) | (Rps::Paper, Rps::Scissors) | (Rps::Scissors, Rps::Rock) => 0,
            _ => 6,
        }
    }

    /// Score if the result is according to the instruction
    fn score_result(self) -> u32 {
        match self {
            Rps::Rock => 0,     // Rock is lose
            Rps::Paper => 3,    // Paper is draw
            Rps::Scissors => 6, // Scissors is win
        }
    }

    /// Move we need to achieve the result indicated by self
    fn needed(self, other: Self) -> Self {
        match (self, other) {
            (Rps::Paper, other) => other,
            (Rps::Rock, Rps::Rock) => Rps::Scissors,
            (Rps::Rock, Rps::Paper) => Rps::Rock,
            (Rps::Rock, Rps::Scissors) => Rps::Paper,
            (Rps::Scissors, Rps::Rock) => Rps::Paper,
            (Rps::Scissors, Rps::Paper) => Rps::Scissors,
            (Rps::Scissors, Rps::Scissors) => Rps::Rock,
        }
    }
}

impl TryFrom<u8> for Rps {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'A' | b'X' => Ok(Rps::Rock),
            b'B' | b'Y' => Ok(Rps::Paper),
            b'C' | b'Z' => Ok(Rps::Scissors),
            _ => Err(anyhow::anyhow!("Invalid RPS: {value}")),
        }
    }
}

fn parse_line(input: &[u8]) -> IResult<&[u8], (Rps, Rps)> {
    fn parse_rps(input: &[u8]) -> IResult<&[u8], Rps> {
        // Note: alpha1 also sort of works but is significantly slower
        map_res(nom::bytes::complete::take(1usize), |v: &[u8]| {
            Rps::try_from(v[0])
        })(input)
    }

    terminated(
        separated_pair(parse_rps, nom::character::complete::char(' '), parse_rps),
        newline,
    )(input)
}

pub fn part1(input: &[u8]) -> Result<String> {
    parse_input(
        input,
        reduce_many1(
            map(parse_line, |(them, us)| us.score() + us.score_against(them)),
            Add::add,
        ),
    )
    .map(|sum| sum.to_string())
}

pub fn part2(input: &[u8]) -> Result<String> {
    parse_input(
        input,
        reduce_many1(
            map(parse_line, |(them, us)| {
                us.score_result() + us.needed(them).score()
            }),
            Add::add,
        ),
    )
    .map(|sum| sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/02.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "15")
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "12")
    }
}
