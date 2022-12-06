use std::ops::RangeInclusive;

use anyhow::Result;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;

use crate::common::parse_input;

type Assignment = RangeInclusive<u32>;

fn parse_assignments(
    input: &[u8],
) -> IResult<&[u8], Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    use nom::character::complete::u32;

    fn parse_single(input: &[u8]) -> IResult<&[u8], Assignment> {
        map(separated_pair(u32, tag("-"), u32), |(start, end)| {
            start..=end
        })(input)
    }

    let parse_line = separated_pair(parse_single, tag(","), parse_single);

    many0(terminated(parse_line, newline))(input)
}

fn is_contained(a: &Assignment, b: &Assignment) -> bool {
    if a.size_hint().0 > b.size_hint().0 {
        a.contains(b.start()) && a.contains(b.end())
    } else {
        b.contains(a.start()) && b.contains(a.end())
    }
}

fn is_overlapping(a: &Assignment, b: &Assignment) -> bool {
    b.end() >= a.start() && b.start() <= a.end() || a.end() >= b.start() && a.start() <= b.end()
}

fn parts_common(input: &[u8], filter: impl Fn(&Assignment, &Assignment) -> bool) -> Result<String> {
    let assigments = parse_input(input, parse_assignments)?;

    let overlapping = assigments.into_iter().filter(|(a, b)| filter(a, b)).count();

    Ok(overlapping.to_string())
}

pub fn part1(input: &[u8]) -> Result<String> {
    parts_common(input, is_contained)
}

pub fn part2(input: &[u8]) -> Result<String> {
    parts_common(input, is_overlapping)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/04.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "2")
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "4")
    }
}
