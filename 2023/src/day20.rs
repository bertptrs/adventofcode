use std::collections::HashMap;
use std::collections::VecDeque;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::bytes::complete::take_while1;
use nom::combinator::map;
use nom::combinator::value;
use nom::multi::fold_many1;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::IResult;

use crate::common::parse_input;

#[derive(Clone)]
enum Node {
    FlipFlop(bool),
    Conjunction(Vec<u32>),
    Broadcaster,
}

#[derive(Clone)]
struct Entry {
    node: Node,
    dest: Vec<u32>,
}

// Wouldn't ya know it, big boy base 26 at it again
fn convert_name(name: &[u8]) -> u32 {
    name.iter()
        // The word "broadcaster" overflows but nothing else is nearly as long so it's fine to cut
        // it short.
        .take(5)
        .fold(0, |cur, &c| cur * 26 + u32::from(c - b'a'))
}

fn parse_cables(i: &[u8]) -> IResult<&[u8], HashMap<u32, Entry>> {
    fn parse_cable(i: &[u8]) -> IResult<&[u8], (u32, Entry)> {
        let (i, node) = alt((
            value(Node::FlipFlop(false), tag("%")),
            value(Node::Conjunction(Vec::new()), tag("&")),
            value(Node::Broadcaster, tag("broadcaster")),
        ))(i)?;

        let (i, id) = if let Node::Broadcaster = node {
            (i, convert_name(b"broadcaster"))
        } else {
            map(take_until(" "), convert_name)(i)?
        };

        let (i, dest) = delimited(
            tag(" -> "),
            separated_list1(
                tag(", "),
                map(take_while1(|c: u8| c.is_ascii_alphabetic()), convert_name),
            ),
            tag("\n"),
        )(i)?;

        Ok((i, (id, Entry { node, dest })))
    }

    let (i, mut cables) = fold_many1(parse_cable, HashMap::new, |mut map, (id, entry)| {
        map.insert(id, entry);
        map
    })(i)?;

    let cable_ids: Vec<_> = cables.keys().copied().collect();
    let mut buffer = Vec::new();

    for cable_id in &cable_ids {
        buffer.extend_from_slice(&cables[cable_id].dest);

        for dest_id in &buffer {
            let Some(cable) = cables.get_mut(dest_id) else {
                continue;
            };

            if let Node::Conjunction(sources) = &mut cable.node {
                sources.push(*cable_id);
            }
        }
        buffer.clear();
    }

    Ok((i, cables))
}

// 604293120 too low
pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let mut cables = parse_input(input, parse_cables)?;
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    let mut todo = VecDeque::new();
    let mut last_pulse: HashMap<u32, bool> = HashMap::new();

    for _ in 0..1000 {
        low_pulses += 1;
        todo.push_back((false, convert_name(b"broadcaster")));

        while let Some((pulse, pos)) = todo.pop_front() {
            let Some(cable) = cables.get_mut(&pos) else {
                // Sometimes cables aren't real, and that's okay
                continue;
            };

            let next_pulse = match &mut cable.node {
                Node::FlipFlop(state) => {
                    if pulse {
                        // Ignore, nothing to be done since it's a high pulse
                        continue;
                    } else {
                        *state = !*state;
                        *state
                    }
                }
                Node::Conjunction(inwards) => {
                    // Need to deal with the check outside the match otherwise lifetime issues :(
                    !inwards
                        .iter()
                        .all(|source| *last_pulse.get(source).unwrap_or(&false))
                }
                Node::Broadcaster => pulse,
            };

            last_pulse.insert(pos, next_pulse);
            if next_pulse {
                high_pulses += cable.dest.len();
            } else {
                low_pulses += cable.dest.len();
            }
            for &other in &cable.dest {
                todo.push_back((next_pulse, other));
            }
        }
    }

    Ok((low_pulses * high_pulses).to_string())
}

pub fn part2(input: &[u8]) -> anyhow::Result<String> {
    let mut cables = parse_input(input, parse_cables)?;

    let mut todo = VecDeque::new();
    let mut last_pulse: HashMap<u32, bool> = HashMap::new();

    for press in 1u64.. {
        todo.push_back((false, convert_name(b"broadcaster")));

        while let Some((pulse, pos)) = todo.pop_front() {
            let Some(cable) = cables.get_mut(&pos) else {
                // Sometimes cables aren't real, and that's okay
                continue;
            };

            if !pulse && pos == convert_name(b"rx") {
                return Ok(press.to_string());
            }

            let next_pulse = match &mut cable.node {
                Node::FlipFlop(state) => {
                    if pulse {
                        // Ignore, nothing to be done since it's a high pulse
                        continue;
                    } else {
                        *state = !*state;
                        *state
                    }
                }
                Node::Conjunction(inwards) => {
                    // Need to deal with the check outside the match otherwise lifetime issues :(
                    !inwards
                        .iter()
                        .all(|source| *last_pulse.get(source).unwrap_or(&false))
                }
                Node::Broadcaster => pulse,
            };

            last_pulse.insert(pos, next_pulse);
            for &other in &cable.dest {
                todo.push_back((next_pulse, other));
            }
        }
    }

    anyhow::bail!("Somehow counted to infinity")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &[u8] = include_bytes!("samples/20.1.txt");
    const SAMPLE2: &[u8] = include_bytes!("samples/20.2.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("32000000", part1(SAMPLE1).unwrap());
        assert_eq!("11687500", part1(SAMPLE2).unwrap());
    }
}
