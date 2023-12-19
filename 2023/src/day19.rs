use std::ops::Index;
use std::ops::Range;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::bytes::complete::take_until1;
use nom::bytes::complete::take_while1;
use nom::combinator::map;
use nom::multi::fold_many1;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use crate::common::parse_input;

const RULES_LEN: usize = 26 * 26 * 26;

#[derive(Clone, Copy)]
struct Item([u16; 4]);

impl Item {
    fn rating_sum(self) -> u32 {
        self.0.into_iter().map(u32::from).sum()
    }

    fn passes(&self, rules: &[Rule]) -> bool {
        let mut pos = convert_name(b"in");

        'outer: loop {
            let rule = &rules[pos as usize];

            for &(condition, end) in &rule.checks {
                let is_valid = match condition {
                    Condition::Greater(c, val) => self[c] > val,
                    Condition::Less(c, val) => self[c] < val,
                };

                if is_valid {
                    match end {
                        RuleEnd::Reject => return false,
                        RuleEnd::Accept => return true,
                        RuleEnd::Next(new_pos) => {
                            pos = new_pos;
                            continue 'outer;
                        }
                    }
                }
            }

            match rule.end {
                RuleEnd::Reject => return false,
                RuleEnd::Accept => return true,
                RuleEnd::Next(new_pos) => {
                    pos = new_pos;
                    continue 'outer;
                }
            }
        }
    }
}

impl Index<u8> for Item {
    type Output = u16;

    fn index(&self, index: u8) -> &Self::Output {
        static FALLBACK: u16 = 0;

        match index {
            b'x' => &self.0[0],
            b'm' => &self.0[1],
            b'a' => &self.0[2],
            b's' => &self.0[3],
            other => {
                debug_assert!(false, "Invalid index: {}", other as char);
                &FALLBACK
            }
        }
    }
}

fn parse_item(i: &[u8]) -> IResult<&[u8], Item> {
    use nom::character::complete::u16;
    map(
        tuple((
            preceded(tag("{x="), u16),
            preceded(tag(",m="), u16),
            preceded(tag(",a="), u16),
            delimited(tag(",s="), u16, tag("}\n")),
        )),
        |(x, m, a, s)| Item([x, m, a, s]),
    )(i)
}

#[derive(Default, Copy, Clone)]
enum RuleEnd {
    #[default]
    Reject,
    Accept,
    Next(u16),
}

#[derive(Clone, Copy)]
enum Condition {
    Greater(u8, u16),
    Less(u8, u16),
}

#[derive(Default, Clone)]
struct Rule {
    checks: Vec<(Condition, RuleEnd)>,
    end: RuleEnd,
}

// Base 26 at it again, but now in lowercase
fn convert_name(name: &[u8]) -> u16 {
    name.iter()
        .fold(0, |cur, &c| cur * 26 + u16::from(c - b'a'))
}

fn parse_rule(i: &[u8]) -> IResult<&[u8], (u16, Rule)> {
    fn convert_end(i: &[u8]) -> RuleEnd {
        match i {
            b"A" => RuleEnd::Accept,
            b"R" => RuleEnd::Reject,
            other => RuleEnd::Next(convert_name(other)),
        }
    }

    let (i, name) = map(take_while1(|c: u8| c.is_ascii_alphabetic()), convert_name)(i)?;
    let (i, _) = tag("{")(i)?;
    let (i, checks) = many1(terminated(
        map(
            tuple((
                take::<_, &[u8], _>(1usize),
                alt((tag("<"), tag(">"))),
                nom::character::complete::u16,
                preceded(tag(":"), take_until1(",")),
            )),
            |(index, cmp, val, dest)| {
                let condition = if cmp[0] == b'<' {
                    Condition::Less(index[0], val)
                } else {
                    debug_assert_eq!(cmp[0], b'>');
                    Condition::Greater(index[0], val)
                };

                (condition, convert_end(dest))
            },
        ),
        tag(","),
    ))(i)?;

    let (i, end) = map(take_until1("}\n"), convert_end)(i)?;
    Ok((&i[2..], (name, Rule { checks, end })))
}

fn parse_text(i: &[u8]) -> IResult<&[u8], (Box<[Rule]>, Vec<Item>)> {
    separated_pair(
        fold_many1(
            parse_rule,
            || vec![Rule::default(); RULES_LEN].into_boxed_slice(),
            |mut rules, (index, rule)| {
                rules[index as usize] = rule;
                rules
            },
        ),
        tag("\n"),
        many1(parse_item),
    )(i)
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let (rules, items) = parse_input(input, parse_text)?;

    let passing = items
        .iter()
        .filter(|c| c.passes(&rules))
        .map(|c| c.rating_sum())
        .sum::<u32>();

    Ok(passing.to_string())
}

type ValidRange = Range<u16>;

#[derive(Clone)]
struct State {
    numbers: [ValidRange; 4],
    pos: u16,
}

pub fn part2(input: &[u8]) -> anyhow::Result<String> {
    let (rules, _) = parse_input(input, parse_text)?;
    let mut passing = 0u64;

    let mut todo = Vec::new();
    todo.push(State {
        numbers: [(); 4].map(|_| 1..4001),
        pos: convert_name(b"in"),
    });

    while let Some(State { mut numbers, pos }) = todo.pop() {
        let mut enqueue = |numbers: [ValidRange; 4], end| match end {
            RuleEnd::Reject => (),
            RuleEnd::Accept => {
                passing += numbers.iter().map(|r| r.len() as u64).product::<u64>();
            }
            RuleEnd::Next(pos) => todo.push(State { numbers, pos }),
        };

        let rule = &rules[pos as usize];

        'outer: for &(condition, end) in &rule.checks {
            let mut new_numbers = numbers.clone();

            let (old_range, new_range) = match condition {
                Condition::Less(b'x', _) | Condition::Greater(b'x', _) => {
                    (&mut numbers[0], &mut new_numbers[0])
                }
                Condition::Less(b'm', _) | Condition::Greater(b'm', _) => {
                    (&mut numbers[1], &mut new_numbers[1])
                }
                Condition::Less(b'a', _) | Condition::Greater(b'a', _) => {
                    (&mut numbers[2], &mut new_numbers[2])
                }
                Condition::Less(b's', _) | Condition::Greater(b's', _) => {
                    (&mut numbers[3], &mut new_numbers[3])
                }
                Condition::Less(other, _) | Condition::Greater(other, _) => {
                    anyhow::bail!("Invalid variable {}", other as char)
                }
            };

            match condition {
                Condition::Greater(_, value) => {
                    old_range.end = value + 1;
                    new_range.start = value + 1;
                }
                Condition::Less(_, value) => {
                    old_range.start = value;
                    new_range.end = value;
                }
            }

            if !Range::is_empty(new_range) {
                enqueue(new_numbers, end);
            }

            if Range::is_empty(old_range) {
                continue 'outer;
            }
        }
        enqueue(numbers, rule.end);
    }

    Ok(passing.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/19.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("19114", part1(SAMPLE).unwrap());
    }

    #[test]
    fn sample_part2() {
        assert_eq!("167409079868000", part2(SAMPLE).unwrap());
    }
}
