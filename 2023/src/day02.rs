use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::iterator;
use nom::combinator::opt;
use nom::combinator::value;
use nom::multi::fold_many1;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;

use crate::common::convert_nom_error;
use crate::common::parse_input;

#[derive(Clone, Copy)]
#[repr(usize)]
enum Color {
    Red,
    Green,
    Blue,
}

fn parse_game(i: &[u8]) -> IResult<&[u8], (u8, [u8; 3])> {
    let parse_color = terminated(
        separated_pair(
            nom::character::complete::u8,
            tag(" "),
            alt((
                value(Color::Red, tag("red")),
                value(Color::Green, tag("green")),
                value(Color::Blue, tag("blue")),
            )),
        ),
        opt(alt((tag(", "), tag("; ")))),
    );

    separated_pair(
        preceded(tag("Game "), nom::character::complete::u8),
        tag(": "),
        terminated(
            fold_many1(
                parse_color,
                || [0u8; 3],
                |mut cur, (value, color)| {
                    cur[color as usize] = Ord::max(cur[color as usize], value);
                    cur
                },
            ),
            newline,
        ),
    )(i)
}

fn parts_common(input: &[u8], map: impl Fn((u8, [u8; 3])) -> u32) -> anyhow::Result<String> {
    let mut game_it = iterator(input, parse_game);

    let total: u32 = game_it.into_iter().map(map).sum();

    game_it.finish().map_err(|e| match e {
        nom::Err::Incomplete(_) => anyhow::anyhow!("unreachable"),
        nom::Err::Failure(e) | nom::Err::Error(e) => convert_nom_error(e),
    })?;

    Ok(total.to_string())
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    parts_common(input, |(id, colors)| {
        if colors[0] <= 12 && colors[1] <= 13 && colors[2] <= 14 {
            u32::from(id)
        } else {
            0
        }
    })
}

pub fn part2(input: &[u8]) -> anyhow::Result<String> {
    parts_common(input, |(_, colors)| {
        u32::from(colors[0]) * u32::from(colors[1]) * u32::from(colors[2])
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/02.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "8");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "2286");
    }
}
