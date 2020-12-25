use std::collections::HashMap;
use std::convert::Infallible;
use std::io::Read;
use std::str::FromStr;

use crate::common::from_lines;
use crate::Solution;

enum Entry {
    Mask(u64, u64, u64),
    Set(u64, u64),
}

impl FromStr for Entry {
    // You're always infallible if you just crash when you fail
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask = ") {
            let mut zero_mask = 0;
            let mut one_mask = 0;
            let mut x_mask = 0;

            for c in s[7..].chars() {
                zero_mask <<= 1;
                one_mask <<= 1;
                x_mask <<= 1;

                match c {
                    '0' => {}
                    '1' => {
                        zero_mask |= 1;
                        one_mask |= 1;
                    }
                    'X' => {
                        x_mask |= 1;
                        zero_mask |= 1;
                    }
                    _ => panic!("Invalid mask character {}", c),
                }
            }

            Ok(Entry::Mask(zero_mask, one_mask, x_mask))
        } else {
            let mut nums = s
                .split(|c: char| !c.is_ascii_digit())
                .filter_map(|s| s.parse().ok());

            let pos = nums.next().unwrap();
            let val = nums.next().unwrap();

            Ok(Entry::Set(pos, val))
        }
    }
}

fn x_mask_permutations(mut x_mask: u64, permutations: &mut Vec<u64>) {
    permutations.clear();
    permutations.reserve(1 << x_mask.count_ones());

    permutations.push(0);

    let mut offset = 0;

    while x_mask > 0 {
        let trailing = x_mask.trailing_zeros();

        let bit = 1 << (trailing + offset);

        x_mask >>= trailing + 1;
        offset += trailing + 1;

        for i in 0..permutations.len() {
            let new_permutation = permutations[i] | bit;
            permutations.push(new_permutation);
        }
    }
}

#[derive(Default)]
pub struct Day14;

impl Solution for Day14 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let instructions: Vec<Entry> = from_lines(input);

        let mut mem = HashMap::new();

        let mut zero_mask = (1 << 35) - 1;
        let mut one_mask = 0;

        for entry in instructions {
            match entry {
                Entry::Mask(z, o, _) => {
                    zero_mask = z;
                    one_mask = o;
                }
                Entry::Set(pos, val) => {
                    mem.insert(pos, (val & zero_mask) | one_mask);
                }
            }
        }

        mem.values().sum::<u64>().to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let instructions: Vec<Entry> = from_lines(input);

        let mut mem = HashMap::new();

        let mut one_mask = 0;
        let mut permutations = vec![0];

        for entry in instructions {
            match entry {
                Entry::Mask(_, o, x) => {
                    one_mask = o;
                    x_mask_permutations(x, &mut permutations);
                }
                Entry::Set(pos, val) => {
                    let pos = pos | one_mask;

                    for &p in &permutations {
                        mem.insert(pos ^ p, val);
                    }
                }
            }
        }

        mem.values().sum::<u64>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../samples/14.txt");
    const SAMPLE2: &[u8] = include_bytes!("../samples/14.2.txt");

    #[test]
    fn sample_part1() {
        test_implementation(Day14, 1, SAMPLE, 165);
    }

    #[test]
    fn sample_part2() {
        test_implementation(Day14, 2, SAMPLE2, 208);
    }

    #[test]
    fn test_x_mask_permutations() {
        let mut permutations = Vec::new();
        x_mask_permutations(0b101001, &mut permutations);

        assert_eq!(permutations, [0, 1, 8, 9, 32, 33, 40, 41]);
    }
}
