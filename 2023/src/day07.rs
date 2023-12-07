use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::newline;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::IResult;

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

impl<'a> From<&'a [u8; 5]> for Kind {
    fn from(cards: &'a [u8; 5]) -> Self {
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
}

struct Hand {
    cards: [u8; 5],
    bid: u32,
    kind: Kind,
}

fn parse_hands(i: &[u8]) -> IResult<&[u8], Vec<Hand>> {
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
        |(hand, bid)| -> anyhow::Result<Hand> {
            let mut cards = [0; 5];
            for (t, &s) in cards.iter_mut().zip(hand) {
                *t = map_card(s)?
            }
            let kind = Kind::from(&cards);

            Ok(Hand { cards, bid, kind })
        },
    ))(i)
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let mut hands = parse_input(input, parse_hands)?;

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

pub fn part2(_input: &[u8]) -> anyhow::Result<String> {
    anyhow::bail!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/07.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "6440");
    }
}
