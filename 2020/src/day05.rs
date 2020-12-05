use std::io::Read;

use crate::common::Lines;
use crate::Solution;

fn seat_id(boarding_pass: &str) -> u32 {
    boarding_pass
        .chars()
        .fold(0, |b, c| (b << 1) | ((c == 'B' || c == 'R') as u32))
}

fn seat_iter<'a>(input: &'a mut dyn Read) -> impl Iterator<Item = u32> + 'a {
    Lines::new(input).map(|s| seat_id(&s))
}

#[derive(Default)]
pub struct Day05;

impl Solution for Day05 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        seat_iter(input).max().unwrap().to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let mut taken = [false; 0x400];

        for seat_id in seat_iter(input) {
            taken[seat_id as usize] = true;
        }

        let pattern = [true, false, true];

        for (i, window) in taken.windows(3).enumerate() {
            if window == pattern {
                return (i + 1).to_string();
            }
        }

        panic!("No seat found!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_id() {
        assert_eq!(567, seat_id("BFFFBBFRRR"));
        assert_eq!(119, seat_id("FFFBBBFRRR"));
        assert_eq!(820, seat_id("BBFFBBFRLL"));
    }
}
