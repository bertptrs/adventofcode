use std::io::Read;
use std::ops::RangeInclusive;

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
        if a > 0. {
            Some((-b + d.sqrt()) / 2. / a)
        } else {
            Some((-b - d.sqrt()) / 2. / a)
        }
    }
}

fn position(initial: i32, time: i32) -> i32 {
    time * (2 * initial - time + 1) / 2
}

fn find_hit(initial: i32, range: &RangeInclusive<i32>) -> impl Iterator<Item = i32> + '_ {
    // y position at time x: f(x) = x * (1 + initial + initial - x) / 2
    //                            = -1/2x^2 + (initial + 0.5)x
    //
    // to hit, find x := (max(box) + min(box)) / 2 = f(x)
    //                                             = -1/2x^2 + (initial + 0.5)x
    // -1/2x^2 + (initial + 0.5)x - (max(box) + min(box)) / 2 = 0
    let middle = (*range.start() + *range.end()) as f64 / 2.;
    let b = initial as f64 + 0.5;
    let hit = if let Some(hit) = solve_quadratic(-0.5, b, -middle) {
        hit as i32
    } else {
        -1
    };

    (0..=hit)
        .rev()
        .take_while(move |&n| range.contains(&position(initial, n)))
        .chain(((hit + 1)..).take_while(move |&n| range.contains(&position(initial, n))))
}

fn find_speed(x: i32, range: &RangeInclusive<i32>) -> Option<(i32, i32)> {
    if *range.end() <= position(x, x) {
        // Can and should come to a full stop
        let max = solve_quadratic(0.5, 0.5, -*range.end() as f64)? as i32;

        let min = (0..=max)
            .rev()
            .take_while(|&n| range.contains(&position(n, n)))
            .last()?;

        Some((min, max))
    } else {
        // Might hit the target at speed
        let max = (x * x + 2 * *range.end() - x) / (2 * x);

        let min = (0..=max)
            .rev()
            .take_while(|&n| range.contains(&position(n, n.min(x))))
            .last()?;

        Some((min, max))
    }
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

    let check_value =
        |y_speed| find_hit(y_speed, &y_range).any(|time| find_speed(time, &x_range).is_some());

    debug_assert!(*y_range.start() < 0);
    let y_max = -*y_range.start();

    (0..y_max)
        .rev()
        .find(|&speed| check_value(speed))
        .map(|speed| position(speed, speed))
        .unwrap()
        .to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let (x_range, y_range) = read_input(input, parse_input);

    let num_options = |y_speed| {
        find_hit(y_speed, &y_range)
            .filter_map(|time| find_speed(time, &x_range))
            .reduce(|(a_min, a_max), (b_min, b_max)| (a_min.min(b_min), a_max.max(b_max)))
            .map(|(min, max)| max - min + 1)
            .unwrap_or(0)
    };

    debug_assert!(*y_range.start() < 0);
    let y_max = -*y_range.start();

    (-y_max..y_max).map(num_options).sum::<i32>().to_string()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = &*b"target area: x=20..30, y=-10..-5";

    #[test]
    fn test_find_hit() {
        assert_eq!(find_hit(2, &(-10..=-5)).collect::<Vec<_>>(), vec![7]);
        assert_eq!(find_hit(3, &(-10..=-5)).collect::<Vec<_>>(), vec![9]);
        assert_eq!(find_hit(0, &(-10..=-5)).collect::<Vec<_>>(), vec![4, 5]);
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
