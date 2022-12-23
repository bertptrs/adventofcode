use std::collections::hash_map::Entry;
use std::ops::RangeInclusive;

use ahash::AHashMap;
use ahash::AHashSet;
use anyhow::Context;
use anyhow::Result;
use itertools::Itertools;
use nom::bytes::complete::take_until;
use nom::character::complete::newline;
use nom::multi::fold_many1;
use nom::sequence::terminated;
use nom::IResult;

use crate::common::enumerate;
use crate::common::parse_input;
use crate::common::Vec2;

const OPTIONS: [[Vec2; 3]; 4] = [
    // North
    [Vec2([0, -1]), Vec2([-1, -1]), Vec2([1, -1])],
    // South
    [Vec2([0, 1]), Vec2([-1, 1]), Vec2([1, 1])],
    // West
    [Vec2([-1, 0]), Vec2([-1, -1]), Vec2([-1, 1])],
    // East
    [Vec2([1, 0]), Vec2([1, -1]), Vec2([1, 1])],
];

fn parse_elves(input: &[u8]) -> IResult<&[u8], AHashSet<Vec2>> {
    fold_many1(
        enumerate(terminated(take_until("\n"), newline)),
        AHashSet::new,
        |mut elves, (y, line): (usize, &[u8])| {
            let y = y as i32;

            elves.extend(
                line.iter()
                    .enumerate()
                    .filter_map(|(x, &val)| (val == b'#').then_some(Vec2([x as i32, y]))),
            );

            elves
        },
    )(input)
}

fn determine_bounding_box(
    elves: &AHashSet<Vec2>,
) -> Result<(RangeInclusive<i32>, RangeInclusive<i32>)> {
    let (x_min, x_max) = elves
        .iter()
        .map(|&Vec2([x, _])| x)
        .minmax()
        .into_option()
        .context("Could not determine x range")?;

    let (y_min, y_max) = elves
        .iter()
        .map(|&Vec2([_, y])| y)
        .minmax()
        .into_option()
        .context("Could not determine y range")?;

    Ok((x_min..=x_max, y_min..=y_max))
}

#[allow(unused)]
fn print(elves: &AHashSet<Vec2>) -> Result<()> {
    let (x_bounds, y_bounds) = determine_bounding_box(elves)?;

    for y in y_bounds {
        for x in x_bounds.clone() {
            print!(
                "{}",
                if elves.contains(&Vec2([x, y])) {
                    '#'
                } else {
                    '.'
                }
            );
        }

        println!();
    }

    Ok(())
}

pub fn part1(input: &[u8]) -> Result<String> {
    let mut elves = parse_input(input, parse_elves)?;

    simulate(&mut elves, 10);

    let (x_bounds, y_bounds) = determine_bounding_box(&elves)?;

    let area = (x_bounds.end() - x_bounds.start() + 1) * (y_bounds.end() - y_bounds.start() + 1);

    let free = area - elves.len() as i32;

    Ok(free.to_string())
}

fn simulate(elves: &mut AHashSet<Vec2>, max: usize) -> Option<usize> {
    let mut todo = Vec::new();
    let mut to_return = Vec::new();
    let mut origin = AHashMap::new();

    for it in 0..max {
        // Remove all todos from a previous iteration
        todo.clear();

        // Find all the elves with at least one neighbour
        todo.extend(elves.iter().copied().filter(|&Vec2([x, y])| {
            for dx in [-1, 0, 1] {
                for dy in [-1, 0, 1] {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    if elves.contains(&Vec2([x + dx, y + dy])) {
                        return true;
                    }
                }
            }

            false
        }));

        for &elf in &todo {
            let mut moved = false;

            for &deltas in OPTIONS[(it % 4)..].iter().chain(&OPTIONS[..(it % 4)]) {
                if deltas
                    .into_iter()
                    .all(|delta| !elves.contains(&(elf + delta)))
                {
                    // Observation: any collision will only happen between opposite pairs of elves,
                    // not three. Otherwise they wouldn't have chosen to move this direction.

                    // Somewhat messy but it avoids computing the hash more than once per elf
                    match origin.entry(deltas[0] + elf) {
                        Entry::Occupied(entry) => {
                            to_return.push(elf);
                            to_return.push(entry.remove());
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(elf);
                        }
                    };

                    // We moved, or collided, but we shouldn't look other directions any more
                    moved = true;
                    break;
                }
            }

            if !moved {
                to_return.push(elf);
            }
        }

        if origin.is_empty() {
            return Some(it + 1);
        }

        // Remove entries we processed
        for elf in &todo {
            elves.remove(elf);
        }

        // Add back any entries we ended up not moving
        elves.extend(to_return.drain(..));

        // Add all the elves in their new positions
        elves.extend(origin.drain().map(|(dest, _)| dest));
    }

    None
}

pub fn part2(input: &[u8]) -> Result<String> {
    let mut elves = parse_input(input, parse_elves)?;

    let first_non_moved = simulate(&mut elves, usize::MAX).context("Elves didn't stop moving?")?;

    Ok(first_non_moved.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("./samples/23.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "110");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "20");
    }
}
