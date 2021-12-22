use std::io::Read;
use std::ops::RangeInclusive;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::sequence::tuple;
use nom::IResult;

use crate::common::read_input;

type CRange = RangeInclusive<i32>;

fn parse_range(input: &[u8]) -> IResult<&[u8], CRange> {
    use nom::character::complete::i32;

    map(separated_pair(i32, tag(".."), i32), |(first, last)| {
        first..=last
    })(input)
}

fn parse_input(input: &[u8]) -> IResult<&[u8], Vec<(bool, [CRange; 3])>> {
    let parse_state = alt((map(tag("on x="), |_| true), map(tag("off x="), |_| false)));
    let parse_line = map(
        tuple((
            parse_state,
            parse_range,
            preceded(tag(",y="), parse_range),
            preceded(tag(",z="), parse_range),
        )),
        |(b, x, y, z)| (b, [x, y, z]),
    );

    separated_list1(newline, parse_line)(input)
}

pub fn part1(input: &mut dyn Read) -> String {
    const MAX_ABS_VAL: i32 = 50;
    const SIDE_LEN: usize = 2 * (MAX_ABS_VAL as usize) + 1;

    let mut state = [[0u128; SIDE_LEN]; SIDE_LEN];

    let valid_range = -MAX_ABS_VAL..=MAX_ABS_VAL;

    let ranges = read_input(input, parse_input);

    for (toggle, [xr, yr, zr]) in ranges {
        for z in zr {
            if !valid_range.contains(&z) {
                continue;
            }
            for y in yr.clone() {
                if !valid_range.contains(&y) {
                    continue;
                }
                let row = &mut state[(z + MAX_ABS_VAL) as usize][(y + MAX_ABS_VAL) as usize];

                for x in xr.clone() {
                    if !valid_range.contains(&x) {
                        continue;
                    }
                    let bit = 1 << (x + MAX_ABS_VAL);

                    if toggle {
                        *row |= bit;
                    } else {
                        *row &= !bit;
                    }
                }
            }
        }
    }

    state
        .iter()
        .flatten()
        .map(|val| val.count_ones())
        .sum::<u32>()
        .to_string()
}

pub fn part2(_input: &mut dyn Read) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/22.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 590784);
    }

    #[test]
    fn sample_part2() {
        // test_implementation(part2, SAMPLE, 230)
    }
}
