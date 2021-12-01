use std::io::Read;

use crate::common::LineParser;

fn read_input(input: &mut dyn Read) -> Vec<u32> {
    LineParser::new(input).collect()
}

pub fn part1(input: &mut dyn Read) -> String {
    let numbers = read_input(input);

    numbers
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count()
        .to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let numbers = read_input(input);

    numbers
        .windows(4)
        .filter(|w| w[3] > w[0])
        .count()
        .to_string()
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
