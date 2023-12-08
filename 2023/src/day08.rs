use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::bytes::complete::take_until;
use nom::combinator::map;
use nom::multi::fold_many1;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

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

pub fn part2(_input: &[u8]) -> anyhow::Result<String> {
    anyhow::bail!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/08.1.txt");
    const SAMPLE2: &[u8] = include_bytes!("samples/08.2.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("2", part1(SAMPLE).unwrap());
        assert_eq!("6", part1(SAMPLE2).unwrap());
    }
}
