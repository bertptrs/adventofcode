use ahash::AHashSet;
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

pub fn part2(_input: &[u8]) -> Result<String> {
    anyhow::bail!("not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/18.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "64");
    }
}
