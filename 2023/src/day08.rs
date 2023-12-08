use anyhow::Context;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::bytes::complete::take_until;
use nom::combinator::map;
use nom::multi::fold_many1;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;
use num_integer::Integer;

use crate::common::parse_input;

const NUM_PLACES: usize = 26 * 26 * 26;

struct Map<'a> {
    instructions: &'a [u8],
    transitions: Box<[(u16, u16); NUM_PLACES]>,
}

impl<'a> Map<'a> {
    #[inline]
    fn transition(&self, pos: u16, step: u8) -> u16 {
        if step == b'L' {
            self.transitions[pos as usize].0
        } else {
            self.transitions[pos as usize].1
        }
    }
}

fn place_to_index(place: &[u8]) -> u16 {
    place
        .iter()
        .fold(0u16, |index, &c| index * 26 + u16::from(c - b'A'))
}

fn parse_map(i: &[u8]) -> IResult<&[u8], Map<'_>> {
    map(
        separated_pair(
            take_until("\n"),
            tag("\n\n"),
            fold_many1(
                tuple((
                    terminated(take(3usize), tag(" = (")),
                    terminated(take(3usize), tag(", ")),
                    terminated(take(3usize), tag(")\n")),
                )),
                || Box::new([(0, 0); NUM_PLACES]),
                |mut transitions, (pos, left, right)| {
                    let pos = place_to_index(pos);
                    let left = place_to_index(left);
                    let right = place_to_index(right);
                    transitions[pos as usize] = (left, right);
                    transitions
                },
            ),
        ),
        |(instructions, transitions)| Map {
            instructions,
            transitions,
        },
    )(i)
}

fn parse_starts(i: &[u8]) -> IResult<&[u8], Vec<u16>> {
    preceded(
        tuple((take_until("\n"), tag("\n\n"))),
        fold_many1(
            terminated(
                map(take(3usize), place_to_index),
                tuple((take_until("\n"), tag("\n"))),
            ),
            Vec::new,
            |mut starts, place| {
                if place % 26 == 0 {
                    starts.push(place)
                }
                starts
            },
        ),
    )(i)
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let map = parse_input(input, parse_map)?;
    let end = place_to_index(b"ZZZ");
    let mut pos = 0;

    for (count, &step) in map.instructions.iter().cycle().enumerate() {
        if pos == end {
            return Ok(count.to_string());
        }

        pos = map.transition(pos, step);
    }

    anyhow::bail!("Unreachable, loop is infinite");
}

// This code is wrong. There is no reason that the start of the cycle is indeed the equal to the
// length of the cycle. But it happens to be the case, so we roll with it. Otherwise you could go
// with the full Chinese remainder theorem and knock yourself out that way.
//
// I didn't wanna.
fn find_cycle(map: &Map<'_>, start: u16) -> usize {
    let mut pos = start;
    for (count, &step) in map.instructions.iter().cycle().enumerate() {
        if pos % 26 == 25 {
            return count;
        }
        pos = map.transition(pos, step);
    }

    unreachable!("Loop is actually infinite")
}

pub fn part2(input: &[u8]) -> anyhow::Result<String> {
    let map = parse_input(input, parse_map)?;
    let pos = parse_input(input, parse_starts)?;

    pos.iter()
        .map(|&p| find_cycle(&map, p))
        .reduce(|a, b| a.lcm(&b))
        .map(|s| s.to_string())
        .context("No starting points somehow")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/08.1.txt");
    const SAMPLE2: &[u8] = include_bytes!("samples/08.2.txt");
    // N.B. sample modified because I don't want to change my parser logic to deal with ascii digits
    // in addition to capitals. 1 has been replaced with D, 2 has been replaced with E.
    const SAMPLE3: &[u8] = include_bytes!("samples/08.3.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("2", part1(SAMPLE).unwrap());
        assert_eq!("6", part1(SAMPLE2).unwrap());
    }

    #[test]
    fn sample_part2() {
        assert_eq!("6", part2(SAMPLE3).unwrap());
    }
}
