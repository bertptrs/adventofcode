use std::io::Read;

use itertools::Itertools;

use crate::common::ordered;

fn compute_groups<'a>(it: impl IntoIterator<Item = &'a usize>) -> Vec<(usize, usize)> {
    let mut it = it.into_iter().copied().dedup_with_count();

    let (mut population, mut last_pos) = it.next().unwrap();
    let mut last_cost = 0;

    let mut costs = vec![(last_pos, 0)];

    for (number, pos) in it {
        let (first, last) = ordered(last_pos, pos);
        let new_cost = last_cost + population * (last - first);

        costs.push((pos, new_cost));

        population += number;
        last_pos = pos;
        last_cost = new_cost;
    }

    costs
}

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

pub fn part1(input: &mut dyn Read) -> String {
    let crabs = read_input(input);

    let forward_costs = compute_groups(&crabs);
    let backwards_costs = compute_groups(crabs.iter().rev());

    backwards_costs
        .into_iter()
        .rev()
        .zip(forward_costs)
        .map(|((pos_b, cost_b), (pos_f, cost_f))| {
            debug_assert_eq!(pos_f, pos_b);

            cost_f + cost_b
        })
        .min()
        .unwrap()
        .to_string()
}

pub fn sum_until(end: usize) -> usize {
    (end * (1 + end)) / 2
}

fn cost_at(pos: usize, groups: &[(usize, usize)]) -> usize {
    groups
        .iter()
        .map(|&(number, new_pos)| {
            let (first, last) = ordered(pos, new_pos);

            number * sum_until(last - first)
        })
        .sum()
}

pub fn part2(input: &mut dyn Read) -> String {
    let crabs = read_input(input);
    let groups: Vec<_> = crabs.into_iter().dedup_with_count().collect();

    let min = groups.first().unwrap().1;
    let max = groups.last().unwrap().1;

    // Brute force approach, better version later
    (min..=max)
        .map(|pos| cost_at(pos, &groups))
        .min()
        .unwrap()
        .to_string()
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
