use std::io::Read;
use std::iter::repeat;

use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

use crate::common::ordered;
use crate::common::read_input;
use crate::common::BitSet;

type Coord = (u16, u16);

fn coordinates(input: &[u8]) -> IResult<&[u8], Coord> {
    use nom::character::complete::char;
    use nom::character::complete::u16;

    separated_pair(u16, char(','), u16)(input)
}

fn parse_input(input: &[u8]) -> IResult<&[u8], Vec<(Coord, Coord)>> {
    let read_line = map(
        separated_pair(coordinates, tag(" -> "), coordinates),
        |(begin, end)| ordered(begin, end),
    );

    separated_list1(newline, read_line)(input)
}

fn stripe(
    once: &mut BitSet,
    twice: &mut BitSet,
    width: usize,
    xs: impl Iterator<Item = u16>,
    ys: impl Iterator<Item = u16>,
) {
    for (x, y) in xs.zip(ys) {
        let index = x as usize + y as usize * width;
        if !once.insert(index) {
            twice.insert(index);
        }
    }
}

fn part_common(input: &mut dyn Read, diagonals: bool) -> String {
    let lines = read_input(input, parse_input);

    let width = lines
        .iter()
        .map(|&(_, (x, _))| x as usize + 1)
        .max()
        .unwrap();

    let mut once_map = BitSet::new();
    let mut twice_map = BitSet::new();

    for (begin, end) in lines {
        if begin.0 == end.0 {
            let y_range = begin.1..=end.1;
            stripe(
                &mut once_map,
                &mut twice_map,
                width,
                repeat(begin.0),
                y_range,
            );
        } else if begin.1 == end.1 {
            let x_range = begin.0..=end.0;
            stripe(
                &mut once_map,
                &mut twice_map,
                width,
                x_range,
                repeat(begin.1),
            );
        } else if diagonals {
            let x_range = begin.0..=end.0;
            let y_range = (begin.1.min(end.1))..=(begin.1.max(end.1));

            if begin.1 > end.1 {
                // For a downward slope we need to reverse Y
                stripe(&mut once_map, &mut twice_map, width, x_range, y_range.rev());
            } else {
                stripe(&mut once_map, &mut twice_map, width, x_range, y_range);
            }
        }
    }

    twice_map.len().to_string()
}

pub fn part1(input: &mut dyn Read) -> String {
    part_common(input, false)
}

pub fn part2(input: &mut dyn Read) -> String {
    part_common(input, true)
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/05.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 5)
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 12)
    }
}
