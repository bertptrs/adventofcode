use std::io::Read;

use crate::common::from_lines;
use crate::Solution;

#[derive(Default)]
pub struct Day01;

fn fuel_required(weight: u32) -> u32 {
    (weight / 3).saturating_sub(2)
}

fn recursive_fuel(mut weight: u32) -> u32 {
    let mut required = 0;

    while weight > 0 {
        weight = fuel_required(weight);
        required += weight
    }

    required
}

impl Solution for Day01 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let weights: Vec<u32> = from_lines(input);

        let fuel_required: u32 = weights.into_iter().map(fuel_required).sum();

        fuel_required.to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let weights: Vec<u32> = from_lines(input);

        let fuel_required: u32 = weights.into_iter().map(recursive_fuel).sum();

        fuel_required.to_string()
    }
}
