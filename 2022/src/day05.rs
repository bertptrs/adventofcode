use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::bytes::complete::take_until;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::fold_many1;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use crate::common::enumerate;
use crate::common::get_both;
use crate::common::parse_input;

type Move = (usize, usize, usize);
type OwnedStacks = Vec<Vec<u8>>;

fn parse_row<'a>(input: &'a [u8], stacks: &mut OwnedStacks) -> IResult<&'a [u8], ()> {
    // Forgive me for this crime
    fold_many1(
        enumerate(terminated(
            alt((
                // Parse a delimited value into a Some(content)
                map(delimited(tag("["), take(1usize), tag("]")), |v: &[u8]| {
                    Some(v[0])
                }),
                // Or an empty stack into a None
                map(tag("   "), |_| None),
            )),
            opt(tag(" ")),
        )),
        || (),
        move |_, (index, c)| {
            if let Some(b) = c {
                if stacks.len() <= index {
                    stacks.resize_with(index + 1, Vec::new);
                }

                stacks[index].push(b)
            }
        },
    )(input)
}

fn parse_stacks(input: &[u8]) -> IResult<&[u8], OwnedStacks> {
    let mut stacks = Vec::new();

    let (input, _) = terminated(
        fold_many1(
            terminated(|input| parse_row(input, &mut stacks), newline),
            || (),
            |_, _| (),
        ),
        // Skip the line with the numbers
        take_until("\n\n"),
    )(input)?;

    // Reverse the stacks since we parsed them top-down
    for stack in &mut stacks {
        stack.reverse();
    }

    Ok((input, stacks))
}

fn parse_task(input: &[u8]) -> IResult<&[u8], (OwnedStacks, Vec<Move>)> {
    fn parse_usize(input: &[u8]) -> IResult<&[u8], usize> {
        map(nom::character::complete::u32, |v| v as usize)(input)
    }
    let (input, stacks) = parse_stacks(input)?;

    // Consume the double newline
    let (input, _) = tag("\n\n")(input)?;

    let (input, moves) = many1(terminated(
        tuple((
            preceded(tag("move "), parse_usize),
            preceded(tag(" from "), parse_usize),
            preceded(tag(" to "), parse_usize),
        )),
        newline,
    ))(input)?;

    Ok((input, (stacks, moves)))
}

fn compute_answer(stacks: &mut [Vec<u8>]) -> Result<String> {
    let mut result = String::with_capacity(stacks.len());

    for stack in stacks {
        result.push(
            *stack
                .last()
                .ok_or_else(|| anyhow::anyhow!("Encountered empty stack"))? as char,
        );
    }

    Ok(result)
}

pub fn part1(input: &[u8]) -> Result<String> {
    let (mut stacks, moves) = parse_input(input, parse_task)?;

    for (count, from, to) in moves {
        let (from, to) = get_both(&mut stacks, from - 1, to - 1);

        let drain_start = from.len() - count;

        to.extend(from.drain(drain_start..).rev());
    }

    compute_answer(&mut stacks)
}

pub fn part2(input: &[u8]) -> Result<String> {
    let (mut stacks, moves) = parse_input(input, parse_task)?;

    for (count, from, to) in moves {
        let (from, to) = get_both(&mut stacks, from - 1, to - 1);

        let drain_start = from.len() - count;

        to.extend(from.drain(drain_start..));
    }

    compute_answer(&mut stacks)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/05.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "CMZ");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "MCD");
    }
}
