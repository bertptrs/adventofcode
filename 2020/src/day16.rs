use std::collections::HashMap;
use std::io::Read;
use std::ops::RangeInclusive;

use crate::common::Lines;
use crate::Solution;

fn split_nums<'a>(s: &'a str) -> impl Iterator<Item = u32> + 'a {
    s.split(|c: char| !c.is_ascii_digit()).filter_map(|s| {
        if s.is_empty() {
            None
        } else {
            Some(s.parse().unwrap())
        }
    })
}

// Clippy allow here because this type is used exactly once
#[allow(clippy::type_complexity)]
fn read_input(input: &mut dyn Read) -> (HashMap<String, Vec<RangeInclusive<u32>>>, Vec<Vec<u32>>) {
    let mut lines = Lines::new(input).filter(|s| !s.is_empty());

    let mut rules = HashMap::new();
    let mut tickets = Vec::new();

    while let Some(line) = lines.next() {
        if line.as_str() == "your ticket:" {
            break;
        }

        let colon = line.find(':').unwrap();

        let nums: Vec<_> = split_nums(&line[colon..]).collect();

        let ranges: Vec<_> = nums.chunks_exact(2).map(|s| s[0]..=s[1]).collect();

        rules.insert(line[..colon].to_owned(), ranges);
    }

    for line in lines {
        if line.as_str() == "nearby tickets:" {
            continue;
        }

        tickets.push(split_nums(&line).collect());
    }

    (rules, tickets)
}

#[derive(Default)]
pub struct Day16;

impl Solution for Day16 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let (rules, tickets) = read_input(input);

        let mut rate = 0;

        for ticket in &tickets[1..] {
            for value in ticket {
                if rules.values().flatten().all(|r| !r.contains(value)) {
                    rate += value;
                }
            }
        }

        rate.to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let (rules, tickets) = read_input(input);
        let mut tickets = tickets;

        // Filter out invalid tickets
        tickets.retain(|t| {
            t.iter()
                .all(|v| rules.values().flatten().any(|r| r.contains(v)))
        });

        let fields: Vec<_> = rules.keys().collect();
        let ranges: Vec<_> = rules.values().collect();

        let my_ticket = &tickets[0];

        let mut pos_can_be = vec![(1u64 << fields.len()) - 1; my_ticket.len()];

        let mut fixed_fields = HashMap::new();

        for ticket in &tickets {
            for (r, &ranges) in ranges.iter().enumerate() {
                for (pos, value) in ticket.iter().enumerate() {
                    if pos_can_be[pos].count_ones() <= 1 {
                        continue;
                    }

                    if !ranges.iter().any(|r| r.contains(value)) {
                        pos_can_be[pos] &= !(1 << r);
                    }
                }
            }
        }

        let mut can_fit = vec![0u64; fields.len()];

        while fixed_fields.len() != rules.len() {
            // Fix fields that are the only option for a certain spot
            for pos in 0..pos_can_be.len() {
                if pos_can_be[pos].count_ones() == 1 {
                    let field_num = pos_can_be[pos].trailing_zeros();
                    let field = fields[field_num as usize];

                    assert!(!fixed_fields.contains_key(&field));

                    for can_be in pos_can_be.iter_mut() {
                        *can_be &= !(1 << field_num);
                    }

                    fixed_fields.insert(field, pos);
                }
            }

            for (r, candidates) in can_fit.iter_mut().enumerate() {
                *candidates = 0;
                let flag = 1 << r;

                for (pos, valid) in pos_can_be.iter().enumerate() {
                    if (valid & flag) != 0 {
                        *candidates |= 1 << pos;
                    }
                }
            }

            for (field, &pos) in can_fit
                .iter()
                .enumerate()
                .filter(|(_, v)| v.count_ones() == 1)
            {
                let pos = pos.trailing_zeros() as usize;
                let field = fields[field];
                assert!(!fixed_fields.contains_key(field));

                pos_can_be[pos] = 0;

                fixed_fields.insert(field, pos);
            }
        }

        fixed_fields
            .iter()
            .filter(|(k, _)| k.starts_with("departure"))
            .map(|(_, &v)| my_ticket[v])
            .fold(1u64, |a, b| a * (b as u64))
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../samples/16.txt");

    #[test]
    fn sample_part1() {
        test_implementation(Day16, 1, SAMPLE, 71);
    }
}
