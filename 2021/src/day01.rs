use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

fn read_input(input: &mut dyn Read) -> Vec<u32> {
    let reader = BufReader::new(input);

    // TODO: optimize allocations out
    reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
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

    let mut last = None;

    numbers
        .windows(3)
        .filter(|w| {
            let sum: u32 = w.iter().sum();

            let prev = last.replace(sum);

            match prev {
                Some(n) if n < sum => true,
                _ => false,
            }
        })
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
