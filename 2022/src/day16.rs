use std::ops::Deref;

use ahash::AHashMap;
use anyhow::Result;
use ndarray::Array2;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::newline;
use nom::combinator::into;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use crate::common::parse_input;

type ParsedValve<'a> = (&'a [u8], u16, Vec<&'a [u8]>);

type ParsedNetwork<'a> = Vec<ParsedValve<'a>>;

#[derive(Debug)]
struct SimpleNetwork {
    valves: Vec<SimpleValve>,
    start: usize,
    useful: usize,
}

impl Deref for SimpleNetwork {
    type Target = [SimpleValve];

    fn deref(&self) -> &Self::Target {
        &self.valves
    }
}

#[derive(Debug)]
struct SimpleValve {
    connected: Vec<usize>,
    flow: u16,
}

impl From<ParsedNetwork<'_>> for SimpleNetwork {
    fn from(mut parsed: ParsedNetwork) -> Self {
        // Make sure the positive numbers are in the front
        parsed.sort_by(|a, b| b.1.cmp(&a.1));

        let mapping: AHashMap<_, _> = parsed
            .iter()
            .enumerate()
            .map(|(index, (name, _, _))| (*name, index))
            .collect();

        let useful = parsed.iter().filter(|(_, flow, _)| *flow > 0).count();

        Self {
            valves: parsed
                .into_iter()
                .map(|(_, flow, connected)| {
                    let connected = connected.into_iter().map(|name| mapping[&name]).collect();

                    SimpleValve { connected, flow }
                })
                .collect(),
            start: mapping[&b"AA"[..]],
            useful,
        }
    }
}

fn parse_network(input: &[u8]) -> IResult<&[u8], ParsedNetwork> {
    let parse_network = terminated(
        tuple((
            // Parse the name of the valve
            preceded(tag("Valve "), alpha1),
            // Parse the flow of the valve
            preceded(tag(" has flow rate="), nom::character::complete::u16),
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

    many1(parse_network)(input)
}

pub fn part1(input: &[u8]) -> Result<String> {
    let network: SimpleNetwork = parse_input(input, into(parse_network))?;

    let (valves_available, dp) = run_optimization(&network, 30);

    Ok(dp[(network.start, valves_available)].to_string())
}

fn run_optimization(network: &SimpleNetwork, time: usize) -> (usize, Array2<u16>) {
    let valves_available = (1 << network.useful) - 1;
    let mut cur = Array2::zeros((network.len(), valves_available + 1));
    let mut prev = cur.clone();

    for time_remaining in 1..time {
        for pos in 0..network.len() {
            let bit = 1 << pos;
            for open_valves in 0..=valves_available {
                let optimal = if (bit & open_valves) != 0 && time_remaining > 2 {
                    prev[(pos, open_valves - bit)] + time_remaining as u16 * network[pos].flow
                } else {
                    0
                };

                cur[(pos, open_valves)] = network[pos]
                    .connected
                    .iter()
                    .map(|&other| prev[(other, open_valves)])
                    .fold(optimal, Ord::max);
            }
        }

        std::mem::swap(&mut prev, &mut cur);
    }
    (valves_available, prev)
}

pub fn part2(input: &[u8]) -> Result<String> {
    let network: SimpleNetwork = parse_input(input, into(parse_network))?;

    let (valves_available, dp) = run_optimization(&network, 26);

    // Find the minimum of all combinations of your work/elephant's work
    let best = (0..=valves_available)
        .map(|my_valves| {
            let elephant_valves = valves_available - my_valves;

            dp[(network.start, my_valves)] + dp[(network.start, elephant_valves)]
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
