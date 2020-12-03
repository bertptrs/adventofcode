use std::io::Read;

use crate::common::read_char_grid;
use crate::Solution;

#[derive(Default)]
pub struct Day03;

fn walk(right: usize, down: usize, forrest: &[Vec<u8>]) -> usize {
    let mut trees = 0;
    let mut x = 0;

    for line in forrest.iter().step_by(down) {
        if line[x] == b'#' {
            trees += 1;
        }

        x = (x + right) % line.len();
    }

    trees
}

impl Solution for Day03 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let forrest = read_char_grid(input);

        walk(3, 1, &forrest).to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let forrest = read_char_grid(input);
        let mut product = 1;

        for &(right, down) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
            product *= walk(right, down, &forrest);
        }

        product.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../samples/03.txt");

    #[test]
    fn sample_part1() {
        test_implementation!(Day03, 1, SAMPLE, 7);
    }

    #[test]
    fn sample_part2() {
        test_implementation!(Day03, 2, SAMPLE, 336);
    }
}
