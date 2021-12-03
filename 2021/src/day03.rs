use std::io::Read;

use crate::common::LineIter;

fn parse_bit(bit: char) -> usize {
    if bit == '1' {
        1
    } else {
        0
    }
}

pub fn part1(input: &mut dyn Read) -> String {
    let mut reader = LineIter::new(input);

    let mut totals: Vec<usize> = reader.next().unwrap().chars().map(parse_bit).collect();

    let mut count = 1;

    while let Some(line) = reader.next() {
        line.chars()
            .map(parse_bit)
            .zip(totals.iter_mut())
            .for_each(|(b, t)| *t += b);

        count += 1;
    }

    let bitmask = (1 << totals.len()) - 1;

    let gamma = totals.into_iter().fold(0, |mut current, total| {
        current <<= 1;
        if total > count / 2 {
            current |= 1;
        };

        current
    });

    let epsilon = (!gamma) & bitmask;

    (gamma * epsilon).to_string()
}

fn find_remaining(mut strings: Vec<Vec<u8>>, most: bool) -> u32 {
    let comp: fn(usize, usize) -> bool = if most {
        |occurrences, len| occurrences * 2 >= len
    } else {
        |occurrences, len| occurrences * 2 < len
    };

    for pos in 0..strings[0].len() {
        if strings.len() == 1 {
            break;
        }

        let occurrences = strings.iter().filter(|b| b[pos] == b'1').count();

        let keep = if comp(occurrences, strings.len()) {
            b'1'
        } else {
            b'0'
        };

        strings.retain(|s| s[pos] == keep);
    }

    strings[0]
        .iter()
        .fold(0, |n, &b| (n << 1) | (b - b'0') as u32)
}

pub fn part2(input: &mut dyn Read) -> String {
    let mut strings = Vec::new();
    let mut reader = LineIter::new(input);

    while let Some(line) = reader.next() {
        strings.push(line.as_bytes().to_owned());
    }

    let oxygen = find_remaining(strings.clone(), true);
    let co2 = find_remaining(strings, false);

    (oxygen * co2).to_string()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/03.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 198);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 230)
    }
}
