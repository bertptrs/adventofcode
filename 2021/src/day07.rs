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

pub fn part1(input: &mut dyn Read) -> String {
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();

    let mut crabs: Vec<usize> = buf
        .trim_end()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    crabs.sort_unstable();

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

pub fn part2(_input: &mut dyn Read) -> String {
    todo!()
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
}
