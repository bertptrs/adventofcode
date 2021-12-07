use std::io::Read;

use itertools::Itertools;

use crate::common::ordered;

fn read_input(input: &mut dyn Read) -> Vec<usize> {
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();

    let mut crabs: Vec<usize> = buf
        .trim_end()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    crabs.sort_unstable();

    crabs
}

fn cost_at(pos: usize, crabs: &[usize]) -> usize {
    crabs
        .iter()
        .map(|&crab_pos| {
            if crab_pos > pos {
                crab_pos - pos
            } else {
                pos - crab_pos
            }
        })
        .sum()
}

pub fn part1(input: &mut dyn Read) -> String {
    let crabs = read_input(input);

    let median = crabs[crabs.len() / 2 + (crabs.len() % 2)];

    cost_at(median, &crabs).to_string()
}

pub fn sum_until(end: usize) -> usize {
    (end * (1 + end)) / 2
}

fn cost_at2(pos: usize, groups: &[(usize, usize)]) -> usize {
    groups
        .iter()
        .map(|&(number, new_pos)| {
            let (first, last) = ordered(pos, new_pos);

            number * sum_until(last - first)
        })
        .sum()
}

fn ternary_search(mut min: usize, mut max: usize, callback: impl Fn(usize) -> usize) -> usize {
    while max - min > 6 {
        let mid1 = min + (max - min) / 3;
        let mid2 = max - (max - min) / 3;

        let cost1 = callback(mid1);
        let cost2 = callback(mid2);

        if cost1 < cost2 {
            max = mid2 - 1
        } else {
            min = mid1 + 1
        }
    }

    // Ternary search isn't effective at such small intervals so we iterate the remaining part
    (min..=max).map(callback).min().unwrap()
}

pub fn part2(input: &mut dyn Read) -> String {
    let groups: Vec<_> = read_input(input).into_iter().dedup_with_count().collect();

    let min = groups.first().unwrap().1;
    let max = groups.last().unwrap().1;

    ternary_search(min, max, |pos| cost_at2(pos, &groups)).to_string()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = &*b"16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 37);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 168);
    }

    #[test]
    fn test_maths() {
        assert_eq!(sum_until(1), 1);
        assert_eq!(sum_until(2), 3);
        assert_eq!(sum_until(3), 6);
        assert_eq!(sum_until(4), 10);
    }
}
