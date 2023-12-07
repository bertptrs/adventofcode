use std::mem;

use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::newline;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::Parser;

use crate::common::parse_input;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Kind {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[inline]
fn kind_parser(cards: &[u8; 5], part2: bool) -> Kind {
    let mut counts = [0u8; 15];
    for &card in cards {
        counts[card as usize] += 1;
    }

    let jokers = if part2 {
        mem::take(&mut counts[1]) as usize
    } else {
        0
    };

    let mut counts_counts = [0u8; 6];
    for count in counts {
        counts_counts[count as usize] += 1;
    }

    let mut first = 0;
    let mut second = 0;

    for (count, &occurrences) in counts_counts.iter().enumerate() {
        match occurrences {
            0 => continue,
            1 => {
                second = first;
                first = count;
            }
            _ => {
                first = count;
                second = count;
            }
        }
    }

    match (first + jokers, second) {
        (5, _) => Kind::FiveOfAKind,
        (4, _) => Kind::FourOfAKind,
        (3, 2) => Kind::FullHouse,
        (3, _) => Kind::ThreeOfAKind,
        (2, 2) => Kind::TwoPair,
        (2, _) => Kind::Pair,
        _ => Kind::HighCard,
    }
}

struct Hand {
    cards: [u8; 5],
    bid: u32,
    kind: Kind,
}

#[inline]
fn map_card(c: u8, part2: bool) -> anyhow::Result<u8> {
    Ok(match c {
        d @ b'2'..=b'9' => d - b'0',
        b'T' => 10,
        b'J' => {
            if part2 {
                1
            } else {
                11
            }
        }
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        other => anyhow::bail!("Invalid card {other}"),
    })
}

#[inline]
fn hands_parser<'a>(part2: bool) -> impl Parser<&'a [u8], Vec<Hand>, nom::error::Error<&'a [u8]>> {
    many1(map_res(
        pair(
            terminated(take(5usize), tag(" ")),
            terminated(nom::character::complete::u32, newline),
        ),
        move |(hand, bid)| -> anyhow::Result<Hand> {
            let mut cards = [0; 5];
            for (t, &s) in cards.iter_mut().zip(hand) {
                *t = map_card(s, part2)?
            }
            let kind = kind_parser(&cards, part2);

            Ok(Hand { cards, bid, kind })
        },
    ))
}

fn parts_common(hands: &mut [Hand]) -> anyhow::Result<String> {
    hands.sort_unstable_by(|first, second| {
        first
            .kind
            .cmp(&second.kind)
            .then_with(|| first.cards.cmp(&second.cards))
    });

    let sum: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u64 * u64::from(hand.bid))
        .sum();

    Ok(sum.to_string())
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let mut hands = parse_input(input, hands_parser(false))?;

    parts_common(&mut hands)
}

// Too high: 248859461
pub fn part2(input: &[u8]) -> anyhow::Result<String> {
    let mut hands = parse_input(input, hands_parser(true))?;

    parts_common(&mut hands)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/07.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "6440");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "5905");
    }
}
