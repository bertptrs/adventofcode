use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use itertools::Itertools;
use regex::Regex;

use common::Solution;

type Coordinate = (usize, usize);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Default)]
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

fn get_grid<T>(list: &[T], size: usize, (x, y): Coordinate) -> &T {
    &list[x + size * y]
}

fn get_mut_grid<T>(list: &mut [T], size: usize, (x, y): Coordinate) -> &mut T {
    &mut list[x + size * y]
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
struct State {
    pos: Coordinate,
    empty_pos: Coordinate,
    grid: Vec<usize>,
}

impl State {
    fn estimate(&self) -> usize {
        self.pos.0 + self.pos.1
            + self.pos.0.max(self.empty_pos.0) + self.pos.1.max(self.empty_pos.1)
            - self.pos.0.min(self.empty_pos.0) - self.pos.1.min(self.empty_pos.1)
            - 1
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

        self.nodes.sort_unstable();

        assert_ne!(0, self.nodes.len())
    }

    fn build_grid(&self) -> (usize, Vec<usize>, Vec<usize>) {
        let size = self.nodes.iter().map(|node| node.pos.0).max().unwrap_or(0) + 1;

        let mut used = vec![0usize; self.nodes.len()];
        let mut capacity = used.clone();

        for node in &self.nodes {
            let pos = node.pos;
            *get_mut_grid(&mut used, size, pos) = node.used;
            *get_mut_grid(&mut capacity, size, pos) = node.capacity;
        }

        (size, used, capacity)
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

    fn part2(&mut self, input: &mut Read) -> String {
        self.read_input(input);

        let (size, used, capacity) = self.build_grid();

        let (empty_pos, _) = used.iter()
            .find_position(|&&used| used == 0)
            .unwrap();

        let height = used.len() / size;

        let state = State {
            pos: (size - 1, 0),
            empty_pos: (empty_pos % size, empty_pos / size),
            grid: used,
        };


        let mut visited = HashSet::new();
        visited.insert((state.pos, state.empty_pos));

        let mut todo = BinaryHeap::new();
        todo.push(Reverse((state.estimate(), 0usize, state)));

        while let Some(Reverse((_, dist, state))) = todo.pop() {
            if state.pos == (0, 0) {
                return dist.to_string();
            }
            let (xe, ye) = state.empty_pos;
            let empty_capacity = *get_grid(&capacity, size, state.empty_pos);

            let valid_x = [xe.checked_sub(1), Some(xe), Some(xe), if xe < size - 1 { Some(xe + 1) } else { None }];
            let valid_y = [Some(ye), ye.checked_sub(1), if ye < height - 1 { Some(ye + 1) } else { None }, Some(ye)];
            for (x, y) in valid_x.iter().zip(valid_y.iter()) {
                if x.is_none() || y.is_none() {
                    continue;
                }
                let switch = (x.unwrap(), y.unwrap());
                let contents = *get_grid(&state.grid, size, switch);

                if contents > empty_capacity {
                    // Not enough capacity
                    continue;
                }

                let mut new_state = State {
                    grid: state.grid.clone(),
                    empty_pos: switch,
                    pos: if switch == state.pos { state.empty_pos } else { state.pos },
                };

                if !visited.contains(&(new_state.pos, new_state.empty_pos)) {
                    visited.insert((new_state.pos, new_state.empty_pos));
                    *get_mut_grid(&mut new_state.grid, size, switch) = 0;
                    *get_mut_grid(&mut new_state.grid, size, state.empty_pos) = contents;

                    todo.push(Reverse((dist + 1 + new_state.estimate(), dist + 1, new_state)));
                }
            }
        }

        unreachable!("Did not arrive at an end state")
    }
}

#[cfg(test)]
mod tests {
    use common::Solution;
    use day22::Day22;

    const SAMPLE_INPUT: &[u8] = include_bytes!("samples/22.txt");

    #[test]
    fn sample_part2() {
        let mut instance = Day22::new();
        let result = instance.part2(&mut SAMPLE_INPUT);
        assert_eq!("7", result);
    }
}
