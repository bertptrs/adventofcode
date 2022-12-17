use std::collections::VecDeque;

use ahash::AHashMap;
use ahash::AHashSet;
use anyhow::Result;
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

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct State {
    pos: usize,
    valves_open: u32,
}

impl State {
    fn open(&self) -> Option<State> {
        let bit = 1 << self.pos;
        if (self.valves_open & bit) == 0 {
            Some(State {
                pos: self.pos,
                valves_open: self.valves_open | bit,
            })
        } else {
            None
        }
    }

    pub fn is_open(&self, pos: usize) -> bool {
        (self.valves_open & (1 << pos)) != 0
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reversed because having fewer valves with the same score gives more opportunities for gains
        self.valves_open
            .count_ones()
            .cmp(&other.valves_open.count_ones())
            // Compare open valves and pos. Shouldn't really matter but required for a total order.
            .then(self.valves_open.cmp(&other.valves_open))
            .then(self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(input: &[u8]) -> Result<String> {
    let network: SimpleNetwork = parse_input(input, into(parse_network))?;

    let mut best = AHashMap::new();

    let initial_state = State {
        valves_open: 0,
        pos: network.start,
    };

    best.insert(initial_state.clone(), 0);

    let mut todo = VecDeque::new();

    todo.push_back((0, 0, initial_state));

    let mut best_score = 0;

    while let Some((score, minute, state)) = todo.pop_front() {
        if best[&state] > score {
            continue;
        }

        let mut enqueue = |score, minute, state: State| {
            if minute >= 29
                || best
                    .get(&state)
                    .map(|&previous| previous >= score)
                    .unwrap_or(false)
            {
                return;
            }

            best.insert(state.clone(), score);
            todo.push_back((score, minute, state));
        };

        if let Some(new_state) = state.open() {
            let pos = new_state.pos;
            let valve_strength = network.valves[pos].flow;
            let time_remaining = 29 - minute;
            let new_score = score + time_remaining as u32 * network.valves[pos].flow;

            println!("Opening valve {pos} for {valve_strength} for {time_remaining} minutes: {new_score} ({best_score})");

            best_score = best_score.max(new_score);

            enqueue(new_score, minute + 1, new_state)
        }

        for &(other, dist) in &network.valves[state.pos].connected {
            if state.is_open(other) {
                continue;
            }

            let new_state = State {
                pos: other,
                valves_open: state.valves_open,
            };

            enqueue(score, minute + dist, new_state);
        }
    }

    // Guesses: 1802 (too low)
    Ok(best_score.to_string())
}

pub fn part2(_input: &[u8]) -> Result<String> {
    anyhow::bail!("not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/16.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "1651");
    }
}
