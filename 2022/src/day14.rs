use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::multi::fold_many0;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;

use crate::common::parse_input;
use crate::common::reduce_many1;
use crate::common::IndexSet;

#[derive(Debug)]
struct Cave {
    width: usize,
    height: usize,
    occupied: IndexSet,
}

impl Cave {
    pub fn insert(&mut self, x: usize, y: usize) -> bool {
        // Note: we're indexing column major for better cache locality
        self.occupied.insert(self.height * x + y)
    }

    pub fn drop(&mut self, x: usize, y: usize, total: &mut usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        } else if !self.occupied.insert(x * self.height + y) {
            return true;
        }

        // x + usize::MAX is used to compute x - 1 because usize - isize doesn't exist in stable yet.
        let supported = [0, usize::MAX, 1]
            .into_iter()
            .all(|dx| self.drop(x.wrapping_add(dx), y + 1, total));

        if supported {
            *total += 1;
        }

        supported
    }
}

fn parse_pair(input: &[u8]) -> IResult<&[u8], (usize, usize)> {
    fn parse_usize(input: &[u8]) -> IResult<&[u8], usize> {
        map_res(nom::character::complete::u32, usize::try_from)(input)
    }

    separated_pair(parse_usize, tag(","), parse_usize)(input)
}

fn find_dimensions(input: &[u8]) -> IResult<&[u8], (usize, usize)> {
    fold_many0(
        terminated(parse_pair, alt((tag(" -> "), tag("\n")))), // Somehow this cant be `newline` because type deduction goes awry
        || (0usize, 0usize),
        |(width, height), (x, y)| (width.max(x + 1), height.max(y + 1)),
    )(input)
}

fn parse_cave(input: &[u8]) -> IResult<&[u8], Cave> {
    let (width, height) = find_dimensions(input)?.1;

    // Assume parsing went somewhat right
    debug_assert_ne!(width, 0);
    debug_assert_ne!(height, 0);

    let mut cave = Cave {
        width,
        height,
        occupied: IndexSet::with_capacity(width * height),
    };

    let mut input = input;

    while input != &[][..] {
        let new_input = terminated(
            reduce_many1(
                terminated(parse_pair, opt(tag(" -> "))),
                |(x_old, y_old), (x_prime, y_prime)| {
                    if x_prime == x_old {
                        for y in (y_old.min(y_prime))..=(y_old.max(y_prime)) {
                            cave.insert(x_old, y);
                        }
                    } else {
                        for x in (x_old.min(x_prime))..=(x_old.max(x_prime)) {
                            cave.insert(x, y_old);
                        }
                    }
                    (x_prime, y_prime)
                },
            ),
            newline,
        )(input)?
        .0;

        input = new_input;
    }

    Ok((input, cave))
}

pub fn part1(input: &[u8]) -> Result<String> {
    let mut cave = parse_input(input, parse_cave)?;

    let mut total = 0;

    cave.drop(500, 0, &mut total);

    Ok(total.to_string())
}

pub fn part2(_input: &[u8]) -> Result<String> {
    anyhow::bail!("not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/14.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "24");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "93")
    }
}
