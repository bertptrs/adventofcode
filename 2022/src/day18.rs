use ahash::AHashMap;
use ahash::AHashSet;
use anyhow::Context;
use anyhow::Result;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::fold_many1;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use crate::common::parse_input;

fn parse_voxels(input: &[u8]) -> IResult<&[u8], AHashSet<[u8; 3]>> {
    use nom::character::complete::u8;

    fold_many1(
        terminated(
            map(
                tuple((u8, preceded(tag(","), u8), preceded(tag(","), u8))),
                |(x, y, z)| [x, y, z],
            ),
            newline,
        ),
        AHashSet::new,
        |mut set, coord| {
            set.insert(coord);
            set
        },
    )(input)
}

pub fn part1(input: &[u8]) -> Result<String> {
    let voxels = parse_input(input, parse_voxels)?;

    let mut faces = 0;

    for &voxel in &voxels {
        for axis in 0..3 {
            for offset in [-1, 1] {
                let mut to_search = voxel;
                to_search[axis] = to_search[axis].wrapping_add_signed(offset);

                if !voxels.contains(&to_search) {
                    faces += 1;
                }
            }
        }
    }

    Ok(faces.to_string())
}

pub fn part2(input: &[u8]) -> Result<String> {
    let voxels = parse_input(input, parse_voxels)?;
    let max = voxels
        .iter()
        .copied()
        .flatten()
        .max()
        .context("No voxels?")?;

    let mut outside = AHashMap::new();
    let mut outside_candidates = AHashSet::new();
    let mut todo = Vec::new();

    let mut is_outside = |voxel: [u8; 3]| {
        if let Some(&state) = outside.get(&voxel) {
            return state;
        }

        let mut is_outside = false;

        todo.push(voxel);
        outside_candidates.insert(voxel);

        'outer: while let Some(voxel) = todo.pop() {
            for axis in 0..3 {
                for offset in [-1, 1] {
                    let mut to_search = voxel;
                    if let Some(new_coord) = to_search[axis].checked_add_signed(offset) {
                        to_search[axis] = new_coord;

                        if voxels.contains(&to_search) {
                            // Filled voxels cannot lead outside
                            continue;
                        } else if new_coord >= max {
                            is_outside = true;
                            break 'outer;
                        } else if let Some(&state) = outside.get(&to_search) {
                            is_outside = state;
                            break 'outer;
                        } else if outside_candidates.insert(to_search) {
                            todo.push(to_search);
                        }
                    } else {
                        // Managed to get below zero, which is outside
                        is_outside = true;
                        break 'outer;
                    }
                }
            }
        }

        let map = |voxel| (voxel, is_outside);

        outside.extend(todo.drain(..).map(map));
        outside.extend(outside_candidates.drain().map(map));

        is_outside
    };

    let mut faces = 0;

    for &voxel in &voxels {
        for axis in 0..3 {
            for offset in [-1, 1] {
                let mut to_search = voxel;
                to_search[axis] = to_search[axis].wrapping_add_signed(offset);

                if !voxels.contains(&to_search) && is_outside(to_search) {
                    faces += 1;
                }
            }
        }
    }

    Ok(faces.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/18.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "64");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "58");
    }
}
