use std::ops::Add;

use anyhow::Result;
use nom::character::complete::newline;
use nom::combinator::opt;
use nom::multi::fold_many1;
use nom::multi::separated_list0;
use nom::sequence::terminated;
use nom::IResult;

use crate::common::parse_input;

fn parse_elf(input: &[u8]) -> IResult<&[u8], i32> {
    fold_many1(
        terminated(nom::character::complete::i32, newline),
        || 0,
        Add::add,
    )(input)
}

fn parse_max(input: &[u8]) -> IResult<&[u8], i32> {
    fold_many1(terminated(parse_elf, opt(newline)), || 0, Ord::max)(input)
}

pub fn part1(input: &[u8]) -> Result<String> {
    let result = parse_input(input, parse_max)?.to_string();
    Ok(result)
}

fn parse_elf_list(input: &[u8]) -> IResult<&[u8], Vec<i32>> {
    separated_list0(newline, parse_elf)(input)
}

pub fn part2(input: &[u8]) -> Result<String> {
    let mut elves = parse_input(input, parse_elf_list)?;

    let (first, third, _) = elves.select_nth_unstable_by(2, |a, b| Ord::cmp(b, a));

    let result = first[1] + first[0] + *third;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/01.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "24000");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "45000");
    }
}
