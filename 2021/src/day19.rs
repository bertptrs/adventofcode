use std::collections::HashSet;
use std::io::Read;
use std::ops::Add;
use std::ops::Sub;

use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use crate::common::read_input;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
struct Point3(pub [i32; 3]);

impl Point3 {
    pub fn manhattan(&self) -> i32 {
        self.0.into_iter().map(i32::abs).sum()
    }
}

impl Sub for Point3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point3([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl Add for Point3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point3([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

struct Rotations<'a> {
    points: &'a [Point3],
    axes: [usize; 3],
    rotation_index: usize,
}

impl<'a> Rotations<'a> {
    const ROTATIONS: [[i32; 3]; 8] = [
        [1, 1, 1],
        [1, 1, -1],
        [1, -1, 1],
        [1, -1, -1],
        [-1, 1, 1],
        [-1, 1, -1],
        [-1, -1, 1],
        [-1, -1, -1],
    ];

    pub fn new(points: &'a [Point3]) -> Self {
        Self {
            points,
            axes: [0, 1, 2],
            rotation_index: 0,
        }
    }
}

impl Iterator for Rotations<'_> {
    type Item = Vec<Point3>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rotation_index >= Self::ROTATIONS.len() {
            if !next_permutation(&mut self.axes) {
                return None;
            }

            self.rotation_index = 0;
        }

        let axes = &self.axes;
        let rot = &Self::ROTATIONS[self.rotation_index];

        let result = self
            .points
            .iter()
            .map(|Point3(coords)| {
                Point3([
                    coords[axes[0]] * rot[0],
                    coords[axes[1]] * rot[1],
                    coords[axes[2]] * rot[2],
                ])
            })
            .collect();

        self.rotation_index += 1;

        Some(result)
    }
}

fn parse_point(input: &[u8]) -> IResult<&[u8], Point3> {
    use nom::character::complete::char;
    use nom::character::complete::i32;

    map(
        tuple((i32, preceded(char(','), i32), preceded(char(','), i32))),
        |(x, y, z)| Point3([x, y, z]),
    )(input)
}

fn parse_input(input: &[u8]) -> IResult<&[u8], Vec<Vec<Point3>>> {
    use nom::character::complete::i32;
    let parse_header = delimited(tag("--- scanner "), i32, tag(" ---\n"));

    let parse_scanner = preceded(parse_header, many1(terminated(parse_point, newline)));
    separated_list1(newline, parse_scanner)(input)
}

fn try_overlap(
    correct: &[(Point3, HashSet<Point3>)],
    candidate: &[Point3],
) -> Option<(Point3, Vec<Point3>)> {
    let mut relative = HashSet::new();
    for rot in Rotations::new(candidate) {
        for &start in &rot {
            relative.clear();

            relative.extend(rot.iter().map(|&other| other - start));

            if let Some((base, _)) = correct.iter().find(|(_, correct_relative)| {
                correct_relative.intersection(&relative).count() >= 12
            }) {
                // Found a solution, build the correct output
                let translated = relative.drain().map(|point| point + *base).collect();

                return Some((start - *base, translated));
            }
        }
    }

    None
}

fn parts_common(input: &mut dyn Read) -> (HashSet<Point3>, Vec<Point3>) {
    let mut scanners = read_input(input, parse_input);

    let mut points: HashSet<_> = scanners[0].iter().copied().collect();

    let mut todo = vec![std::mem::take(&mut scanners[0])];
    let mut scanners_found = vec![Point3::default()];

    while let Some(matched) = todo.pop() {
        if scanners.iter().all(Vec::is_empty) {
            break;
        }

        let relative: Vec<(Point3, HashSet<Point3>)> = matched
            .iter()
            .map(|&base| (base, matched.iter().map(|&other| (other - base)).collect()))
            .collect();

        for candidate in &mut scanners {
            if candidate.is_empty() {
                continue;
            }

            if let Some((scanner, result)) = try_overlap(&relative, candidate) {
                scanners_found.push(scanner);
                points.extend(result.iter().copied());
                todo.push(result);
                candidate.clear();
            }
        }
    }

    (points, scanners_found)
}

pub fn part1(input: &mut dyn Read) -> String {
    let (points, _) = parts_common(input);
    points.len().to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let (_, scanners) = parts_common(input);

    scanners
        .iter()
        .flat_map(|&first| {
            scanners
                .iter()
                .map(move |&second| (first - second).manhattan())
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn next_permutation<T: Ord>(list: &mut [T]) -> bool {
    // Based on: https://en.cppreference.com/w/cpp/algorithm/next_permutation
    if list.len() <= 1 {
        return false;
    }

    if let Some((i, val1)) = list
        .windows(2)
        .enumerate()
        .rev()
        .find_map(|(i, window)| (window[0] < window[1]).then(|| (i, &window[0])))
    {
        let it2 = list
            .iter()
            .enumerate()
            .skip(i + 1)
            .rev()
            .find_map(|(idx, val2)| (val1 < val2).then(|| idx))
            .expect("Unreachable, ascending pair exists");

        list.swap(i, it2);
        list[(i + 1)..].reverse();
        true
    } else {
        list.reverse();
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_implementation;

    const SAMPLE: &[u8] = include_bytes!("samples/19.txt");

    #[test]
    fn test_next_permutation() {
        let mut list = [1, 2, 3];

        assert!(next_permutation(&mut list));
        assert_eq!(list, [1, 3, 2]);

        assert!(next_permutation(&mut list));
        assert_eq!(list, [2, 1, 3]);

        assert!(next_permutation(&mut list));
        assert_eq!(list, [2, 3, 1]);

        assert!(next_permutation(&mut list));
        assert_eq!(list, [3, 1, 2]);

        assert!(next_permutation(&mut list));
        assert_eq!(list, [3, 2, 1]);

        // Note the negation!
        assert!(!next_permutation(&mut list));
        assert_eq!(list, [1, 2, 3]);

        // 24 is a bit too much to write out, but we can check the number
        let mut list2 = [1, 2, 3, 4];
        for _ in 1..24 {
            assert!(next_permutation(&mut list2));
        }

        // Should be back to the start
        assert!(!next_permutation(&mut list2));
        assert_eq!(list2, [1, 2, 3, 4]);
    }

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 79);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 3621);
    }
}
