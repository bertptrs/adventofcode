use std::collections::HashMap;
use std::io::Read;
use std::iter::repeat;

use nom::bytes::complete::tag;
use nom::sequence::tuple;
use nom::Finish;
use nom::IResult;

use crate::common::ordered;
use crate::common::LineIter;

type Coord = (u16, u16);

fn coordinates(input: &str) -> IResult<&str, Coord> {
    use nom::character::complete;

    let (input, (x, _, y)) = tuple((complete::u16, complete::char(','), complete::u16))(input)?;

    Ok((input, (x, y)))
}

fn line_definition(input: &str) -> IResult<&str, (Coord, Coord)> {
    let (input, (begin, _, end)) = tuple((coordinates, tag(" -> "), coordinates))(input)?;

    // Sorting the coordinates saves trouble later
    Ok((input, ordered(begin, end)))
}

fn stripe(
    map: &mut HashMap<Coord, u16>,
    xs: impl Iterator<Item = u16>,
    ys: impl Iterator<Item = u16>,
) {
    for (x, y) in xs.zip(ys) {
        *map.entry((x, y)).or_default() += 1;
    }
}

fn part_common(input: &mut dyn Read, diagonals: bool) -> String {
    let mut reader = LineIter::new(input);
    let mut map = HashMap::new();

    while let Some(line) = reader.next() {
        let (begin, end) = line_definition(line).finish().unwrap().1;

        if begin.0 == end.0 {
            let y_range = begin.1..=end.1;
            stripe(&mut map, repeat(begin.0), y_range);
        } else if begin.1 == end.1 {
            let x_range = begin.0..=end.0;
            stripe(&mut map, x_range, repeat(begin.1));
        } else if diagonals {
            let x_range = begin.0..=end.0;
            let y_range = (begin.1.min(end.1))..=(begin.1.max(end.1));

            if begin.1 > end.1 {
                // For a downward slope we need to reverse Y
                stripe(&mut map, x_range, y_range.rev());
            } else {
                stripe(&mut map, x_range, y_range);
            }
        }
    }

    map.values().filter(|&&v| v > 1).count().to_string()
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
    fn test_parser() {
        assert_eq!(line_definition("6,4 -> 2,0"), Ok(("", ((2, 0), (6, 4)))));
    }

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 5)
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 12)
    }
}
