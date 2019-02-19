use std::io::Read;

use common::Solution;
use std::io::BufReader;
use regex::Regex;
use std::io::BufRead;

type Coordinate = (usize, usize);

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
struct Node {
    pos: Coordinate,
    capacity: usize,
    used: usize,
}

impl Node {
    fn fits(self, other: Node) -> bool {
        self.capacity - self.used >= other.used
    }
}

#[derive(Default)]
pub struct Day22 {
    nodes: Vec<Node>,
}

impl Day22 {
    pub fn new() -> Self {
        Default::default()
    }

    fn read_input(&mut self, input: &mut Read) {
        let reader = BufReader::new(input);
        let matcher = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)(?:\s+)(\d+)T(?:\s+)+(\d+)T").unwrap();

        for line in reader.lines() {
            let line = line.unwrap();
            let node = match matcher.captures(&line) {
                Some(captures) => Node {
                    pos: (captures[1].parse().unwrap(), captures[2].parse().unwrap()),
                    capacity: captures[3].parse().unwrap(),
                    used: captures[4].parse().unwrap(),
                },

                None => continue
            };

            self.nodes.push(node)
        }

        assert_ne!(0, self.nodes.len())
    }
}

impl Solution for Day22 {
    fn part1(&mut self, input: &mut Read) -> String {
        self.read_input(input);

        let fitting = iproduct!(&self.nodes, &self.nodes)
            .filter(|&(_, b)| b.used > 0)
            .filter(|&(a, b)| a != b)
            .filter(|&(a, b)| a.fits(*b))
            .count();


        fitting.to_string()
    }
}