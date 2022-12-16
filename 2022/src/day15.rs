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
    range: i32,
}

impl Beacon {
    pub fn can_contain_unseen(&self, lower: Vec2, upper: Vec2) -> bool {
        let corners = [
            lower,
            upper,
            Vec2([lower[0], upper[1]]),
            Vec2([upper[0], lower[1]]),
        ];

        corners
            .into_iter()
            .any(|c| (c - self.pos).l1() > self.range)
    }
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
        |(pos, closest)| Beacon {
            pos,
            closest,
            range: (pos - closest).l1(),
        },
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
        let horizontal_distance = beacon.range - (beacon.pos[1] - Y).abs();

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

fn part2_generic<const MAX: i32>(input: &[u8]) -> Result<String> {
    let beacons = parse_input(input, parse_beacons)?;

    fn find_unseen<const MAX: i32>(beacons: &[Beacon]) -> Result<Vec2> {
        let mut todo = vec![(Vec2([0, 0]), Vec2([MAX, MAX]))];

        while let Some((lower, upper)) = todo.pop() {
            if lower == upper {
                if beacons
                    .iter()
                    .all(|beacon| (beacon.pos - lower).l1() > beacon.range)
                {
                    return Ok(lower);
                }
            } else {
                let mid = (lower + upper) / 2;

                let quads = [
                    (lower, mid),
                    (Vec2([lower[0], mid[1] + 1]), Vec2([mid[0], upper[1]])),
                    (Vec2([mid[0] + 1, lower[1]]), Vec2([upper[0], mid[1]])),
                    (Vec2([mid[0] + 1, mid[1] + 1]), upper),
                ];

                for (lower_new, upper_new) in quads {
                    if lower_new[0] > upper_new[0] || lower_new[1] > upper_new[1] {
                        // Empty quad in quad tree, skip
                    } else if beacons
                        .iter()
                        .all(|beacon| beacon.can_contain_unseen(lower_new, upper_new))
                    {
                        todo.push((lower_new, upper_new));
                    }
                }
            }
        }

        anyhow::bail!("Did not find position")
    }

    let Vec2([x, y]) = find_unseen::<MAX>(&beacons)?;

    Ok((i64::from(x) * 4000000 + i64::from(y)).to_string())
}

pub fn part2(input: &[u8]) -> Result<String> {
    part2_generic::<4000000>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/15.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1_generic::<10>(SAMPLE).unwrap(), "26");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2_generic::<20>(SAMPLE).unwrap(), "56000011");
    }
}
