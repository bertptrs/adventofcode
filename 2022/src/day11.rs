use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::digit1;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::sequence::tuple;
use nom::IResult;

use crate::common::parse_input;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Mul(u64),
    Add(u64),
    Square,
}

impl Operation {
    fn transform(self, worry: u64) -> u64 {
        match self {
            Operation::Mul(val) => worry * val,
            Operation::Add(val) => worry + val,
            Operation::Square => worry * worry,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_mod: u64,
    targets: [usize; 2],
    inspected: usize,
}

impl Monkey {
    fn handle_items(&mut self, drains: &mut [Vec<u64>; 2]) {
        self.inspected += self.items.len();

        for item in self.items.drain(..) {
            let mut new_val = self.operation.transform(item);
            // Miraculously get less worried
            new_val /= 3;

            drains[(new_val % self.test_mod == 0) as usize].push(new_val);
        }
    }

    fn handle_items2(&mut self, drains: &mut [Vec<u64>], mod_base: u64) {
        self.inspected += self.items.len();

        for item in self.items.drain(..) {
            let mut new_val = self.operation.transform(item);
            // Modular arithmetic is a good way to get less worried
            new_val %= mod_base;

            drains[(new_val % self.test_mod == 0) as usize].push(new_val);
        }
    }
}

fn parse_operation(input: &[u8]) -> IResult<&[u8], Operation> {
    preceded(
        tag("new = old "),
        alt((
            map_res(
                separated_pair(take(1usize), tag(" "), nom::character::complete::u64),
                |(op, val): (&[u8], u64)| match op[0] {
                    b'*' => Ok(Operation::Mul(val)),
                    b'+' => Ok(Operation::Add(val)),
                    _ => Err(anyhow::anyhow!("Invalid operation {op:?}")),
                },
            ),
            map(tag("* old"), |_| Operation::Square),
        )),
    )(input)
}

fn parse_monkey(input: &[u8]) -> IResult<&[u8], Monkey> {
    use nom::character::complete::u64;

    map(
        preceded(
            // Skip the useless header line
            tuple((tag("Monkey "), digit1, tag(":\n"))),
            // Parse the actual interesting bits of the monkey
            tuple((
                // Parse the starting items
                delimited(
                    tag("  Starting items: "),
                    separated_list1(tag(", "), u64),
                    newline,
                ),
                // Parse operation
                delimited(tag("  Operation: "), parse_operation, newline),
                // Parse the test
                delimited(tag("  Test: divisible by "), u64, newline),
                // Parse both cases
                delimited(tag("    If true: throw to monkey "), u64, newline),
                delimited(tag("    If false: throw to monkey "), u64, newline),
            )),
        ),
        |(items, operation, test_mod, if_true, if_false)| Monkey {
            items,
            operation,
            test_mod,
            targets: [if_false as usize, if_true as usize],
            inspected: 0,
        },
    )(input)
}

pub fn part1(input: &[u8]) -> Result<String> {
    let mut monkeys = parse_input(input, separated_list0(newline, parse_monkey))?;
    let mut drains = [Vec::new(), Vec::new()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            monkeys[i].handle_items(&mut drains);

            for (j, drain) in drains.iter_mut().enumerate() {
                let target = monkeys[i].targets[j];
                monkeys[target].items.append(drain);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));

    let result: usize = monkeys[0].inspected * monkeys[1].inspected;

    Ok(result.to_string())
}

pub fn part2(input: &[u8]) -> Result<String> {
    let mut monkeys = parse_input(input, separated_list0(newline, parse_monkey))?;
    let mut drains = [Vec::new(), Vec::new()];

    let mod_base: u64 = monkeys.iter().map(|m| m.test_mod).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            monkeys[i].handle_items2(&mut drains, mod_base);

            for (j, drain) in drains.iter_mut().enumerate() {
                let target = monkeys[i].targets[j];
                monkeys[target].items.append(drain);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));

    let result: usize = monkeys[0].inspected * monkeys[1].inspected;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/11.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "10605");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "2713310158");
    }
}
