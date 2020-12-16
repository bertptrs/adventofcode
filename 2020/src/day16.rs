use std::collections::HashMap;
use std::collections::HashSet;
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

    while let Some(line) = lines.next() {
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

        // O(n) operation but it's fine
        let my_ticket = tickets.remove(0);

        // Filter out invalid tickets
        tickets.retain(|t| {
            t.iter()
                .all(|v| rules.values().flatten().any(|r| r.contains(v)))
        });

        let all_fields: HashSet<_> = rules.keys().cloned().collect();

        let mut pos_can_be = vec![all_fields; my_ticket.len()];

        let mut fixed_fields = HashMap::new();

        for ticket in &tickets {
            for (field, ranges) in &rules {
                for (pos, value) in ticket.iter().enumerate() {
                    if pos_can_be[pos].len() <= 1 {
                        continue;
                    }

                    if !ranges.iter().any(|r| r.contains(value)) {
                        pos_can_be[pos].remove(field);
                    }
                }
            }
        }

        let mut can_fit: HashMap<&str, u64> = rules.keys().map(|k| (k.as_str(), 0)).collect();

        while fixed_fields.len() != rules.len() {
            // Fix fields that are the only option for a certain spot
            for pos in 0..pos_can_be.len() {
                if pos_can_be[pos].len() == 1 {
                    let field = pos_can_be[pos]
                        .drain()
                        .next()
                        .expect("Safe because if statement");

                    assert!(!fixed_fields.contains_key(&field));

                    for can_be in pos_can_be.iter_mut() {
                        can_be.remove(&field);
                    }

                    fixed_fields.insert(field, pos);
                }
            }

            // Reset can_fit
            can_fit.values_mut().for_each(|v| *v = 0);

            for (pos, candiates) in pos_can_be.iter().enumerate() {
                for candidate in candiates {
                    *can_fit.get_mut(candidate.as_str()).unwrap() |= 1 << pos;
                }
            }

            for (&field, &pos) in can_fit.iter().filter(|(_, v)| v.count_ones() == 1) {
                let pos = pos.trailing_zeros() as usize;
                assert!(!fixed_fields.contains_key(field));

                pos_can_be[pos].clear();

                fixed_fields.insert(field.to_owned(), pos);
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
        test_implementation!(Day16, 1, SAMPLE, 71);
    }
}
