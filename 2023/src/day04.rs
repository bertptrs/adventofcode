use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::character::complete::space1;
use nom::combinator::iterator;
use nom::combinator::map;
use nom::multi::fold_many1;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;

use crate::common::convert_nom_error;
use crate::common::parse_input;

struct Card {
    have: u128,
    winning: u128,
}

fn parse_card(i: &[u8]) -> IResult<&[u8], Card> {
    fn parse_set(i: &[u8]) -> IResult<&[u8], u128> {
        fold_many1(
            preceded(space1, nom::character::complete::u8),
            || 0u128,
            |cur, bit| cur | (1 << bit),
        )(i)
    }

    map(
        pair(
            preceded(
                delimited(
                    pair(tag("Card"), space1),
                    nom::character::complete::u32,
                    tag(":"),
                ),
                parse_set,
            ),
            delimited(tag(" |"), parse_set, newline),
        ),
        |(have, winning)| Card { have, winning },
    )(i)
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let mut card_it = iterator(input, parse_card);

    let total: u32 = card_it
        .into_iter()
        .map(|card| {
            let winners = (card.have & card.winning).count_ones();

            if winners > 0 {
                1 << winners - 1
            } else {
                0
            }
        })
        .sum();

    card_it.finish().map_err(|e| match e {
        nom::Err::Incomplete(_) => anyhow::anyhow!("unreachable"),
        nom::Err::Failure(e) | nom::Err::Error(e) => convert_nom_error(e),
    })?;

    Ok(total.to_string())
}

pub fn part2(input: &[u8]) -> anyhow::Result<String> {
    let cards = parse_input(input, many1(parse_card))?;
    let mut counts = vec![1; cards.len()];

    for (id, card) in cards.iter().enumerate() {
        let winners = (card.have & card.winning).count_ones() as usize;
        let count = counts[id];

        for offset in 1..=winners {
            counts[id + offset] += count;
        }
    }

    let total: u32 = counts.iter().sum();

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/04.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "13");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "30");
    }
}
