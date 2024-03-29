use ahash::AHashSet;
use anyhow::Result;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::newline;
use nom::combinator::map_res;
use nom::multi::many0;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;

use crate::common::parse_input;
use crate::common::Vec2;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn vec_for(self) -> Vec2 {
        Vec2(match self {
            Direction::Up => [0, -1],
            Direction::Left => [1, 0],
            Direction::Right => [-1, 0],
            Direction::Down => [0, 1],
        })
    }
}

impl TryFrom<u8> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'U' => Ok(Direction::Up),
            b'L' => Ok(Direction::Left),
            b'R' => Ok(Direction::Right),
            b'D' => Ok(Direction::Down),
            b => anyhow::bail!("Invalid direction '{b}'"),
        }
    }
}

fn parse_moves(input: &[u8]) -> IResult<&[u8], Vec<(Direction, u32)>> {
    many0(terminated(
        separated_pair(
            map_res(take(1usize), |bs: &[u8]| Direction::try_from(bs[0])),
            tag(" "),
            nom::character::complete::u32,
        ),
        newline,
    ))(input)
}

fn part_generic<const N: usize>(input: &[u8]) -> Result<String> {
    let moves = parse_input(input, parse_moves)?;

    let mut head_pos = Vec2([0, 0]);
    let mut tails = [head_pos; N];

    let mut visited = AHashSet::new();
    visited.insert(head_pos);

    for (direction, steps) in moves {
        let step = direction.vec_for();

        for _ in 0..steps {
            head_pos = head_pos + step;

            let mut ref_pos = head_pos;

            for tail_pos in &mut tails {
                let delta = ref_pos - *tail_pos;

                if delta[0].abs() <= 1 && delta[1].abs() <= 1 {
                    break;
                }

                let step = Vec2([delta[0].signum(), delta[1].signum()]);

                *tail_pos = *tail_pos + step;

                ref_pos = *tail_pos;
            }

            visited.insert(*tails.last().unwrap());
        }
    }

    Ok(visited.len().to_string())
}

pub fn part1(input: &[u8]) -> Result<String> {
    part_generic::<1>(input)
}

pub fn part2(input: &[u8]) -> Result<String> {
    part_generic::<9>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/09.txt");
    const SAMPLE_LARGE: &[u8] = include_bytes!("samples/09.large.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "13");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "1");
        assert_eq!(part2(SAMPLE_LARGE).unwrap(), "36");
    }
}
