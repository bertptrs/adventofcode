use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use regex::Regex;

use common::Solution;

#[derive(Debug, Eq, PartialEq)]
struct Worker {
    time: usize,
    work: char,
}

impl Ord for Worker {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)
            .then(other.work.cmp(&self.work))
    }
}

impl PartialOrd for Worker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default, Debug)]
pub struct Day07 {
    forward: HashMap<char, Vec<char>>,
    dep_count: HashMap<char, usize>,
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

            self.forward.entry(a).or_insert(Vec::new()).push(b);
            *self.dep_count.entry(b).or_insert(0) += 1;
        }
    }

    fn part2_parametrized(&mut self, input: &mut Read, base_time: usize, max_workers: usize) -> usize {
        self.read_edges(input);
        let mut starting_points: BinaryHeap<Reverse<char>> = self.forward.keys()
            .filter(|&x| !self.dep_count.contains_key(x))
            .map(|&x| Reverse(x)).collect();

        let mut workers: BinaryHeap<Worker> = BinaryHeap::new();
        let mut time = 0;

        while !starting_points.is_empty() || !workers.is_empty() {
            while workers.len() < max_workers && !starting_points.is_empty() {
                let Reverse(to_start) = starting_points.pop().unwrap();
                workers.push(Worker {
                    time: time + base_time + ((to_start as u8) - b'A' + 1) as usize,
                    work: to_start,
                });
            }

            time = workers.peek().unwrap().time;

            while let Some(worker) = workers.pop() {
                if worker.time == time {
                    let c = worker.work;

                    if let Some(dependents) = self.forward.get(&c) {
                        for d in dependents {
                            let mut entry = self.dep_count.get_mut(d).unwrap();
                            if *entry == 1 {
                                starting_points.push(Reverse(*d));
                            } else {
                                *entry -= 1;
                            }
                        }
                    }
                } else {
                    workers.push(worker);
                    break;
                }
            }
        }

        time
    }
}

impl Solution for Day07 {
    fn part1(&mut self, input: &mut Read) -> String {
        self.read_edges(input);

        let mut result = String::new();

        let mut starting_points: BinaryHeap<_> = self.forward.keys().filter(|&x| !self.dep_count.contains_key(x))
            .map(|&x| Reverse(x)).collect();

        while let Some(Reverse(c)) = starting_points.pop() {
            result.push(c);

            if let Some(dependents) = self.forward.get(&c) {
                for d in dependents {
                    let mut entry = self.dep_count.get_mut(d).unwrap();
                    if *entry == 1 {
                        starting_points.push(Reverse(*d));
                    } else {
                        *entry -= 1;
                    }
                }
            }
        }

        result
    }

    fn part2(&mut self, input: &mut Read) -> String {
        format!("{}", self.part2_parametrized(input, 60, 5))
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

    #[test]
    pub fn sample_part2() {
        let mut instance = Day07::new();
        assert_eq!(15, instance.part2_parametrized(&mut SAMPLE_INPUT, 0, 2));
    }
}
