use std::collections::HashSet;

use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use crate::common::minmax;
use crate::common::parse_input;

struct Brick([[u16; 3]; 2]);

impl Brick {
    fn lowest(&self) -> u16 {
        Ord::min(self.0[0][2], self.0[1][2])
    }

    fn iter_squares(&self) -> impl Iterator<Item = (u16, u16)> {
        let xs = minmax(self.0[0][0], self.0[1][0]);
        let ys = minmax(self.0[0][1], self.0[1][1]);

        (ys.0..=ys.1).flat_map(move |z| (xs.0..=xs.1).map(move |x| (x, z)))
    }

    fn parse(i: &[u8]) -> IResult<&[u8], Brick> {
        use nom::character::complete::u16;
        let parse_coordinates = |i| {
            map(
                tuple((terminated(u16, tag(",")), terminated(u16, tag(",")), u16)),
                |(x, y, z)| [x, y, z],
            )(i)
        };

        map(
            tuple((
                terminated(parse_coordinates, tag("~")),
                terminated(parse_coordinates, tag("\n")),
            )),
            |(first, second)| Brick([first, second]),
        )(i)
    }
}

fn parse_bricks(i: &[u8]) -> IResult<&[u8], Vec<Brick>> {
    many1(Brick::parse)(i)
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let mut bricks = parse_input(input, parse_bricks)?;
    bricks.sort_unstable_by_key(Brick::lowest);

    let mut width = 0;
    let mut breadth = 0;

    for brick in &bricks {
        width = Ord::max(width, Ord::max(brick.0[0][0], brick.0[1][0]));
        breadth = Ord::max(breadth, Ord::max(brick.0[0][2], brick.0[1][2]));
    }

    let width = width as usize + 1;
    let breadth = breadth as usize + 1;

    let mut height_map = vec![0; width * breadth];
    let mut top_map = vec![usize::MAX; height_map.len()];
    let mut supported = vec![0u8; bricks.len()];
    let mut supporting = vec![Vec::new(); bricks.len()];

    let mut temp = HashSet::new();

    for ((i, brick), supported) in bricks.iter().enumerate().zip(&mut supported) {
        let max_z = brick
            .iter_squares()
            .map(|(x, y)| height_map[x as usize + (y as usize) * width])
            .max()
            .expect("Guaranteed non-empty iterator.");

        let zs = minmax(brick.0[0][2], brick.0[1][2]);

        debug_assert!(
            zs.0 >= max_z,
            "Falling piece should be higher than existing tower"
        );

        let new_y = max_z + zs.1 - zs.0 + 1;

        for (x, y) in brick.iter_squares() {
            let offset = x as usize + (y as usize) * width;
            if height_map[offset] == max_z && max_z > 0 {
                debug_assert_ne!(top_map[offset], usize::MAX);
                temp.insert(top_map[offset]);
            }
            height_map[offset] = new_y;
            top_map[offset] = i;
        }

        *supported = temp.len().try_into()?;
        for support in temp.drain() {
            supporting[support].push(i);
        }
    }

    let removable = supporting
        .iter()
        .filter(|b| b.iter().all(|&other| supported[other] >= 2))
        .count();

    Ok(removable.to_string())
}

pub fn part2(_input: &[u8]) -> anyhow::Result<String> {
    anyhow::bail!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/22.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("5", part1(SAMPLE).unwrap());
    }
}
