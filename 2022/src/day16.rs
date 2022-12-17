use std::collections::VecDeque;
use std::ops::Deref;

use ahash::AHashMap;
use ahash::AHashSet;
use anyhow::Result;
use ndarray::Array3;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::newline;
use nom::combinator::into;
use nom::multi::fold_many1;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use crate::common::parse_input;

type ParsedNetwork<'a> = AHashMap<&'a [u8], ParsedValve<'a>>;

struct ParsedValve<'a> {
    connected: Vec<&'a [u8]>,
    flow: u32,
}

#[derive(Debug)]
struct SimpleNetwork {
    valves: Vec<SimpleValve>,
    start: usize,
}

impl Deref for SimpleNetwork {
    type Target = [SimpleValve];

    fn deref(&self) -> &Self::Target {
        &*self.valves
    }
}

#[derive(Debug)]
struct SimpleValve {
    connected: Vec<(usize, u8)>,
    flow: u32,
}

impl From<ParsedNetwork<'_>> for SimpleNetwork {
    fn from(parsed: ParsedNetwork) -> Self {
        let mapping: AHashMap<_, _> = parsed
            .iter()
            .filter_map(|(&k, v)| (v.flow > 0 || k == b"AA").then_some(k))
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect();

        let mut todo = VecDeque::new();
        let mut seen = AHashSet::new();

        let mut network = Vec::with_capacity(mapping.len());

        for (&key, valve_data) in &parsed {
            if valve_data.flow == 0 && key != b"AA" {
                continue;
            }

            todo.extend(valve_data.connected.iter().map(|&valve| (valve, 1)));

            let mut connected = Vec::new();

            seen.clear();
            while let Some((valve, dist)) = todo.pop_front() {
                if seen.insert(valve) {
                    let data = &parsed[&valve];

                    if data.flow != 0 {
                        connected.push((mapping[valve], dist));
                    }
                    for &other in &data.connected {
                        if other != key {
                            todo.push_back((other, dist + 1));
                        }
                    }
                }
            }

            network.push(SimpleValve {
                flow: valve_data.flow,
                connected,
            })
        }

        Self {
            valves: network,
            start: mapping[&b"AA"[..]],
        }
    }
}

fn parse_network(input: &[u8]) -> IResult<&[u8], ParsedNetwork> {
    let parse_network = terminated(
        tuple((
            // Parse the name of the valve
            preceded(tag("Valve "), alpha1),
            // Parse the flow of the valve
            preceded(tag(" has flow rate="), nom::character::complete::u32),
            // Parse the connections
            preceded(
                // Did you really have to distinguish plural
                alt((
                    tag("; tunnels lead to valves "),
                    tag("; tunnel leads to valve "),
                )),
                separated_list1(tag(", "), alpha1),
            ),
        )),
        newline,
    );

    fold_many1(
        parse_network,
        ParsedNetwork::new,
        |mut map, (valve, flow, connected)| {
            map.insert(valve, ParsedValve { flow, connected });

            map
        },
    )(input)
}

pub fn part1(input: &[u8]) -> Result<String> {
    let network: SimpleNetwork = parse_input(input, into(parse_network))?;

    let (valves_available, dp) = run_optimization(&network, 30);

    // Guesses: 1802 (too low)
    Ok(dp[(29, network.start, valves_available)].to_string())
}

fn run_optimization(network: &SimpleNetwork, time: usize) -> (usize, Array3<u16>) {
    let num_valves = network.len();
    let valves_available = (1 << num_valves) - 1;
    let mut dp = Array3::<u16>::zeros((time, network.len(), valves_available + 1));
    for time_remaining in 1..time {
        for pos in 0..network.len() {
            let bit = 1 << pos;
            for open_valves in 0..=valves_available {
                let mut optimal = if (bit & open_valves) != 0 && time_remaining > 2 {
                    dp[(time_remaining - 1, pos, open_valves - bit)]
                        + time_remaining as u16 * network[pos].flow as u16
                } else {
                    0
                };

                for &(other, dist) in &*network[pos].connected {
                    let dist = usize::from(dist);
                    if dist <= time_remaining {
                        optimal = optimal.max(dp[(time_remaining - dist, other, open_valves)]);
                    }
                }

                dp[(time_remaining, pos, open_valves)] = optimal;
            }
        }
    }
    (valves_available, dp)
}

pub fn part2(input: &[u8]) -> Result<String> {
    let network: SimpleNetwork = parse_input(input, into(parse_network))?;

    let (valves_available, dp) = run_optimization(&network, 26);

    // Find the minimum of all combinations of your work/elephant's work
    let best = (0..=valves_available)
        .map(|my_valves| {
            let elephant_valves = valves_available - my_valves;

            dp[(25, network.start, my_valves)] + dp[(25, network.start, elephant_valves)]
        })
        .max()
        .unwrap_or(0);

    // Guesses: 1802 (too low)
    Ok(best.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/16.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "1651");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "1707");
    }
}
