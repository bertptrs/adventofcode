use std::collections::VecDeque;
use std::io::Read;
use std::num::NonZeroU8;

use crate::common::LineIter;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Segments(NonZeroU8);

impl Segments {
    pub fn overlap(self, other: Segments) -> u32 {
        (self.0 | other.0).get().count_ones()
    }

    pub fn len(self) -> u32 {
        self.0.get().count_ones()
    }
}

impl<'a> From<&'a str> for Segments {
    fn from(s: &'a str) -> Self {
        let mut buffer = 0;

        for &b in s.as_bytes() {
            debug_assert!((b'a'..=b'g').contains(&b));

            buffer |= 1 << (b - b'a');
        }

        Self(NonZeroU8::new(buffer).unwrap())
    }
}

pub fn part1(input: &mut dyn Read) -> String {
    let mut reader = LineIter::new(input);

    let mut total = 0;

    while let Some(line) = reader.next() {
        total += line
            .split(' ')
            .skip_while(|&s| s != "|")
            .filter(|s| [2, 3, 4, 7].contains(&s.len()))
            .count();
    }

    total.to_string()
}

fn decode(line: &str, unmatched: &mut VecDeque<Segments>) -> usize {
    let mut mapping = [None; 10];

    unmatched.clear();
    unmatched.extend(line.split(' ').filter(|&s| s != "|").map(Segments::from));

    while let Some(segments) = unmatched.pop_front() {
        // Note: this loop might "deduce" a combination more than once, but deducing digits is
        // idempotent so it does not interfere.
        match segments.len() {
            2 => mapping[1] = Some(segments),
            3 => mapping[7] = Some(segments),
            4 => mapping[4] = Some(segments),
            5 => {
                // Could be 2, 3, or 5
                if let Some(one) = mapping[1] {
                    if segments.overlap(one) == 5 {
                        // No lines added, so must be a three
                        mapping[3] = Some(segments);
                        continue;
                    } else if let Some(four) = mapping[4] {
                        // Should be 6 for 5 and 7 for 2
                        if segments.overlap(four) == 6 {
                            mapping[5] = Some(segments);
                        } else {
                            mapping[2] = Some(segments);
                        }
                        continue;
                    }
                }
                unmatched.push_back(segments);
            }
            6 => {
                // Could be 0, 6, or 9
                if let Some(one) = mapping[1] {
                    if segments.overlap(one) == 7 {
                        mapping[6] = Some(segments);
                        continue;
                    } else if let Some(four) = mapping[4] {
                        if segments.overlap(four) == 6 {
                            mapping[9] = Some(segments);
                        } else {
                            mapping[0] = Some(segments);
                        }
                        continue;
                    }
                    unmatched.push_back(segments);
                }
            }
            7 => mapping[8] = Some(segments),
            _ => panic!("Invalid segments!"),
        }
    }

    line.split(' ')
        .skip_while(|&s| s != "|")
        .skip(1)
        .map(|s| {
            let segments = Segments::from(s);
            mapping.iter().position(|s| &Some(segments) == s).unwrap()
        })
        .fold(0, |acc, n| acc * 10 + n)
}

pub fn part2(input: &mut dyn Read) -> String {
    let mut reader = LineIter::new(input);
    let mut total = 0;

    // Allocate work memory outside the decode function so we can reuse it
    let mut work_buffer = VecDeque::new();

    while let Some(line) = reader.next() {
        total += decode(line, &mut work_buffer);
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE1: &[u8] =
        &*b"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    const SAMPLE2: &[u8] = include_bytes!("samples/08.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE1, 0);
        test_implementation(part1, SAMPLE2, 26);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE1, 5353);
        test_implementation(part2, SAMPLE2, 61229);
    }
}
