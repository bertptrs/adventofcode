use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::newline;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::Parser;

use crate::common::parse_input;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn kind_parser1(cards: &[u8; 5]) -> Kind {
    let mut counts = [0u8; 15];
    for &card in cards {
        counts[card as usize] += 1;
    }

    counts.sort_unstable();
    match (counts[14], counts[13]) {
        (5, _) => Kind::FiveOfAKind,
        (4, _) => Kind::FourOfAKind,
        (3, 2) => Kind::FullHouse,
        (3, _) => Kind::ThreeOfAKind,
        (2, 2) => Kind::TwoPair,
        (2, _) => Kind::Pair,
        _ => Kind::HighCard,
    }
}

fn kind_parser2(cards: &[u8; 5]) -> Kind {
    let mut counts = [0u8; 15];
    for &card in cards {
        counts[card as usize] += 1;
    }

    let jokers = counts[11];
    counts[11] = 0;

    counts.sort_unstable();
    match (counts[14] + jokers, counts[13]) {
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

fn hands_parser<'a>(
    kind_parser: impl Fn(&[u8; 5]) -> Kind,
) -> impl Parser<&'a [u8], Vec<Hand>, nom::error::Error<&'a [u8]>> {
    fn map_card(c: u8) -> anyhow::Result<u8> {
        Ok(match c {
            d @ b'2'..=b'9' => d - b'0',
            b'T' => 10,
            b'J' => 11,
            b'Q' => 12,
            b'K' => 13,
            b'A' => 14,
            other => anyhow::bail!("Invalid card {other}"),
        })
    }

    many1(map_res(
        pair(
            terminated(take(5usize), tag(" ")),
            terminated(nom::character::complete::u32, newline),
        ),
        move |(hand, bid)| -> anyhow::Result<Hand> {
            let mut cards = [0; 5];
            for (t, &s) in cards.iter_mut().zip(hand) {
                *t = map_card(s)?
            }
            let kind = kind_parser(&cards);

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
    let mut hands = parse_input(input, hands_parser(kind_parser1))?;

    parts_common(&mut hands)
}

// Too high: 248859461
pub fn part2(input: &[u8]) -> anyhow::Result<String> {
    let mut hands = parse_input(input, hands_parser(kind_parser2))?;

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
