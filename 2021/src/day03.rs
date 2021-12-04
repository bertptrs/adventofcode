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

fn find_remaining(mut strings: Vec<u32>, most: bool, len: usize) -> u32 {
    for pos in 1..=len {
        if strings.len() == 1 {
            break;
        }

        let bit = 1 << (len - pos);

        let occurrences = strings.iter().filter(|&&b| (b & bit) == bit).count();

        let keep = if (occurrences * 2 < strings.len()) ^ most {
            bit
        } else {
            0
        };

        strings.retain(|&b| (b & bit) == keep);
    }

    strings[0]
}

pub fn part2(input: &mut dyn Read) -> String {
    let mut strings = Vec::new();
    let mut reader = LineIter::new(input);
    let mut read_line = reader.next();
    let len = read_line.unwrap().len();

    while let Some(line) = read_line {
        strings.push(u32::from_str_radix(line, 2).unwrap());
        read_line = reader.next();
    }

    let oxygen = find_remaining(strings.clone(), true, len);
    let co2 = find_remaining(strings, false, len);

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
