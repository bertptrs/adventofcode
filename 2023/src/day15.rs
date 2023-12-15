use linked_hash_map::LinkedHashMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_till;

use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;

use crate::common::parse_input;

fn trim(input: &[u8]) -> &[u8] {
    let whitespace = input
        .iter()
        .rev()
        .take_while(|c| c.is_ascii_whitespace())
        .count();

    &input[..(input.len() - whitespace)]
}

fn hash(input: &[u8]) -> u8 {
    input
        .iter()
        .fold(0, |cur, &c| cur.wrapping_add(c).wrapping_mul(17))
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let input = trim(input);

    Ok(input
        .split(|&c| c == b',')
        .map(|word| u32::from(hash(word)))
        .sum::<u32>()
        .to_string())
}

enum Command<'a> {
    Add(&'a [u8], u32),
    Remove(&'a [u8]),
}
fn parse_commands(i: &[u8]) -> IResult<&[u8], Vec<Command>> {
    fn is_op(c: u8) -> bool {
        c == b'=' || c == b'-'
    }

    separated_list1(
        tag(","),
        alt((
            map(
                separated_pair(take_till(is_op), tag("="), nom::character::complete::u32),
                |(a, b)| Command::Add(a, b),
            ),
            map(terminated(take_till(is_op), tag("-")), Command::Remove),
        )),
    )(i)
}

pub fn part2(input: &[u8]) -> anyhow::Result<String> {
    let commands = parse_input(trim(input), parse_commands)?;
    let mut state = LinkedHashMap::new();

    for command in commands {
        match command {
            Command::Add(identifier, focal_len) => {
                *state.entry(identifier).or_default() = focal_len;
            }
            Command::Remove(identifier) => {
                state.remove(identifier);
            }
        }
    }

    let mut box_slot = [0; 256];
    let mut total = 0;

    for (&identifier, &focal_len) in &state {
        let index = hash(identifier);
        let box_no = u32::from(index) + 1;
        let slot_no = &mut box_slot[index as usize];
        *slot_no += 1;
        total += box_no * *slot_no * focal_len;
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/15.txt");

    #[test]
    fn sample_hash() {
        assert_eq!(hash(b"HASH"), 52);
    }

    #[test]
    fn sample_part1() {
        assert_eq!("1320", part1(SAMPLE).unwrap());
    }

    #[test]
    fn sample_part2() {
        assert_eq!("145", part2(SAMPLE).unwrap());
    }
}
