use std::io::Read;

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

type Point3 = [i32; 3];

#[derive(Debug, Eq, PartialEq)]
struct Cuboid {
    lower: Point3,
    upper: Point3,
}

impl Cuboid {
    pub fn new(lower: Point3, upper: Point3) -> Self {
        // The input uses an inclusive range for representation, but an exclusive one simplifies a
        // lot of computations, so we convert here.
        Self::new_internal(lower, upper.map(|c| c + 1))
    }

    fn new_internal(lower: Point3, upper: Point3) -> Self {
        debug_assert!(lower.iter().zip(&upper).all(|(a, b)| a < b));
        Self { lower, upper }
    }

    pub fn is_small(&self) -> bool {
        self.lower
            .iter()
            .chain(&self.upper.map(|c| c - 1)) // begrudgingly convert back
            .all(|c| c.abs() <= 50)
    }

    pub fn volume(&self) -> i64 {
        self.lower
            .iter()
            .zip(&self.upper)
            .map(|(&l, &h)| (h - l) as i64)
            .product()
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.lower
            .iter()
            .zip(&self.upper)
            .zip(other.lower.iter().zip(&other.upper))
            .all(|((&al, &ah), (&bl, &bh))| al < bh && bl < ah)
    }

    pub fn retain_nonoverlapping(&self, other: &Self, new_cubes: &mut Vec<Self>) -> bool {
        if !self.overlaps(other) {
            // Cube can be kept as-is.
            return true;
        }

        if other.lower[0] > self.lower[0] {
            // We have a set of X-coordinates below the overlap, add a cube for that
            new_cubes.push(Self::new_internal(
                self.lower,
                [other.lower[0], self.upper[1], self.upper[2]],
            ));
        }

        if other.upper[0] < self.upper[0] {
            // Similarly, we can remove an upper section.
            new_cubes.push(Self::new_internal(
                [other.upper[0], self.lower[1], self.lower[2]],
                self.upper,
            ));
        }

        // Now we can fix the X-coordinates of the overlap section
        let overlap_xl = self.lower[0].max(other.lower[0]);
        let overlap_xh = self.upper[0].min(other.upper[0]);

        // Same strategy for the Y axis
        if other.lower[1] > self.lower[1] {
            new_cubes.push(Self::new_internal(
                [overlap_xl, self.lower[1], self.lower[2]],
                [overlap_xh, other.lower[1], self.upper[2]],
            ))
        }
        if other.upper[1] < self.upper[1] {
            new_cubes.push(Self::new_internal(
                [overlap_xl, other.upper[1], self.lower[2]],
                [overlap_xh, self.upper[1], self.upper[2]],
            ))
        }

        let overlap_yl = self.lower[1].max(other.lower[1]);
        let overlap_yh = self.upper[1].min(other.upper[1]);

        // Finally, handle the last axis
        if other.lower[2] > self.lower[2] {
            new_cubes.push(Self::new_internal(
                [overlap_xl, overlap_yl, self.lower[2]],
                [overlap_xh, overlap_yh, other.lower[2]],
            ))
        }
        if other.upper[2] < self.upper[2] {
            new_cubes.push(Self::new_internal(
                [overlap_xl, overlap_yl, other.upper[2]],
                [overlap_xh, overlap_yh, self.upper[2]],
            ))
        }

        false
    }
}

fn parse_tuple(input: &[u8]) -> IResult<&[u8], (i32, i32)> {
    use nom::character::complete::i32;
    separated_pair(i32, tag(".."), i32)(input)
}

fn parse_cuboid(input: &[u8]) -> IResult<&[u8], Cuboid> {
    map(
        tuple((
            parse_tuple,
            preceded(tag(",y="), parse_tuple),
            preceded(tag(",z="), parse_tuple),
        )),
        |((xl, xh), (yl, yh), (zl, zh))| Cuboid::new([xl, yl, zl], [xh, yh, zh]),
    )(input)
}

fn parse_input(input: &[u8]) -> IResult<&[u8], Vec<(bool, Cuboid)>> {
    let parse_state = alt((map(tag("on x="), |_| true), map(tag("off x="), |_| false)));
    let parse_line = tuple((parse_state, parse_cuboid));

    separated_list1(newline, parse_line)(input)
}

pub fn part1(input: &mut dyn Read) -> String {
    let commands = read_input(input, parse_input);
    let mut cubes = Vec::new();
    let mut new_cubes = Vec::new();

    for (state, cube) in commands.into_iter().filter(|(_, c)| c.is_small()) {
        cubes.retain(|existing: &Cuboid| existing.retain_nonoverlapping(&cube, &mut new_cubes));

        // Add new cubes to the end
        cubes.append(&mut new_cubes);

        if state {
            cubes.push(cube);
        }
    }

    cubes.iter().map(Cuboid::volume).sum::<i64>().to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let commands = read_input(input, parse_input);
    let mut cubes = Vec::new();
    let mut new_cubes = Vec::new();

    for (state, cube) in commands {
        cubes.retain(|existing: &Cuboid| existing.retain_nonoverlapping(&cube, &mut new_cubes));

        // Add new cubes to the end
        cubes.append(&mut new_cubes);

        if state {
            cubes.push(cube);
        }
    }

    cubes.iter().map(Cuboid::volume).sum::<i64>().to_string()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE1: &[u8] = include_bytes!("samples/22.1.txt");
    const SAMPLE2: &[u8] = include_bytes!("samples/22.2.txt");

    #[test]
    fn test_overlap() {
        let cube_a = Cuboid {
            lower: [1, 1, 1],
            upper: [4, 4, 4],
        };

        let cube_b = Cuboid {
            lower: [2, 2, 2],
            upper: [3, 3, 3],
        };

        let mut new_cubes = Vec::new();

        // B is fully inside A so it should overlap and the result should be empty
        assert!(!cube_b.retain_nonoverlapping(&cube_a, &mut new_cubes));
        assert_eq!(new_cubes, Vec::new());

        // In the reverse case, we should have lots of new cubes
        assert!(!cube_a.retain_nonoverlapping(&cube_b, &mut new_cubes));
        assert_eq!(new_cubes.len(), 6);
    }

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE1, 590784);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE2, 2758514936282235u64);
    }
}
