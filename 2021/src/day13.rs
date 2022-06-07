use std::io::Read;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;

use crate::common::read_input;

type Coords = (u16, u16);

#[derive(Copy, Clone)]
enum Fold {
    X(u16),
    Y(u16),
}

fn parse_input(input: &[u8]) -> IResult<&[u8], (Vec<Coords>, Vec<Fold>)> {
    use nom::character::complete::char;

    let parse_coordinates = many0(terminated(parse_coordinate, char('\n')));
    let parse_folds = many0(terminated(parse_fold, char('\n')));

    separated_pair(parse_coordinates, char('\n'), parse_folds)(input)
}

fn parse_coordinate(input: &[u8]) -> IResult<&[u8], Coords> {
    use nom::character::complete::char;
    use nom::character::complete::u16;

    separated_pair(u16, char(','), u16)(input)
}

fn parse_fold(input: &[u8]) -> IResult<&[u8], Fold> {
    use nom::character::complete::u16;

    preceded(
        tag("fold along "),
        alt((
            preceded(tag("x="), map(u16, Fold::X)),
            preceded(tag("y="), map(u16, Fold::Y)),
        )),
    )(input)
}

fn apply_fold(dots: &mut [Coords], fold: Fold) {
    match fold {
        Fold::X(coord) => dots.iter_mut().for_each(|(x, _)| {
            if *x >= coord {
                *x = 2 * coord - *x;
            }
        }),
        Fold::Y(coord) => dots.iter_mut().for_each(|(_, y)| {
            if *y >= coord {
                *y = 2 * coord - *y;
            }
        }),
    }
}

fn print_dots(dots: &[Coords]) -> String {
    let (width, height) = dots.iter().fold((0, 0), |(xc, yc), &(xn, yn)| {
        (xc.max(xn as usize + 1), yc.max(yn as usize + 1))
    });

    let mut buffer = vec![b' '; (width + 1) * height - 1];

    for &(x, y) in dots {
        buffer[x as usize + (width + 1) * y as usize] = b'#';
    }

    for line in buffer.chunks_exact_mut(width + 1) {
        line[width] = b'\n';
    }

    String::from_utf8(buffer).unwrap()
}

pub fn part1(input: &mut dyn Read) -> String {
    let (mut dots, folds) = read_input(input, parse_input);

    apply_fold(&mut dots, folds[0]);

    dots.sort_unstable();

    dots.into_iter().unique().count().to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let (mut dots, folds) = read_input(input, parse_input);

    folds
        .into_iter()
        .for_each(|fold| apply_fold(&mut dots, fold));

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
