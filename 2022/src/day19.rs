use std::ops::Add;
use std::ops::Sub;

use anyhow::Result;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::character::streaming::alpha1;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use crate::common::parse_input;

#[repr(usize)]
#[derive(Clone, Copy)]
enum Mineral {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl TryFrom<&'_ [u8]> for Mineral {
    type Error = String;

    fn try_from(value: &'_ [u8]) -> std::result::Result<Self, Self::Error> {
        dbg!(String::from_utf8_lossy(value));
        match value {
            b"ore" => Ok(Self::Ore),
            b"clay" => Ok(Self::Clay),
            b"obsidian" => Ok(Self::Obsidian),
            b"geode" => Ok(Self::Geode),
            other => Err(format!(
                "Invalid mineral '{}'",
                String::from_utf8_lossy(other)
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Resources([u8; 4]);

impl Resources {
    fn enough_for(self, other: Self) -> bool {
        self.0.iter().zip(&other.0).all(|(a, b)| a >= b)
    }
}

impl Sub for Resources {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(std::array::from_fn(|i| self.0[i] - rhs.0[i]))
    }
}

impl Add<[u8; 4]> for Resources {
    type Output = Self;

    fn add(self, rhs: [u8; 4]) -> Self::Output {
        Self(std::array::from_fn(|i| self.0[i] + rhs[i]))
    }
}

#[derive(Debug)]
struct BluePrint {
    id: u32,
    costs: [Resources; 4],
}

impl BluePrint {
    pub fn max_geodes(&self) -> u8 {
        self.max_geodes_recursive(24, 0, [1, 0, 0, 0], Resources::default())
    }

    fn max_geodes_recursive(
        &self,
        time_left: u32,
        // forbidden is a bitset for convenience
        forbidden: u8,
        machines: [u8; 4],
        resources: Resources,
    ) -> u8 {
        if time_left <= 1 {
            return resources.0[3] + machines[3] * (time_left as u8);
        }

        let resources_after = resources + machines;

        let mut best = 0;

        let mut can_buy = 0;

        for (i, &cost) in self.costs.iter().enumerate() {
            if ((1 << i) & forbidden) == 0 && resources.enough_for(cost) {
                can_buy |= 1 << i;
                let mut new_machines = machines;
                new_machines[i] += 1;

                best = best.max(self.max_geodes_recursive(
                    time_left - 1,
                    0,
                    new_machines,
                    resources_after - cost,
                ))
            }
        }

        best.max(self.max_geodes_recursive(
            time_left - 1,
            forbidden | can_buy,
            machines,
            resources_after,
        ))
    }
}

fn parse_blueprint(input: &[u8]) -> IResult<&[u8], BluePrint> {
    use nom::character::complete::u32;

    fn parse_mineral(input: &[u8]) -> IResult<&[u8], Mineral> {
        map_res(alpha1, Mineral::try_from)(input)
    }

    fn parse_cost(input: &[u8]) -> IResult<&[u8], (u8, Mineral)> {
        separated_pair(nom::character::complete::u8, tag(" "), parse_mineral)(input)
    }

    let (mut input, id) =
        terminated(delimited(tag("Blueprint "), u32, tag(":")), multispace1)(input)?;

    let mut costs: [Resources; 4] = Default::default();

    let mut parse_robot = terminated(
        tuple((
            preceded(tag("Each "), parse_mineral),
            preceded(tag(" robot costs "), parse_cost),
            terminated(opt(preceded(tag(" and "), parse_cost)), tag(".")),
        )),
        multispace1,
    );

    for _ in 0..4 {
        let (remaining, (element, (amount1, req1), cost2)) = parse_robot(input)?;
        input = remaining;

        costs[element as usize].0[req1 as usize] = amount1;

        if let Some((amount2, req2)) = cost2 {
            costs[element as usize].0[req2 as usize] = amount2;
        }
    }

    Ok((input, BluePrint { id, costs }))
}

pub fn part1(input: &[u8]) -> Result<String> {
    let blueprints = parse_input(input, many1(parse_blueprint))?;

    Ok(blueprints
        .into_iter()
        .map(|bp| dbg!(bp.max_geodes()) as u32 * bp.id)
        .sum::<u32>()
        .to_string())
}

pub fn part2(_input: &[u8]) -> Result<String> {
    anyhow::bail!("not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("./samples/19.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "33");
    }
}
