use std::cmp::Ordering;
use std::iter::once;

use anyhow::Context;
use anyhow::Result;
use nom::character::complete::newline;
use nom::multi::many0;
use nom::sequence::terminated;
use nom::IResult;

use crate::common::parse_input;

fn parse_encrypted(input: &[u8]) -> IResult<&[u8], Vec<i64>> {
    many0(terminated(nom::character::complete::i64, newline))(input)
}

// While looping around to find a spot to move to, you must ignore the piece itself, but for
// computing the answer, you shouldn't. This function does the latter.
fn step(steps: &[usize], mut start: usize, count: usize) -> usize {
    for _ in 0..(count % steps.len()) {
        start = steps[start];
    }

    start
}

fn move_between(prev: &mut [usize], next: &mut [usize], i: usize, before: usize, after: usize) {
    if before == i || after == i {
        return;
    }

    let before_i = prev[i];
    let after_i = next[i];

    // Remove i from its original place
    prev[after_i] = before_i;
    next[before_i] = after_i;

    // Connect i properly to before
    prev[before] = i;
    next[i] = before;

    // Connect i properly to after
    prev[i] = after;
    next[after] = i;
}

pub fn part1(input: &[u8]) -> Result<String> {
    let encrypted = parse_input(input, parse_encrypted)?;

    shuffle(&encrypted, 1)
}

fn shuffle(encrypted: &[i64], times: usize) -> Result<String> {
    let mut next: Vec<_> = (1..encrypted.len()).chain(once(0)).collect();
    let mut prev: Vec<_> = once(encrypted.len() - 1)
        .chain(0..(encrypted.len() - 1))
        .collect();

    let len = encrypted.len() as i64;
    let half_len = len / 2;

    for _ in 0..times {
        for (i, &value) in encrypted.iter().enumerate() {
            let mut value = value % (len - 1);

            if value < -half_len {
                value += len - 1;
            } else if value > half_len {
                value -= len - 1;
            }

            match value.cmp(&0) {
                Ordering::Less => {
                    let before = step(&prev, i, (-value) as usize);
                    let after = prev[before];

                    move_between(&mut prev, &mut next, i, before, after);
                }
                Ordering::Equal => continue,
                Ordering::Greater => {
                    let after = step(&next, i, value as usize);
                    let before = next[after];

                    move_between(&mut prev, &mut next, i, before, after);
                }
            }

            // print(&encrypted, &next);
        }
    }

    let mut start = encrypted
        .iter()
        .position(|&v| v == 0)
        .context("Could not find zero")?;

    let mut sum = 0;

    for _ in 0..3 {
        start = step(&next, start, 1000);
        sum += encrypted[start];
    }

    Ok(sum.to_string())
}

pub fn part2(input: &[u8]) -> Result<String> {
    const ENCRYPTION_KEY: i64 = 811_589_153;

    let mut encrypted = parse_input(input, parse_encrypted)?;

    encrypted.iter_mut().for_each(|v| *v *= ENCRYPTION_KEY);

    shuffle(&encrypted, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/20.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "3");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "1623178306");
    }
}
