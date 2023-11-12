use std::cmp::Ordering;
use std::collections::BinaryHeap;

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

#[derive(Debug)]
struct BluePrint {
    id: u32,
    costs: [[u8; 3]; 4],
}

impl BluePrint {
    pub fn max_geodes(&self, time: u8) -> u8 {
        /// How much would we produce if all we did was produce geode robots for the remaining time
        fn ideal(remaining: u32) -> u32 {
            if remaining <= 1 {
                0
            } else {
                (remaining - 1) * remaining / 2
            }
        }

        #[derive(Eq, PartialEq)]
        struct State {
            missed: u32,
            got: u8,
            time_left: u8,
            resources: [u8; 3],
            machines: [u8; 3],
        }

        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                Ordering::Equal
                    .then(other.missed.cmp(&self.missed))
                    .then(self.got.cmp(&other.got))
                    .then(self.time_left.cmp(&other.time_left))
                    .then(self.machines.cmp(&other.machines))
            }
        }

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let max_needed = self.max_needed();
        let mut todo = BinaryHeap::new();
        let mut best = 0;

        todo.push(State {
            missed: 0,
            got: 0,
            time_left: time,
            resources: [0; 3],
            machines: [1, 0, 0],
        });

        while let Some(State {
            missed,
            got,
            time_left,
            resources,
            machines,
        }) = todo.pop()
        {
            let ideal_from_now = ideal(u32::from(time_left));
            // Need to check again because we might've gotten a better result in the meantime.
            if u32::from(best - got) >= ideal_from_now {
                continue;
            }
            assert!(
                todo.len() <= 1_000_000,
                "Safety: got a todo list of len {}, best: {best}",
                todo.len()
            );
            for (element, &costs) in self.costs.iter().enumerate() {
                let Some(min_to_build) = self.until_buildable(costs, resources, machines) else {
                    break;
                };

                // +1 because we need a turn to build
                let built_after = min_to_build + 1;
                if built_after >= time_left {
                    continue;
                }

                // Ideally, would be written as a nice `array::from_fn`. It turns out that codegen
                // for `array::from_fn` is very bad, and writing it out into this for loop reduces
                // time taken by approximately 100%.
                let mut resources_after = [0; 3];
                for i in 0..3 {
                    resources_after[i] = resources[i] + machines[i] * built_after - costs[i];
                }

                let time_after = time_left - built_after;

                if element == Mineral::Geode as usize {
                    let new_got = got + time_after;
                    best = best.max(new_got);

                    if u32::from(best - new_got) >= ideal(time_after.into()) {
                        continue;
                    }

                    todo.push(State {
                        missed,
                        got: new_got,
                        time_left: time_after,
                        resources: resources_after,
                        machines,
                    });

                    best = best.max(new_got);
                } else {
                    if machines[element] >= max_needed[element]
                        || u32::from(best - got) >= ideal(time_after.into())
                    {
                        continue;
                    }

                    let mut new_machines = machines;
                    new_machines[element] += 1;
                    let new_missed = ideal_from_now - ideal(u32::from(time_after));
                    todo.push(State {
                        missed: new_missed,
                        got,
                        time_left: time_after,
                        resources: resources_after,
                        machines: new_machines,
                    })
                }
            }
        }

        best
    }

    #[inline]
    fn until_buildable(&self, costs: [u8; 3], resources: [u8; 3], machines: [u8; 3]) -> Option<u8> {
        let mut min_to_build = 0;
        for ((&cost, &avail), &machine) in costs.iter().zip(&resources).zip(&machines) {
            if cost > avail {
                if machine == 0 {
                    return None;
                } else {
                    min_to_build = min_to_build.max((cost - avail + machine - 1) / machine);
                }
            }
        }

        Some(min_to_build)
    }

    fn max_needed(&self) -> [u8; 3] {
        let mut max_needed = [0; 3];

        for cost in &self.costs {
            for (max, &new) in max_needed.iter_mut().zip(cost) {
                *max = (*max).max(new);
            }
        }

        max_needed
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

    let mut costs: [[u8; 3]; 4] = Default::default();

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

        costs[element as usize][req1 as usize] = amount1;

        if let Some((amount2, req2)) = cost2 {
            costs[element as usize][req2 as usize] = amount2;
        }
    }

    Ok((input, BluePrint { id, costs }))
}

pub fn part1(input: &[u8]) -> Result<String> {
    let blueprints = parse_input(input, many1(parse_blueprint))?;

    Ok(blueprints
        .into_iter()
        .map(|bp| u32::from(bp.max_geodes(24)) * bp.id)
        .sum::<u32>()
        .to_string())
}

pub fn part2(input: &[u8]) -> Result<String> {
    let blueprints = parse_input(input, many1(parse_blueprint))?;

    let result: u32 = blueprints
        .iter()
        .take(3)
        .map(|bp| u32::from(bp.max_geodes(32)))
        .product();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("./samples/19.txt");

    fn get_samples() -> Vec<BluePrint> {
        parse_input(SAMPLE, many1(parse_blueprint)).unwrap()
    }

    #[test]
    fn sample_part1() {
        let samples = get_samples();

        assert_eq!(samples[0].max_geodes(24), 9);
        assert_eq!(samples[1].max_geodes(24), 12);

        assert_eq!(part1(SAMPLE).unwrap(), "33");
    }

    #[test]
    fn sample_part2() {
        let samples = get_samples();

        assert_eq!(samples[0].max_geodes(32), 56);
        assert_eq!(samples[1].max_geodes(32), 62);
    }
}
