use std::io::Read;

use crate::common::LineParser;

fn part_generic(input: &mut dyn Read, window: usize) -> String {
    let numbers: Vec<u32> = LineParser::new(input).collect();

    numbers
        .windows(window)
        .filter(|w| w.last() > w.first())
        .count()
        .to_string()
}

pub fn part1(input: &mut dyn Read) -> String {
    part_generic(input, 2)
}

pub fn part2(input: &mut dyn Read) -> String {
    part_generic(input, 4)
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/01.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 7);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 5);
    }
}
