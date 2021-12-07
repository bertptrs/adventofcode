use std::io::Read;

use itertools::Itertools;

use crate::common::ordered;

fn compute_cumulative<'a, I, It>(it: I) -> Vec<usize>
where
    I: IntoIterator<IntoIter = It>,
    It: Iterator<Item = &'a (usize, usize)> + ExactSizeIterator,
{
    let mut it = it.into_iter().copied();
    let mut costs = Vec::with_capacity(it.len());
    costs.push(0);

    let (mut population, mut last_pos) = it.next().unwrap();
    let mut last_cost = 0;

    for (number, pos) in it {
        let (first, last) = ordered(last_pos, pos);
        let new_cost = last_cost + population * (last - first);

        population += number;
        last_pos = pos;
        last_cost = new_cost;

        costs.push(new_cost);
    }

    costs
}

fn read_input(input: &mut dyn Read) -> Vec<(usize, usize)> {
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();

    let mut crabs: Vec<usize> = buf
        .trim_end()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    crabs.sort_unstable();

    crabs.into_iter().dedup_with_count().collect()
}

pub fn part1(input: &mut dyn Read) -> String {
    let crabs = read_input(input);

    let forwards_costs = compute_cumulative(&crabs);
    let backwards_costs = compute_cumulative(crabs.iter().rev());

    // Note: the optimal position can be proven to be one of the original positions.
    ternary_search(0, forwards_costs.len() - 1, |idx| {
        forwards_costs[idx] + backwards_costs[backwards_costs.len() - 1 - idx]
    })
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
    let groups = read_input(input);

    let min = groups.first().unwrap().1;
    let max = groups.last().unwrap().1;

    ternary_search(min, max, |pos| cost_at(pos, &groups)).to_string()
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
