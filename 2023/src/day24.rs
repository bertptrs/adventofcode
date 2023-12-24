use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;
use num_integer::Integer;

use crate::common::parse_input;

struct Hail {
    position: [i64; 3],
    speed: [i64; 3],
}

impl Hail {
    fn intersect(&self, other: &Self) -> bool {
        // Assumption: speed in no coordinate is 0. This happens to be true.
        let multiplier = self.speed[0].lcm(&self.speed[1]);

        let mult_x = multiplier / self.speed[0];
        let mult_y = multiplier / self.speed[1];

        // use the formula for X
        false
    }

    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        use nom::character::complete::i64;
        let parse_coordinates = |i| {
            map(
                tuple((terminated(i64, tag(", ")), terminated(i64, tag(", ")), i64)),
                |(x, y, z)| [x, y, z],
            )(i)
        };

        map(
            tuple((
                terminated(parse_coordinates, tag(" @ ")),
                terminated(parse_coordinates, tag("\n")),
            )),
            |(position, speed)| Self { position, speed },
        )(i)
    }
}
fn parse_hail(i: &[u8]) -> IResult<&[u8], Vec<Hail>> {
    many1(Hail::parse)(i)
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let hail = parse_input(input, parse_hail)?;

    let intersections = hail
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            hail[i + 1..]
                .iter()
                .map(move |b| (a, b))
                .filter(|(a, b)| a.intersect(b))
        })
        .count();

    Ok(intersections.to_string())
}

pub fn part2(_input: &[u8]) -> anyhow::Result<String> {
    anyhow::bail!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/24.txt");

    #[test]
    #[ignore = "not completely implemented"]
    fn sample_part1() {
        assert_eq!("2", part1(SAMPLE).unwrap());
    }
}
