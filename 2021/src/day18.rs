use std::fmt::Debug;
use std::io::Read;
use std::mem::replace;

use nom::branch::alt;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;

use crate::common::read_input;

#[derive(Clone, Eq, PartialEq)]
enum TurtleNumber {
    Literal(u8),
    Pair(Box<(TurtleNumber, TurtleNumber)>),
}

impl TurtleNumber {
    pub fn add(self, other: Self) -> Self {
        let mut new = TurtleNumber::Pair(Box::new((self, other)));
        new.reduce();

        new
    }

    pub fn magnitude(&self) -> u32 {
        match self {
            TurtleNumber::Literal(num) => *num as u32,
            TurtleNumber::Pair(pair) => 3 * pair.0.magnitude() + 2 * pair.1.magnitude(),
        }
    }

    fn reduce(&mut self) {
        loop {
            while self.try_explode(0).is_some() {}

            if self.split() {
                continue;
            } else {
                break;
            }
        }
    }

    fn leftmost_add(&mut self, value: u8) {
        match self {
            TurtleNumber::Literal(num) => *num += value,
            TurtleNumber::Pair(pair) => pair.0.leftmost_add(value),
        }
    }

    fn rightmost_add(&mut self, value: u8) {
        match self {
            TurtleNumber::Literal(num) => *num += value,
            TurtleNumber::Pair(pair) => pair.1.rightmost_add(value),
        }
    }

    fn try_explode(&mut self, depth: usize) -> Option<(Option<u8>, Option<u8>)> {
        let pair = match self {
            TurtleNumber::Literal(_) => return None,
            TurtleNumber::Pair(pair) => pair,
        };

        if depth == 4 {
            let original = replace(self, TurtleNumber::Literal(0));
            let pair = match original {
                TurtleNumber::Pair(pair) => *pair,
                _ => unreachable!("Already checked for pair above"),
            };

            if let (TurtleNumber::Literal(first), TurtleNumber::Literal(second)) = pair {
                Some((Some(first), Some(second)))
            } else {
                panic!("Too deeply nested turtle number: {:?}", pair);
            }
        } else {
            match pair.0.try_explode(depth + 1) {
                Some((left, Some(right))) => {
                    pair.1.leftmost_add(right);
                    Some((left, None))
                }
                Some((left, None)) => Some((left, None)),
                None => match pair.1.try_explode(depth + 1) {
                    Some((Some(left), right)) => {
                        pair.0.rightmost_add(left);
                        Some((None, right))
                    }
                    other => other,
                },
            }
        }
    }

    pub fn split(&mut self) -> bool {
        match self {
            TurtleNumber::Literal(num) if *num >= 10 => {
                let half = *num / 2;
                let other = *num - half;
                *self = TurtleNumber::from((half, other));
                true
            }
            TurtleNumber::Pair(pair) => pair.0.split() || pair.1.split(),
            _ => false,
        }
    }
}

impl Debug for TurtleNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(num) => write!(f, "{}", num),
            Self::Pair(pair) => write!(f, "[{:?},{:?}]", pair.0, pair.1),
        }
    }
}

// Helper traits to easily convert tuples to turtle numbers
impl From<u8> for TurtleNumber {
    fn from(num: u8) -> Self {
        TurtleNumber::Literal(num)
    }
}

impl<T, U> From<(T, U)> for TurtleNumber
where
    T: Into<TurtleNumber>,
    U: Into<TurtleNumber>,
{
    fn from((left, right): (T, U)) -> Self {
        TurtleNumber::Pair(Box::new((left.into(), right.into())))
    }
}

fn parse_turtle(input: &[u8]) -> IResult<&[u8], TurtleNumber> {
    use nom::character::complete::char;
    use nom::character::complete::u8;

    alt((
        map(u8, TurtleNumber::Literal),
        map(
            delimited(
                char('['),
                separated_pair(parse_turtle, char(','), parse_turtle),
                char(']'),
            ),
            |pair| TurtleNumber::Pair(Box::new(pair)),
        ),
    ))(input)
}

fn parse_input(input: &[u8]) -> IResult<&[u8], Vec<TurtleNumber>> {
    many0(terminated(parse_turtle, newline))(input)
}

pub fn part1(input: &mut dyn Read) -> String {
    let turtles = read_input(input, parse_input);
    turtles
        .into_iter()
        .reduce(|a, b| a.add(b))
        .map(|num| num.magnitude())
        .unwrap()
        .to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let turtles = read_input(input, parse_input);

    turtles
        .iter()
        .flat_map(|a| turtles.iter().map(|b| a.clone().add(b.clone()).magnitude()))
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/18.txt");

    #[test]
    fn test_magnitude() {
        let num: TurtleNumber = (
            (((8, 7), (7, 7)), ((8, 6), (7, 7))),
            (((0, 7), (6, 6)), (8, 7)),
        )
            .into();

        assert_eq!(num.magnitude(), 3488);
    }

    #[test]
    fn test_add() {
        let input = TurtleNumber::from(((((4, 3), 4), 4), (7, ((8, 4), 9))));
        let result = input.add((1, 1).into());

        let expected = TurtleNumber::from(((((0, 7), 4), ((7, 8), (6, 0))), (8, 1)));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_explode() {
        let mut input1 = TurtleNumber::from((((((9, 8), 1), 2), 3), 4));
        let output1 = TurtleNumber::from(((((0, 9), 2), 3), 4));

        input1.reduce();
        assert_eq!(input1, output1);

        let mut input2 = TurtleNumber::from((7, (6, (5, (4, (3, 2))))));
        let output2 = TurtleNumber::from((7, (6, (5, (7, 0)))));

        input2.reduce();
        assert_eq!(input2, output2);

        let mut input3: TurtleNumber = TurtleNumber::from(((6, (5, (4, (3, 2)))), 1));
        let output3 = TurtleNumber::from(((6, (5, (7, 0))), 3));

        input3.reduce();
        assert_eq!(input3, output3);

        let mut input4 = TurtleNumber::from(((3, (2, (1, (7, 3)))), (6, (5, (4, (3, 2))))));
        let output4 = TurtleNumber::from(((3, (2, (8, 0))), (9, (5, (7, 0)))));

        input4.reduce();
        assert_eq!(input4, output4);
    }

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 4140);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 3993);
    }
}
