use std::mem;

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

// This describes the transitions between the different squares.
//
// For every direction, write down which direction you end up going, in which square, and
// whether you should flip the axis.
//
// The squares are laid out as follows:
//
// #01
// #2#
// 34#
// 5##
//
// Entries are specified right, down, left, up.
#[allow(dead_code)]
const TRANSITIONS: [[(Direction, usize, bool); 4]; 6] = [
    // Square 0
    [
        (Direction::Right, 1, false),
        (Direction::Down, 2, false),
        (Direction::Right, 3, true),
        (Direction::Right, 5, false),
    ],
    // Square 1
    [
        (Direction::Left, 4, true),
        (Direction::Left, 2, false),
        (Direction::Left, 0, false),
        (Direction::Up, 5, false),
    ],
    // Square 2
    [
        (Direction::Up, 1, false),
        (Direction::Down, 4, false),
        (Direction::Down, 3, false),
        (Direction::Up, 0, false),
    ],
    // Square 3
    [
        (Direction::Right, 4, false),
        (Direction::Down, 5, false),
        (Direction::Right, 0, true),
        (Direction::Right, 2, false),
    ],
    // Square 4
    [
        (Direction::Left, 1, true),
        (Direction::Left, 5, false),
        (Direction::Left, 3, false),
        (Direction::Up, 2, false),
    ],
    // Square 5
    [
        (Direction::Up, 4, false),
        (Direction::Down, 1, false),
        (Direction::Down, 0, false),
        (Direction::Up, 3, false),
    ],
];

