use anyhow::Result;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;

use crate::common::minmax;
use crate::common::parse_input;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
struct Assignment(u32, u32);

impl Assignment {
    fn one_contains(self, other: Self) -> bool {
        let (first, second) = minmax(self, other);

        if second.0 == first.0 {
            first.1 <= second.1
        } else {
            second.0 <= first.1 && second.1 <= first.1
        }
    }

    fn one_overlaps(self, other: Self) -> bool {
        let (first, second) = minmax(self, other);

        if second.0 == first.0 {
            first.1 <= second.1
        } else {
            second.0 <= first.1
        }
    }
}

fn parse_assignments(input: &[u8]) -> IResult<&[u8], Vec<(Assignment, Assignment)>> {
    use nom::character::complete::u32;

    fn parse_single(input: &[u8]) -> IResult<&[u8], Assignment> {
        map(separated_pair(u32, tag("-"), u32), |(start, end)| {
            Assignment(start, end)
        })(input)
    }

    let parse_line = separated_pair(parse_single, tag(","), parse_single);

    many0(terminated(parse_line, newline))(input)
}

pub fn part1(input: &[u8]) -> Result<String> {
    let assigments = parse_input(input, parse_assignments)?;

    let overlapping = assigments
        .into_iter()
        .filter(|&(a, b)| a.one_contains(b))
        .count();

    Ok(overlapping.to_string())
}

pub fn part2(input: &[u8]) -> Result<String> {
    let assigments = parse_input(input, parse_assignments)?;

    let overlapping = assigments
        .into_iter()
        .filter(|&(a, b)| a.one_overlaps(b))
        .count();

    Ok(overlapping.to_string())
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
