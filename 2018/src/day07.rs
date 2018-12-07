use std::cmp::Reverse;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use regex::Regex;

use common::Solution;
use std::collections::HashSet;
use std::collections::BinaryHeap;

fn append_edge(target: &mut HashMap<char, Vec<char>>, source: char, dest: char) {
    target.entry(source).or_insert(Vec::new()).push(dest);
}

#[derive(Default, Debug)]
pub struct Day07 {
    forward: HashMap<char, Vec<char>>,
    backward: HashMap<char, Vec<char>>,
}

impl Day07 {
    pub fn new() -> Self {
        Default::default()
    }

    fn read_edges(&mut self, input: &mut Read) {
        let reader = BufReader::new(input);
        let regex = Regex::new(r"Step (\w) must be finished before step (\w) can begin").unwrap();

        for line in reader.lines() {
            let line = line.unwrap();
            let groups = regex.captures(&line).unwrap();
            let a = groups[1].chars().next().unwrap();
            let b = groups[2].chars().next().unwrap();

            append_edge(&mut self.forward, a, b);
            append_edge(&mut self.backward, b, a);
        }
    }
}

impl Solution for Day07 {
    fn part1(&mut self, input: &mut Read) -> String {
        self.read_edges(input);

        let mut result = String::new();
        let mut finished = HashSet::new();

        let mut starting_points: BinaryHeap<Reverse<char>> = self.forward.keys().filter(|&x| !self.backward.contains_key(x))
            .map(|&x| Reverse(x)).collect();

        while let Some(Reverse(c)) = starting_points.pop() {
            result.push(c);
            finished.insert(c);

            if let Some(dependents) = self.forward.get(&c) {
                for d in dependents {
                    if self.backward.get(d).unwrap().iter().all(|x| finished.contains(x)) {
                        starting_points.push(Reverse(*d));
                    }
                }
            }
        }

        result
    }

    fn part2(&mut self, input: &mut Read) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use common::Solution;
    use day07::Day07;

    const SAMPLE_INPUT: &[u8] = include_bytes!("samples/07.txt");

    #[test]
    pub fn sample_part1() {
        let mut instance = Day07::new();
        assert_eq!("CABDFE", instance.part1(&mut SAMPLE_INPUT));
    }
}
