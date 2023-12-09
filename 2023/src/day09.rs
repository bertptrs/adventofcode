use std::mem;

use nom::bytes::complete::tag;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::IResult;

use crate::common::parse_input;

fn parse_reports(i: &[u8]) -> IResult<&[u8], Vec<Vec<i32>>> {
    many1(terminated(
        separated_list1(tag(" "), nom::character::complete::i32),
        tag("\n"),
    ))(i)
}

fn compute_next<'a>(report: impl IntoIterator<Item = &'a i32>) -> i32 {
    let mut deltas = Vec::new();

    for &entry in report {
        let mut delta = entry;
        for prev_delta in &mut deltas {
            let prev = mem::replace(prev_delta, delta);
            delta = delta - prev;
        }

        if delta != 0 {
            deltas.push(delta);
        }
    }

    deltas.drain(..).rev().fold(0, |c, d| c + d)
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let reports = parse_input(input, parse_reports)?;
    let result: i32 = reports
        .iter()
        .map(|report| compute_next(report.iter()))
        .sum();
    Ok(result.to_string())
}

pub fn part2(input: &[u8]) -> anyhow::Result<String> {
    let mut reports = parse_input(input, parse_reports)?;
    let result: i32 = reports
        .iter_mut()
        .map(|report| compute_next(report.iter().rev()))
        .sum();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/09.txt");

    #[test]
    fn samples_separate() {
        assert_eq!(18, compute_next(&[0, 3, 6, 9, 12, 15]));
        assert_eq!(28, compute_next(&[1, 3, 6, 10, 15, 21]));
        assert_eq!(68, compute_next(&[10, 13, 16, 21, 30, 45]));
    }

    #[test]
    fn sample_part1() {
        assert_eq!("114", part1(SAMPLE).unwrap());
    }

    #[test]
    fn sample_part2() {
        assert_eq!("2", part2(SAMPLE).unwrap());
    }
}
