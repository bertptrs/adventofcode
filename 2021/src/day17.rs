use std::io::Read;
use std::ops::RangeInclusive;

use itertools::Itertools;
use itertools::MinMaxResult;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::IResult;

use crate::common::read_input;

#[inline]
fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<f64> {
    let d = b * b - 4. * a * c;

    if d < 0. {
        None
    } else {
        // Don't care about the smaller solution due to problem statement
        Some((-b - d.sqrt()) / 2. / a)
    }
}

fn position(initial: i32, time: i32) -> i32 {
    time * (2 * initial - time + 1) / 2
}

fn find_hit(initial: i32, range: &RangeInclusive<i32>) -> Option<RangeInclusive<i32>> {
    // y position at time x: f(x) = x * (1 + initial + initial - x) / 2
    //                            = -1/2x^2 + (initial + 0.5)x
    //
    // to hit, find x := (max(box) + min(box)) / 2 = f(x)
    //                                             = -1/2x^2 + (initial + 0.5)x
    // -1/2x^2 + (initial + 0.5)x - (max(box) + min(box)) / 2 = 0
    let middle = (*range.start() + *range.end()) as f64 / 2.;
    let b = initial as f64 + 0.5;
    let hit = solve_quadratic(-0.5, b, -middle)? as i32;

    if hit < 0 {
        // Should not happen because of the shape but for correctness
        None
    } else {
        let min_hit = (0..=hit)
            .rev()
            .take_while(|&n| range.contains(&position(initial, n)))
            .min();

        let max_hit = ((hit + 1)..)
            .take_while(|&n| range.contains(&position(initial, n)))
            .max();

        match (min_hit, max_hit) {
            (Some(min), Some(max)) => Some(min..=max),
            (Some(val), None) | (None, Some(val)) => Some(val..=val),
            _ => None,
        }
    }
}

fn find_speed(x: i32, range: &RangeInclusive<i32>) -> Option<RangeInclusive<i32>> {
    let mut min = 0;
    let mut max = *range.end();

    // Need to tweak the formula as x slows down
    let x_pos = |speed| position(speed, speed.min(x));

    while max >= min {
        let speed = (max + min) / 2;

        let pos = x_pos(speed);

        if range.contains(&x_pos(speed)) {
            let min_speed = (0..speed)
                .rev()
                .take_while(|&speed| range.contains(&x_pos(speed)))
                .min()
                .unwrap_or(speed);

            let max_speed = ((speed + 1)..)
                .take_while(|&speed| range.contains(&x_pos(speed)))
                .max()
                .unwrap_or(speed);
            return Some(min_speed..=max_speed);
        } else if pos < *range.start() {
            min = speed + 1;
        } else {
            max = speed - 1;
        }
    }

    None
}

fn parse_range(input: &[u8]) -> IResult<&[u8], RangeInclusive<i32>> {
    use nom::character::complete::i32;

    map(separated_pair(i32, tag(".."), i32), |(start, end)| {
        start..=end
    })(input)
}

fn parse_input(input: &[u8]) -> IResult<&[u8], (RangeInclusive<i32>, RangeInclusive<i32>)> {
    preceded(
        tag("target area: x="),
        separated_pair(parse_range, tag(", y="), parse_range),
    )(input)
}

pub fn part1(input: &mut dyn Read) -> String {
    let (x_range, y_range) = read_input(input, parse_input);

    let check_value = |y_speed| {
        let mut time = find_hit(y_speed, &y_range)?;

        if time.any(|time| find_speed(time, &x_range).is_some()) {
            Some(position(y_speed, y_speed))
        } else {
            None
        }
    };

    (0..1000).filter_map(check_value).max().unwrap().to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let (x_range, y_range) = read_input(input, parse_input);

    let num_options = |y_speed| {
        let time = find_hit(y_speed, &y_range)?;
        let range = time
            .filter_map(|time| find_speed(time, &x_range))
            .flat_map(|x| [*x.start(), *x.end()])
            .minmax();

        Some(match range {
            MinMaxResult::NoElements => 0,
            MinMaxResult::OneElement(_) => 1,
            MinMaxResult::MinMax(min, max) => max - min + 1,
        })
    };

    (-1000..1000)
        .filter_map(num_options)
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = &*b"target area: x=20..30, y=-10..-5";

    #[test]
    fn test_find_hit() {
        assert_eq!(find_hit(2, &(-10..=-5)), Some(7..=7));
        assert_eq!(find_hit(3, &(-10..=-5)), Some(9..=9));
        assert_eq!(find_hit(0, &(-10..=-5)), Some(4..=5));
    }

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 45);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 112);
    }
}
