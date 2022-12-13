use std::cmp::Ordering;

use anyhow::Context;
use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::character::complete::newline;
use nom::combinator::iterator;
use nom::combinator::map;
use nom::multi::many0;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::IResult;

use crate::common::parse_input;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Signal {
    Number(u32),
    List(Vec<Signal>),
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> Ordering {
        fn list_cmp(a: &[Signal], b: &[Signal]) -> Ordering {
            a.cmp(b)
        }

        match (self, other) {
            (Signal::Number(first), Signal::Number(second)) => first.cmp(second),
            (Signal::Number(first), Signal::List(second)) => {
                list_cmp(&[Signal::Number(*first)], second)
            }
            (Signal::List(first), Signal::Number(second)) => {
                list_cmp(first, &[Signal::Number(*second)])
            }
            (Signal::List(first), Signal::List(second)) => list_cmp(first, second),
        }
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<u32> for Signal {
    fn from(val: u32) -> Self {
        Signal::Number(val)
    }
}

impl<T> From<Vec<T>> for Signal
where
    Signal: From<T>,
{
    fn from(vec: Vec<T>) -> Self {
        Signal::List(vec.into_iter().map(Signal::from).collect())
    }
}

fn parse_signal(input: &[u8]) -> IResult<&[u8], Signal> {
    alt((
        map(nom::character::complete::u32, Signal::Number),
        map(
            delimited(tag("["), separated_list0(tag(","), parse_signal), tag("]")),
            Signal::List,
        ),
    ))(input)
}

fn parse_signal_pair(input: &[u8]) -> IResult<&[u8], (Signal, Signal)> {
    pair(
        terminated(parse_signal, newline),
        terminated(parse_signal, multispace1),
    )(input)
}

pub fn part1(input: &[u8]) -> Result<String> {
    let mut iterator = iterator(input, parse_signal_pair);

    let result: usize = (&mut iterator)
        .enumerate()
        .filter_map(|(i, (first, second))| (first < second).then_some(i + 1))
        .sum();

    Ok(result.to_string())
}

fn parse_signals(input: &[u8]) -> IResult<&[u8], Vec<Signal>> {
    many0(terminated(parse_signal, multispace1))(input)
}

pub fn part2(input: &[u8]) -> Result<String> {
    let marker1 = Signal::from(vec![vec![2]]);
    let marker2 = Signal::from(vec![vec![6]]);

    let mut signals = parse_input(input, parse_signals)?;

    signals.push(marker1.clone());
    signals.push(marker2.clone());

    signals.sort_unstable();

    let pos1 = signals
        .iter()
        .position(|v| v == &marker1)
        .context("Cannot find marker 1")?
        + 1;

    let pos2 = pos1
        + signals[pos1..]
            .iter()
            .position(|v| v == &marker2)
            .context("Cannot find marker 2")?
        + 1;

    Ok((pos1 * pos2).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/13.txt");

    #[test]
    fn test_parse_signal() {
        assert_eq!(
            parse_signal(b"[1,1,3,1,1]").unwrap(),
            (&[][..], Signal::from(vec![1, 1, 3, 1, 1]))
        );
    }

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "13");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "140");
    }
}
