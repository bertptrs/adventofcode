use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::IResult;

use crate::common::parse_input;
use crate::common::Direction;

struct Dig {
    dir: Direction,
    amount: u64,
}

fn parse_instructions(i: &[u8]) -> IResult<&[u8], Vec<Dig>> {
    many1(terminated(
        map(
            pair(
                terminated(
                    map_res(take(1usize), |c: &[u8]| {
                        Ok(match c[0] {
                            b'L' => Direction::Left,
                            b'U' => Direction::Up,
                            b'R' => Direction::Right,
                            b'D' => Direction::Down,
                            e => return Err(format!("Invalid digging direction {e}")),
                        })
                    }),
                    tag(" "),
                ),
                terminated(nom::character::complete::u64, tag(" ")),
            ),
            |(dir, amount)| Dig { dir, amount },
        ),
        take(10usize),
    ))(i)
}

fn compute_points(instructions: &[Dig]) -> Vec<(i64, i64)> {
    let mut result = Vec::with_capacity(instructions.len() + 1);
    result.push((0, 0));

    let mut x = 0;
    let mut y = 0;

    for &Dig { dir, amount } in instructions {
        match dir {
            Direction::Up => y -= amount as i64,
            Direction::Left => x -= amount as i64,
            Direction::Down => y += amount as i64,
            Direction::Right => x += amount as i64,
        }

        result.push((x, y));
    }

    debug_assert_eq!(result.first(), result.last());

    result
}

fn shoelace(points: &[(i64, i64)]) -> i64 {
    points
        .windows(2)
        .map(|w| w[0].0 * w[1].1 - w[0].1 * w[1].0)
        .sum::<i64>()
        / 2
}

fn solve(digs: &[Dig]) -> anyhow::Result<String> {
    let points = compute_points(digs);

    let area = shoelace(&points);
    // Assumption: we don't cross over ourselves
    let perimeter = digs.iter().map(|dig| dig.amount as i64).sum::<i64>();

    let total = area + perimeter / 2 + 1;

    Ok(total.to_string())
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let digs = parse_input(input, parse_instructions)?;

    solve(&digs)
}

pub fn part2(_input: &[u8]) -> anyhow::Result<String> {
    anyhow::bail!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/18.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("62", part1(SAMPLE).unwrap());
    }
}
