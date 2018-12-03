use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::ops::Range;

use common;
use regex;

#[derive(Copy, Clone, Debug)]
struct Claim {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Claim {

    pub fn xrange(&self) -> Range<usize> {
        self.x..(self.x + self.width)
    }

    pub fn yrange(&self) -> Range<usize> {
        self.y..(self.y + self.height)
    }
}

#[derive(Default)]
pub struct Day03 {
    claims: Vec<Claim>
}


impl Day03 {
    pub fn new() -> Day03 {
        Default::default()
    }

    fn read_claims(&mut self, input: &mut io::Read) {
        let reader = io::BufReader::new(input);
        self.claims.clear();

        let matcher = regex::Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

        for line in reader.lines() {
            let line = line.unwrap();
            let matched = matcher.captures(&line).unwrap();

            let claim = Claim{
                x: matched.get(2).unwrap().as_str().parse().unwrap(),
                y: matched.get(3).unwrap().as_str().parse().unwrap(),
                width: matched.get(4).unwrap().as_str().parse().unwrap(),
                height: matched.get(5).unwrap().as_str().parse().unwrap(),
            };
            self.claims.push(claim);
        }
    }
}


impl common::Solution for Day03 {
    fn part1(&mut self, input: &mut io::Read) -> String {
        self.read_claims(input);
        let mut claim_map = HashMap::new();

        for claim in &self.claims {
            for x in claim.xrange() {
                for y in claim.yrange() {
                    *claim_map.entry((x, y)).or_insert(0) += 1;
                }
            }
        }

        let multi_claim = claim_map.values()
            .filter(|&&x| x > 1)
            .count();

        format!("{}", multi_claim)
    }

    fn part2(&mut self, input: &mut io::Read) -> String {
        self.read_claims(input);
        let mut claim_map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
        let mut overlaps: Vec<HashSet<usize>> = Vec::new();
        overlaps.resize(self.claims.len(), HashSet::new());

        for (idx, claim) in self.claims.iter().enumerate() {
            for x in claim.xrange() {
                for y in claim.yrange() {
                    let entry = claim_map.entry((x, y)).or_insert(Vec::new());
                    for claim in entry.iter() {
                        overlaps[*claim].insert(idx);
                        overlaps[idx].insert(*claim);
                    }

                    &entry.push(idx);
                }
            }
        }

        let uncontested = overlaps.iter().position(|x| x.is_empty()).unwrap();
        format!("{}", uncontested + 1)
    }
}

#[cfg(test)]
mod tests {
    use common::Solution;

    use super::*;

    const SAMPLE_INPUT: &[u8] = b"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    #[test]
    fn sample_part1() {
        let mut instance = Day03::new();
        let result = instance.part1(&mut SAMPLE_INPUT);
        assert_eq!("4", result);
    }

    #[test]
    fn sample_part2() {
        let mut instance = Day03::new();
        let result = instance.part2(&mut SAMPLE_INPUT);
        assert_eq!("3", result);
    }
}
