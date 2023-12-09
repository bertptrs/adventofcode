use std::mem;
use std::ops::Range;

use nom::IResult;

use crate::common::parse_input;

fn parse_reports(mut i: &[u8]) -> IResult<&[u8], (Vec<Range<usize>>, Vec<i32>)> {
    let mut begin = 0;
    let mut numbers = Vec::new();
    let mut ranges = Vec::new();
    while !i.is_empty() {
        let (rem, num) = nom::character::complete::i32(i)?;
        numbers.push(num);

        if rem[0] == b'\n' {
            let end = numbers.len();
            ranges.push(begin..end);
            begin = end;
        }

        i = &rem[1..];
    }

    Ok((i, (ranges, numbers)))
}

fn compute_next<'a>(report: impl IntoIterator<Item = &'a i32>, deltas: &mut Vec<i32>) -> i32 {
    deltas.clear();

    for &entry in report {
        let mut delta = entry;
        for prev_delta in &mut *deltas {
            let prev = mem::replace(prev_delta, delta);
            delta = delta - prev;
        }

        if delta != 0 {
            deltas.push(delta);
        }
    }

    deltas.iter().rev().fold(0, |c, d| c + d)
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let mut deltas = Vec::new();
    let (ranges, numbers) = parse_input(input, parse_reports)?;
    let result: i32 = ranges
        .into_iter()
        .map(|range| compute_next(&numbers[range], &mut deltas))
        .sum();
    Ok(result.to_string())
}

pub fn part2(input: &[u8]) -> anyhow::Result<String> {
    let mut deltas = Vec::new();
    let (ranges, numbers) = parse_input(input, parse_reports)?;
    let result: i32 = ranges
        .into_iter()
        .map(|range| compute_next(numbers[range].iter().rev(), &mut deltas))
        .sum();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/09.txt");

    #[test]
    fn samples_separate() {
        assert_eq!(18, compute_next(&[0, 3, 6, 9, 12, 15], &mut Vec::new()));
        assert_eq!(28, compute_next(&[1, 3, 6, 10, 15, 21], &mut Vec::new()));
        assert_eq!(68, compute_next(&[10, 13, 16, 21, 30, 45], &mut Vec::new()));
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
