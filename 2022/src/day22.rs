use anyhow::Context;
use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::combinator::value;
use nom::multi::many1;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;

use crate::common::parse_input;

#[derive(Clone, Copy, Debug)]
enum Step {
    Forward(u32),
    Left,
    Right,
}

#[derive(Clone, Copy)]
enum Direction {
    Up = 3,
    Down = 1,
    Left = 2,
    Right = 0,
}

type Map<'a> = Vec<&'a [u8]>;

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn parse_map(input: &[u8]) -> IResult<&[u8], (Map, Vec<Step>)> {
    separated_pair(
        map(take_until("\n\n"), |map: &[u8]| {
            map.split(|&b| b == b'\n').collect()
        }),
        tag("\n\n"),
        terminated(
            many1(alt((
                map(nom::character::complete::u32, Step::Forward),
                value(Step::Right, tag("R")),
                value(Step::Left, tag("L")),
            ))),
            newline,
        ),
    )(input)
}

pub fn part1(input: &[u8]) -> Result<String> {
    let (map, steps) = parse_input(input, parse_map)?;
    let mut dir = Direction::Right;
    let mut y = 0;
    let mut x = map[y]
        .iter()
        .position(|&b| b == b'.')
        .context("Could not find starting position")?;

    for step in steps {
        match step {
            Step::Forward(amount) => match dir {
                Direction::Up => {
                    for _ in 0..amount {
                        if y == 0 || map[y - 1].get(x).map(|&b| b == b' ').unwrap_or(true) {
                            let new_y = map
                                .iter()
                                .rposition(|line| {
                                    line.get(x)
                                        .map(|&b| b == b'.' || b == b'#')
                                        .unwrap_or(false)
                                })
                                .unwrap();
                            if map[new_y][x] == b'#' {
                                break;
                            } else {
                                y = new_y;
                            }
                        } else if map[y - 1][x] == b'#' {
                            break;
                        } else {
                            y -= 1;
                        }
                    }
                }
                Direction::Down => {
                    for _ in 0..amount {
                        if y + 1 >= map.len()
                            || map[y + 1].get(x).map(|&b| b == b' ').unwrap_or(true)
                        {
                            let new_y = map
                                .iter()
                                .position(|line| {
                                    line.get(x)
                                        .map(|&b| b == b'.' || b == b'#')
                                        .unwrap_or(false)
                                })
                                .unwrap();

                            if map[new_y][x] == b'#' {
                                break;
                            } else {
                                y = new_y;
                            }
                        } else if map[y + 1][x] == b'#' {
                            break;
                        } else {
                            y += 1;
                        }
                    }
                }
                Direction::Left => {
                    for _ in 0..amount {
                        if x == 0 || map[y][x - 1] == b' ' {
                            let new_x = map[y]
                                .iter()
                                .rposition(|&b| b == b'.' || b == b'#')
                                .unwrap();
                            if map[y][new_x] == b'.' {
                                x = new_x;
                            } else {
                                break;
                            }
                        } else if map[y][x - 1] == b'#' {
                            break;
                        } else {
                            x -= 1;
                        }
                    }
                }
                Direction::Right => {
                    for _ in 0..amount {
                        if x + 1 >= map[y].len() || map[y][x + 1] == b' ' {
                            let new_x =
                                map[y].iter().position(|&b| b == b'.' || b == b'#').unwrap();
                            if map[y][new_x] == b'.' {
                                x = new_x;
                            } else {
                                break;
                            }
                        } else if map[y][x + 1] == b'#' {
                            break;
                        } else {
                            x += 1;
                        }
                    }
                }
            },
            Step::Left => dir = dir.turn_left(),
            Step::Right => dir = dir.turn_right(),
        }
    }

    Ok((1000 * (y + 1) + 4 * (x + 1) + dir as usize).to_string())
}

pub fn part2(_input: &[u8]) -> Result<String> {
    anyhow::bail!("not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/22.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "6032");
    }
}
