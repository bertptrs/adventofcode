use std::collections::HashSet;
use std::ops::Add;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Sub;

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

fn parse_moves<'a>(input: &'a [u8]) -> IResult<&'a [u8], Vec<(Direction, u32)>> {
    many0(terminated(
        separated_pair(
            map_res(take(1usize), |bs: &[u8]| Direction::try_from(bs[0])),
            tag(" "),
            nom::character::complete::u32,
        ),
        newline,
    ))(input)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Vec2(pub [i32; 2]);

impl Add<Self> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self([self[0] + rhs[0], self[1] + rhs[1]])
    }
}

impl Sub<Self> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self([self[0] - rhs[0], self[1] - rhs[1]])
    }
}

impl Index<usize> for Vec2 {
    type Output = i32;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

fn part_generic<const N: usize>(input: &[u8]) -> Result<String> {
    let moves = parse_input(input, parse_moves)?;

    let mut head_pos = Vec2([0, 0]);
    let mut tails = [head_pos; N];

    let mut visited = HashSet::new();
    visited.insert(head_pos);

    for (direction, steps) in moves {
        let step = direction.vec_for();

        for _ in 0..steps {
            head_pos = head_pos + step;

            let mut ref_pos = head_pos;

            for (i, tail_pos) in tails.iter_mut().enumerate() {
                let delta = ref_pos - *tail_pos;

                if delta[0].abs() <= 1 && delta[1].abs() <= 1 {
                    break;
                }

                let step = Vec2([delta[0].signum(), delta[1].signum()]);

                *tail_pos = *tail_pos + step;

                if i == N - 1 {
                    visited.insert(*tail_pos);
                }
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
