use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use crate::common::parse_input;

const EPSILON: f64 = 1e-6;

struct Hail {
    position: [i64; 3],
    speed: [i64; 3],
}

impl Hail {
    // Convert to y = ax + b form
    fn to_yab(&self) -> (f64, f64) {
        debug_assert_ne!(0, self.speed[0]);

        let slope = self.speed[1] as f64 / self.speed[0] as f64;
        let offset = self.position[1] as f64 - self.position[0] as f64 * slope;

        (slope, offset)
    }

    fn intersect(&self, other: &Self, min: f64, max: f64) -> bool {
        let (a1, b1) = self.to_yab();
        let (a2, b2) = other.to_yab();

        if (a1 - a2).abs() < EPSILON {
            return false;
        }

        let a = a1 - a2;
        let b = b2 - b1;

        let x = b / a;
        let y = a1 * x + b1;

        let t1 = (x - self.position[0] as f64) / self.speed[0] as f64;
        let t2 = (x - other.position[0] as f64) / other.speed[0] as f64;

        if t1 < 0.0 || t2 < 0.0 {
            return false;
        }

        // use the formula for X
        x >= min && x <= max && y >= min && y <= max
    }

    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        use nom::character::complete::i64;
        let sep = |i| tuple((tag(","), space1))(i);
        let parse_coordinates = move |i| {
            map(
                tuple((terminated(i64, sep), terminated(i64, sep), i64)),
                |(x, y, z)| [x, y, z],
            )(i)
        };

        map(
            tuple((
                terminated(parse_coordinates, tuple((tag(" @"), space1))),
                terminated(parse_coordinates, tag("\n")),
            )),
            |(position, speed)| Self { position, speed },
        )(i)
    }
}
fn parse_hail(i: &[u8]) -> IResult<&[u8], Vec<Hail>> {
    many1(Hail::parse)(i)
}

fn part1_parametrized(input: &[u8], min: f64, max: f64) -> anyhow::Result<String> {
    let hail = parse_input(input, parse_hail)?;

    let intersections = hail
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            hail[i + 1..]
                .iter()
                .map(move |b| (a, b))
                .filter(|(a, b)| a.intersect(b, min, max))
        })
        .count();

    Ok(intersections.to_string())
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    part1_parametrized(input, 200000000000000.0, 400000000000000.0)
}

pub fn part2(_input: &[u8]) -> anyhow::Result<String> {
    anyhow::bail!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/24.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("2", part1_parametrized(SAMPLE, 7.0, 27.0).unwrap());
    }
}
