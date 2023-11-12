use ahash::AHashMap;
use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::alpha1;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::multi::fold_many1;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use crate::common::parse_input;

type Input<'a> = AHashMap<&'a [u8], Monkey<'a>>;

#[derive(Clone, Copy)]
enum Operation {
    Mul,
    Div,
    Add,
    Sub,
}

impl Operation {
    pub fn apply(self, first: i64, second: i64) -> i64 {
        match self {
            Operation::Mul => first * second,
            Operation::Div => first / second,
            Operation::Add => first + second,
            Operation::Sub => first - second,
        }
    }
}

impl TryFrom<u8> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'*' => Operation::Mul,
            b'/' => Operation::Div,
            b'+' => Operation::Add,
            b'-' => Operation::Sub,
            other => anyhow::bail!("Invalid operation: {other}"),
        })
    }
}

enum Monkey<'a> {
    Operation(&'a [u8], &'a [u8], Operation),
    Literal(i64),
}

fn parse_monkeys(input: &[u8]) -> IResult<&[u8], Input> {
    let parse_monkey = terminated(
        tuple((
            alpha1,
            preceded(
                tag(": "),
                alt((
                    map(nom::character::complete::i64, Monkey::Literal),
                    map(
                        tuple((
                            terminated(alpha1, tag(" ")),
                            map_res(take(1usize), |v: &[u8]| Operation::try_from(v[0])),
                            preceded(tag(" "), alpha1),
                        )),
                        |(first, operation, second)| Monkey::Operation(first, second, operation),
                    ),
                )),
            ),
        )),
        newline,
    );

    fold_many1(parse_monkey, AHashMap::new, |mut map, (name, monkey)| {
        map.insert(name, monkey);
        map
    })(input)
}

fn evaluate(monkeys: &Input, start: &[u8]) -> i64 {
    match &monkeys[start] {
        Monkey::Operation(first, second, op) => {
            let first = evaluate(monkeys, first);
            let second = evaluate(monkeys, second);

            op.apply(first, second)
        }
        Monkey::Literal(value) => *value,
    }
}

enum IncompleteSide {
    Left,
    Right,
}

fn evaluate2(
    monkeys: &Input,
    start: &[u8],
) -> std::result::Result<i64, Vec<(i64, IncompleteSide, Operation)>> {
    if start == b"humn" {
        return Err(Vec::new());
    }

    match &monkeys[start] {
        Monkey::Operation(first, second, op) => {
            match (evaluate2(monkeys, first), evaluate2(monkeys, second)) {
                (Ok(first), Ok(second)) => Ok(op.apply(first, second)),
                (Ok(first), Err(mut incomplete)) => {
                    incomplete.push((first, IncompleteSide::Right, *op));
                    Err(incomplete)
                }
                (Err(mut incomplete), Ok(second)) => {
                    incomplete.push((second, IncompleteSide::Left, *op));
                    Err(incomplete)
                }
                (Err(_), Err(_)) => unreachable!("Should not happen on fair input"),
            }
        }
        Monkey::Literal(val) => Ok(*val),
    }
}

pub fn part1(input: &[u8]) -> Result<String> {
    let monkeys = parse_input(input, parse_monkeys)?;

    Ok(evaluate(&monkeys, b"root").to_string())
}

pub fn part2(input: &[u8]) -> Result<String> {
    let monkeys = parse_input(input, parse_monkeys)?;

    let Monkey::Operation(first, second, _) = &monkeys[&b"root"[..]] else {
        anyhow::bail!("root is a literal somehow")
    };

    let result = match (evaluate2(&monkeys, first), evaluate2(&monkeys, second)) {
        (Ok(_), Ok(_)) => anyhow::bail!("both arms succeeded"),
        (Ok(goal), Err(incomplete)) | (Err(incomplete), Ok(goal)) => incomplete
            .into_iter()
            .rev()
            .fold(goal, |next, (complete, arm, op)| match (op, arm) {
                // Multiplication and addition are commutative so the arm doesn't matter
                (Operation::Mul, _) => {
                    // This was a very useful sanity check
                    debug_assert_eq!(next % complete, 0);
                    next / complete
                }

                (Operation::Add, _) => next - complete,

                // The other operations need some tweaking. x: unknown quantity. c: known quantity. n: current value
                // x - c = n -> x = n + c
                (Operation::Sub, IncompleteSide::Left) => next + complete,
                // c - x = n -> c = n + x -> c - n = x
                (Operation::Sub, IncompleteSide::Right) => complete - next,

                // Similarly for division, if we miss the left arm we can undo the division and multiply instead
                // x / c = n -> x = n * c
                (Operation::Div, IncompleteSide::Left) => next * complete,
                // c / x = n -> c = n * x -> c / n = x
                (Operation::Div, IncompleteSide::Right) => complete / next,
            }),
        (Err(_), Err(_)) => anyhow::bail!("both arms failed"),
    };

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/21.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "152");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "301");
    }
}
