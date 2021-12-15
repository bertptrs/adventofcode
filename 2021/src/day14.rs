use std::io::Read;

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::character::is_alphabetic;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::Finish;
use nom::IResult;

type Rule = (u8, u8, u8);

type Pairs = [[u64; 26]; 26];
type Rules = [[u8; 26]; 26];

fn parse_input(input: &[u8]) -> IResult<&[u8], (&[u8], Vec<Rule>)> {
    use nom::character::complete::char;
    use nom::number::complete::u8;

    let parse_start = take_while(is_alphabetic);
    let parse_rule = tuple((u8, u8, preceded(tag(" -> "), u8)));

    tuple((
        parse_start,
        preceded(tag("\n\n"), many0(terminated(parse_rule, char('\n')))),
    ))(input)
}

fn read_input(input: &mut dyn Read) -> (u8, u8, Pairs, Rules) {
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer).unwrap();

    let (initial, rules) = parse_input(&buffer).finish().unwrap().1;

    let mut pairs = Pairs::default();

    for window in initial.windows(2) {
        pairs[(window[0] - b'A') as usize][(window[1] - b'A') as usize] += 1;
    }

    let mut rule_map = Rules::default();
    for (first, second, product) in rules {
        rule_map[(first - b'A') as usize][(second - b'A') as usize] = product - b'A';
    }

    (
        initial[0] - b'A',
        initial[initial.len() - 1] - b'A',
        pairs,
        rule_map,
    )
}

fn update(pairs: Pairs, rules: &Rules) -> Pairs {
    let mut new_pairs = Pairs::default();

    pairs.iter().enumerate().for_each(|(first, row)| {
        row.iter().enumerate().for_each(|(second, &count)| {
            let product = rules[first][second] as usize;
            new_pairs[first][product] += count;
            new_pairs[product][second] += count;
        })
    });

    new_pairs
}

fn parts_common(input: &mut dyn Read, rounds: usize) -> String {
    let (first, last, mut pairs, rules) = read_input(input);

    (0..rounds).for_each(|_| pairs = update(pairs, &rules));

    let mut pair_counts = [0; 26];
    pairs.iter().enumerate().for_each(|(first, row)| {
        row.iter().enumerate().for_each(|(second, &count)| {
            pair_counts[first] += count;
            pair_counts[second] += count;
        })
    });

    pair_counts[first as usize] += 1;
    pair_counts[last as usize] += 1;

    // Now everything is counted twice, so first half everything
    let counts = pair_counts.map(|pair_count| pair_count / 2);

    match counts.into_iter().filter(|&c| c != 0).minmax() {
        itertools::MinMaxResult::NoElements => unreachable!(),
        itertools::MinMaxResult::OneElement(_) => 0,
        itertools::MinMaxResult::MinMax(min, max) => max - min,
    }
    .to_string()
}

pub fn part1(input: &mut dyn Read) -> String {
    parts_common(input, 10)
}

pub fn part2(input: &mut dyn Read) -> String {
    parts_common(input, 40)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_implementation;

    const SAMPLE: &[u8] = include_bytes!("samples/14.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 1588);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 2188189693529u64);
    }
}
