use std::collections::HashMap;
use std::io::Read;

use crate::common::from_lines;
use crate::Solution;

const MOD_BASE: u32 = 20201227;
const SUBJECT_NUMBER: u32 = 7;

fn loop_count(public_key: u32) -> u64 {
    discrete_log(SUBJECT_NUMBER, public_key, MOD_BASE).unwrap() as u64
}

// Implementation of the baby-step giant-step algorithm
//
// Based on:https://en.wikipedia.org/wiki/Baby-step_giant-step#C++_algorithm_(C++17)
fn discrete_log(g: u32, h: u32, mod_base: u32) -> Option<u32> {
    let m = (mod_base as f64).sqrt().ceil() as u32;
    let mut table = HashMap::new();
    let mut e: u32 = 1;

    for i in 0..m {
        table.insert(e, i);
        e = ((e as u64 * g as u64) % mod_base as u64) as u32;
    }

    let factor = mod_exp(g as u64, (mod_base - m - 1) as u64, mod_base as u64);
    e = h;

    for i in 0..m {
        if let Some(&val) = table.get(&e) {
            return Some(i * m + val);
        }

        e = ((e as u64 * factor) % mod_base as u64) as u32;
    }

    None
}

#[inline]
fn mod_exp(base: u64, mut power: u64, mod_base: u64) -> u64 {
    let mut result = 1;
    let mut cur = base;

    while power > 0 {
        if power % 2 == 1 {
            result *= cur;
            result %= mod_base;
        }

        cur *= cur;
        cur %= mod_base;

        power /= 2;
    }

    result
}

#[derive(Default)]
pub struct Day25;

impl Solution for Day25 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let nums: Vec<_> = from_lines(input);

        let exponent: u64 = nums.into_iter().map(loop_count).product();
        let result = mod_exp(SUBJECT_NUMBER as u64, exponent, MOD_BASE as u64);

        result.to_string()
    }

    fn part2(&mut self, _input: &mut dyn Read) -> String {
        "Part 2 is free!".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../samples/25.txt");

    #[test]
    fn test_loop_count() {
        assert_eq!(8, loop_count(5764801));
        assert_eq!(11, loop_count(17807724));
    }

    #[test]
    fn sample_part1() {
        test_implementation!(Day25, 1, SAMPLE, 14897079);
    }
}