#[derive(Clone, Copy, Debug)]
enum Step {
    Forward(u32),
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
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

fn find_starting_x(top_row: &[u8]) -> Result<usize> {
    top_row
        .iter()
        .position(|&b| b == b'.')
        .context("Could not find starting position")
}

pub fn part1(input: &[u8]) -> Result<String> {
    let (map, steps) = parse_input(input, parse_map)?;
    let mut dir = Direction::Right;
    let mut y = 0;
    let mut x = find_starting_x(map[y])?;

    for step in steps {
        match step {
            Step::Forward(amount) => match dir {
                Direction::Up => {
                    for _ in 0..amount {
                        if y == 0 || map[y - 1].get(x).map_or(true, |&b| b == b' ') {
                            let new_y = map
                                .iter()
                                .rposition(|line| {
                                    line.get(x).map_or(false, |&b| b == b'.' || b == b'#')
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
                        if y + 1 >= map.len() || map[y + 1].get(x).map_or(true, |&b| b == b' ') {
                            let new_y = map
                                .iter()
                                .position(|line| {
                                    line.get(x).map_or(false, |&b| b == b'.' || b == b'#')
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

fn side_length_of(map: &[&[u8]]) -> usize {
    let taken_tiles = map
        .iter()
        .flat_map(|r| r.iter())
        .filter(|c| !c.is_ascii_whitespace())
        .count();

    // Future Bert: this needs to be an integer square root.
    ((taken_tiles / 6) as f64).sqrt() as usize
}

fn break_squares<'a>(map: &[&'a [u8]], side_length: usize) -> [(Map<'a>, usize, usize); 6] {
    let mut result: [(Map<'a>, usize, usize); 6] = Default::default();

    let mut row_holder = [(); 4].map(|_| Map::new());
    let mut index = 0;

    for (y, block_row) in map.chunks_exact(side_length).enumerate() {
        for row in block_row {
            for (i, segment) in row.chunks_exact(side_length).enumerate() {
                if segment[0] != b' ' {
                    row_holder[i].push(segment);
                }
            }
        }

        for (x, potential_side) in row_holder.iter_mut().enumerate() {
            if !potential_side.is_empty() {
                mem::swap(potential_side, &mut result[index].0);
                result[index].1 = x;
                result[index].2 = y;
                index += 1;
            }
        }
    }

    result
}

pub fn part2(input: &[u8]) -> Result<String> {
    let (map, steps) = parse_input(input, parse_map)?;

    let side_length = side_length_of(&map);

    let squares = break_squares(&map, side_length);

    let convert_coords = |square: usize, x: usize, y: usize| {
        let offset_x = squares[square].1 * side_length + x + 1;
        let offset_y = squares[square].2 * side_length + y + 1;

        (offset_x, offset_y)
    };

    let mut current_square = 0;
    let mut y = 0;
    let mut x = find_starting_x(squares[current_square].0[y])?;
    let mut dir = Direction::Right;

    for step in steps {
        match step {
            Step::Left => dir = dir.turn_left(),
            Step::Right => dir = dir.turn_right(),
            Step::Forward(mut amount) => {
                'outer: while amount > 0 {
                    let map = &squares[current_square].0;

                    // Need to allow unused range bound, since we're actually tracking how much we
                    // still have to do. The loops are expected to end early, and we should remember
                    // how much work we've actually done.
                    #[allow(clippy::mut_range_bound)]
                    let coord = match dir {
                        Direction::Up => {
                            for _ in 0..amount {
                                if y == 0 {
                                    break;
                                } else if map[y - 1][x] == b'#' {
                                    break 'outer;
                                } else {
                                    y -= 1;
                                }
                            }

                            x
                        }
                        Direction::Down => {
                            for _ in 0..amount {
                                if y + 1 >= side_length {
                                    break;
                                } else if map[y + 1][x] == b'#' {
                                    break 'outer;
                                } else {
                                    amount -= 1;
                                    y += 1;
                                }
                            }

                            x
                        }
                        Direction::Left => {
                            for _ in 0..amount {
                                if x == 0 {
                                    break;
                                } else if map[y][x - 1] == b'#' {
                                    break 'outer;
                                } else {
                                    x -= 1;
                                    amount -= 1;
                                }
                            }

                            y
                        }
                        Direction::Right => {
                            for _ in 0..amount {
                                if x + 1 >= side_length {
                                    break;
                                } else if map[y][x + 1] == b'#' {
                                    break 'outer;
                                } else {
                                    amount -= 1;
                                    x += 1;
                                }
                            }

                            y
                        }
                    };

                    if amount > 0 {
                        let (new_dir, new_square, invert) =
                            TRANSITIONS[current_square][dir as usize];

                        let flipped_coord = if invert {
                            side_length - 1 - coord
                        } else {
                            coord
                        };

                        let (new_x, new_y) = match new_dir {
                            Direction::Up => (flipped_coord, side_length - 1),
                            Direction::Down => (flipped_coord, 0),
                            Direction::Left => (side_length - 1, flipped_coord),
                            Direction::Right => (0, flipped_coord),
                        };

                        if squares[new_square].0[new_y][new_x] == b'#' {
                            break 'outer;
                        }

                        x = new_x;
                        y = new_y;
                        current_square = new_square;
                        dir = new_dir;
                        amount -= 1;
                    }
                }
            }
        }
    }

    let (real_x, real_y) = convert_coords(current_square, x, y);

    Ok((1000 * real_y + 4 * real_x + dir as usize).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/22.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "6032");
    }

    #[test]
    fn test_side_length() {
        let (map, _) = parse_input(SAMPLE, parse_map).unwrap();

        assert_eq!(side_length_of(&map), 4);
    }

    #[test]
    fn test_break_squares() {
        let (map, _) = parse_input(SAMPLE, parse_map).unwrap();

        let side_length = side_length_of(&map);
        let squares = break_squares(&map, side_length);

        assert_eq!(squares[0].1, 2);
        assert_eq!(squares[0].2, 0);

        assert_eq!(squares[5].1, 3);
        assert_eq!(squares[5].2, 2);

        for square in squares {
            assert_eq!(square.0.len(), side_length);

            for row in square.0 {
                assert_eq!(row.len(), side_length);
            }
        }
    }

    #[test]
    fn test_sanity_transitions() {
        for (cur_face, &face) in TRANSITIONS.iter().enumerate() {
            for (dir, (arrive_dir, arrive_face, invert)) in face.into_iter().enumerate() {
                let inverse_dir = (arrive_dir as usize + 2) % 4;
                let (back_dir, back_face, back_invert) = TRANSITIONS[arrive_face][inverse_dir];

                assert_eq!(
                    invert, back_invert,
                    "Reciprocal invert failed: face {cur_face} dir {dir} to face {arrive_face} arrives as {arrive_dir:?}"
                );

                assert_eq!(back_face, cur_face, "Reciprocal transition failed: face {cur_face} dir {dir} arrives at {arrive_face} but returns at {back_face}");

                let correct_back_dir = (dir + 2) % 4;

                assert_eq!(back_dir as usize, correct_back_dir, "Reciprocal direction failed: face {cur_face} dir {dir} did not arrive the opposite direction from {arrive_face}");
            }
        }
    }
}
