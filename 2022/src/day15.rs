use std::cmp::Reverse;
use std::collections::BinaryHeap;

use ahash::AHashSet;
use anyhow::Result;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;

use crate::common::parse_input;
use crate::common::Vec2;

struct Beacon {
    pos: Vec2,
    closest: Vec2,
}

fn parse_pos(input: &[u8]) -> IResult<&[u8], Vec2> {
    use nom::character::complete::i32;
    map(
        pair(preceded(tag("x="), i32), preceded(tag(", y="), i32)),
        |(x, y)| Vec2([x, y]),
    )(input)
}

fn parse_beacons(input: &[u8]) -> IResult<&[u8], Vec<Beacon>> {
    let parse_beacon = map(
        pair(
            preceded(tag("Sensor at "), parse_pos),
            preceded(tag(": closest beacon is at "), parse_pos),
        ),
        |(pos, closest)| Beacon { pos, closest },
    );

    many0(terminated(parse_beacon, newline))(input)
}

fn part1_generic<const Y: i32>(input: &[u8]) -> Result<String> {
    let beacons = parse_input(input, parse_beacons)?;

    let mut not_blocking_yet = BinaryHeap::new();
    let mut blocking = BinaryHeap::new();

    let mut total = 0;

    let mut on_line = AHashSet::new();

    for beacon in beacons {
        let distance = (beacon.closest - beacon.pos).l1();
        let horizontal_distance = distance - (beacon.pos[1] - Y).abs();

        if horizontal_distance >= 0 {
            not_blocking_yet.push(Reverse((
                beacon.pos[0] - horizontal_distance,
                beacon.pos[0] + horizontal_distance + 1,
            )))
        }

        // Beacons can be beacons, so we should uncount them
        if beacon.closest[1] == Y {
            on_line.insert(beacon.closest);
        }
    }

    while let Some(Reverse((block_from, mut block_until))) = not_blocking_yet.pop() {
        blocking.push(Reverse(block_until));

        while let Some(Reverse(block_end)) = blocking.pop() {
            block_until = block_until.max(block_end);

            while matches!(not_blocking_yet.peek(), Some(Reverse((block_from, _))) if block_from < &block_end)
            {
                let Reverse((_, additional_block_until)) = not_blocking_yet.pop().unwrap();
                blocking.push(Reverse(additional_block_until));
            }
        }

        total += block_until - block_from;
    }

    total -= on_line.len() as i32;

    Ok(total.to_string())
}

pub fn part1(input: &[u8]) -> Result<String> {
    part1_generic::<2000000>(input)
}

pub fn part2(_input: &[u8]) -> Result<String> {
    anyhow::bail!("not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/15.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1_generic::<10>(SAMPLE).unwrap(), "26");
    }
}
