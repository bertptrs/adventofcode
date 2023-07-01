use core::slice;
use std::cmp::Ordering;

use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::character::complete::newline;
use nom::combinator::iterator;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::IResult;

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
            (first @ Signal::Number(_), Signal::List(second)) => {
                list_cmp(slice::from_ref(first), second)
            }
            (Signal::List(first), second @ Signal::Number(_)) => {
                list_cmp(first, slice::from_ref(second))
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

pub fn part2(input: &[u8]) -> Result<String> {
    let marker1 = Signal::Number(2);
    let marker2 = Signal::Number(6);

    let mut iterator = iterator(input, terminated(parse_signal, multispace1));

    let mut pos1 = 1;
    let mut pos2 = 2;

    for signal in &mut iterator {
        if signal < marker1 {
            pos1 += 1;
            pos2 += 1;
        } else if signal < marker2 {
            pos2 += 1;
        }
    }

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
