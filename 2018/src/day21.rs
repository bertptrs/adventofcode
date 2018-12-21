use std::collections::HashSet;
use std::io::Read;

use common::Solution;

#[derive(Default)]
pub struct Day21 {
}

struct ValidInputs {
    f: i64
}

impl ValidInputs {
    pub fn new(start: i64) -> Self {
        ValidInputs {
            f: start
        }
    }
}

impl Iterator for ValidInputs {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        let mut f = self.f;
        let mut e = f | 0x10000;
        f = 13284195;

        loop {
            let d = e & 0xff;
            f += d;
            f &= 0xffffff;
            f *= 65899;
            f &= 0xffffff;

            if 0x100 > e {
                self.f = f;
                return Some(f);
            }

            e >>= 8;
        }
    }
}

impl Day21 {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Solution for Day21 {
    fn part1(&mut self, _input: &mut Read) -> String {
        format!("{}", ValidInputs::new(0).next().unwrap())
    }

    fn part2(&mut self, _input: &mut Read) -> String {
        let inputs = ValidInputs::new(0);
        let mut seen = HashSet::new();
        let mut last = None;

        for input in inputs {
            if seen.contains(&input) {
                return format!("{}", last.unwrap());
            } else {
                last = Some(input);
                seen.insert(input);
            }
        }
        unreachable!();
    }
}
