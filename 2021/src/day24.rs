//! Very input-specific reverse-engineered solution
//!
//! # General implementation
//!
//! The code in the examples is a series of 14 times this:
//!
//! ```txt
//! inp w       -> read digit
//! mul x 0
//! add x z
//! mod x 26    -> x = z % 26
//! div z $A    -> pop Z (see below)
//! add x $B
//! eql x w     -> x = ((z + $B) == w)
//! eql x 0     -> x = ((z + $B) != w)
//! mul y 0
//! add y 25
//! mul y x
//! add y 1     -> if x { 26 } else { 1 }
//! mul z y     -> if x { z *= 26 } (push z, see below)
//! mul y 0
//! add y w
//! add y $C    -> y = w + $C
//! mul y x
//! add z y     -> if x { z += w + $C }
//! ```
//!
//! `$A` is either `1` or `26` which we can translate to a bool `$A == 26` for convenience. This
//! simplifies to the following rust.
//!
//! ```
//! fn validate<const A: bool, const B: i32, const C: i32>(mut z: i32, digit: i32) -> i32 {
//!     let x = (z % 26 + B) != digit;
//!     if A {
//!         z /= 26;
//!     }
//!
//!     if x {
//!         z = 26 * z + digit + C;
//!     }
//!
//!     z
//! }
//! ```
//!
//! In human terms, `z` is used to hold a base 26 number. When `$A` is `true`, we pop off the least
//! significant digit by dividing by 26. Then, depending on whether `(z + $B) % 26` is equal to our
//! digit, we push `digit + $C`. Ideally, we should pop as often as we push in order to arrive at `z
//! == 0` in the end. The input contains 7 pops, so we want each of those to not push.
//!
//! To solve this problem, we use a backtracking memoizing algorithm, where we cancel every sequence
//! that fails to pop at some point. A pop is failed whenever we execute a validation pop where we
//! can pop if `x` happened to be set to `0` at the time of the check. We can memoize this over the
//! running value of `z`.
//!
//! This implementation probably doesn't work on other people's input; to fix it, you'll want to
//! update the `parse_step` function. Good luck with that.
use std::collections::HashMap;
use std::io::Read;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;

use crate::common::read_input;

type Cache = HashMap<(usize, i32), Option<i64>>;

#[derive(Debug)]
struct Step(bool, i32, i32);

impl Step {
    fn evaluate(&self, digit: i32, z: i32) -> Option<i32> {
        if self.0 {
            (z % 26 + self.1 == digit).then(|| z / 26)
        } else {
            Some(z * 26 + digit + self.2)
        }
    }
}

fn parse_step(input: &[u8]) -> IResult<&[u8], Step> {
    use nom::character::complete::i32;

    let parse_pop = preceded(
        tag("inp w\nmul x 0\nadd x z\nmod x 26\ndiv z "),
        alt((map(tag("1"), |_| false), map(tag("26"), |_| true))),
    );

    let parse_a = preceded(tag("\nadd x "), i32);

    let parse_b = delimited(
        tag("\neql x w\neql x 0\nmul y 0\nadd y 25\nmul y x\nadd y 1\nmul z y\nmul y 0\nadd y w\nadd y "),
        i32,
        tag("\nmul y x\nadd z y"),
    );

    map(tuple((parse_pop, parse_a, parse_b)), |(pop, a, b)| {
        Step(pop, a, b)
    })(input)
}

fn parse_input(input: &[u8]) -> IResult<&[u8], Vec<Step>> {
    separated_list1(newline, parse_step)(input)
}

fn optimize(
    current: usize,
    steps: &[Step],
    digits: &[i32],
    z: i32,
    cache: &mut Cache,
) -> Option<i64> {
    if current >= steps.len() {
        return (z == 0).then(|| 0);
    }

    if let Some(&memoized) = cache.get(&(current, z)) {
        return memoized;
    }

    let result = digits.iter().find_map(|&digit| {
        let z = steps[current].evaluate(digit, z)?;
        let result = optimize(current + 1, steps, digits, z, cache)?;

        Some(result + digit as i64 * 10i64.pow(13 - current as u32))
    });
    cache.insert((current, z), result);

    result
}

fn parts_common(input: &mut dyn Read, digits: &[i32]) -> String {
    let steps = read_input(input, parse_input);

    let mut cache = Cache::new();

    optimize(0, &steps, digits, 0, &mut cache)
        .unwrap()
        .to_string()
}

pub fn part1(input: &mut dyn Read) -> String {
    let digits = [9, 8, 7, 6, 5, 4, 3, 2, 1];

    parts_common(input, &digits)
}

pub fn part2(input: &mut dyn Read) -> String {
    let digits = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    parts_common(input, &digits)
}
