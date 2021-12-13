use std::collections::HashSet;
use std::io::Read;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::error::Error;
use nom::sequence::tuple;
use nom::Finish;
use nom::IResult;

use crate::common::LineIter;
use crate::common::LineParser;

type Coords = (u16, u16);

enum Fold {
    X(u16),
    Y(u16),
}

impl FromStr for Fold {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, fold) = parse_fold(s)
            .finish()
            .map_err(|Error { input, code }| Error {
                input: input.to_string(),
                code,
            })?;

        Ok(fold)
    }
}

fn parse_coordinates(input: &str) -> IResult<&str, Coords> {
    use nom::character::complete::char;
    use nom::character::complete::u16;

    let (input, (x, _, y)) = tuple((u16, char(','), u16))(input)?;

    Ok((input, (x, y)))
}

fn parse_fold(input: &str) -> IResult<&str, Fold> {
    use nom::character::complete::char;
    use nom::character::complete::u16;

    let (input, (_, axis, _, coord)) = tuple((
        tag("fold along "),
        alt((char('x'), char('y'))),
        char('='),
        u16,
    ))(input)?;

    let fold = match axis {
        'x' => Fold::X(coord),
        'y' => Fold::Y(coord),
        _ => unreachable!("Should be filtered by nom"),
    };

    Ok((input, fold))
}

fn read_dots(reader: &mut LineIter<'_>) -> HashSet<Coords> {
    let mut dots = HashSet::new();

    while let Some(line) = reader.next() {
        if line.is_empty() {
            break;
        }

        let (_, coords) = parse_coordinates(line).unwrap();

        dots.insert(coords);
    }

    dots
}

fn apply_fold(dots: &mut HashSet<Coords>, fold: Fold, to_fold: &mut Vec<Coords>) {
    match fold {
        Fold::X(coord) => dots.retain(|&(x, y)| {
            if x < coord {
                true
            } else {
                to_fold.push((2 * coord - x, y));
                false
            }
        }),
        Fold::Y(coord) => dots.retain(|&(x, y)| {
            if y < coord {
                true
            } else {
                to_fold.push((x, 2 * coord - y));
                false
            }
        }),
    }

    dots.extend(to_fold.drain(..));
}

fn print_dots(dots: &HashSet<Coords>) -> String {
    let (x, y) = dots
        .iter()
        .fold((0, 0), |(xc, yc), &(xn, yn)| (xc.max(xn), yc.max(yn)));

    let mut buffer = String::with_capacity((x as usize + 1) * y as usize);

    for y in 0..=y {
        for x in 0..=x {
            if dots.contains(&(x, y)) {
                buffer.push('#');
            } else {
                buffer.push(' ');
            }
        }
        buffer.push('\n');
    }

    buffer.pop();

    buffer
}

pub fn part1(input: &mut dyn Read) -> String {
    let mut reader = LineIter::new(input);

    let mut dots = read_dots(&mut reader);

    let fold = reader.next().unwrap().parse().unwrap();
    apply_fold(&mut dots, fold, &mut Vec::new());

    dots.len().to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let mut reader = LineIter::new(input);

    let mut dots = read_dots(&mut reader);
    let mut to_fold = Vec::new();

    LineParser::from(reader).for_each(|fold| apply_fold(&mut dots, fold, &mut to_fold));

    print_dots(&dots)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_implementation;

    const SAMPLE: &[u8] = include_bytes!("samples/13.txt");

    #[test]
    fn sample_part() {
        test_implementation(part1, SAMPLE, 17);
    }
}
