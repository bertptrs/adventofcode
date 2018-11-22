use std::cmp;
use std::collections::{HashSet,VecDeque};
use std::io;
use std::io::prelude::*;
use permutohedron::Heap;
use common;

#[derive(Default)]
pub struct Day24 {
    map: Vec<Vec<char>>,
    positions: [(usize, usize);10],
    max_index: usize,
    adjacency: [[u32;10];10],
}

impl Day24 {
    pub fn new() -> Day24 {
        Default::default()
    }

    fn read_map(&mut self, input: &mut io::Read) {
        let reader = io::BufReader::new(input);
        self.map.clear();
        for (y, line) in reader.lines().enumerate() {
            let mut contents = Vec::new();

            for (x, c) in line.unwrap().chars().enumerate() {
                if let Some(val) = c.to_digit(10) {
                    self.max_index = cmp::max(self.max_index, val as usize);
                    self.positions[val as usize] = (x, y);
                }
                contents.push(c);
            }
            self.map.push(contents);
        }
        self.compute_adjacency();
    }

    fn compute_adjacency(&mut self) {
        for i in 0..(self.max_index + 1) {
            let mut visited = HashSet::new();
            let mut todo = VecDeque::new();
            visited.insert(self.positions[i]);
            todo.push_back((self.positions[i], 0));

            while !todo.is_empty() {
                let ((x, y), dist) = todo.pop_front().unwrap();
                if let Some(d) = self.map[y][x].to_digit(10) {
                    self.adjacency[i][d as usize] = dist;
                }

                for dx in -1..2 {
                    for dy in -1..2 {
                        if dx * dy != 0 {
                            continue;
                        }
                        let nx = (x as i32 + dx) as usize;
                        let ny = (y as i32 + dy) as usize;
                        let npos = (nx, ny);
                        if self.map[y][x] != '#' && !visited.contains(&npos) {
                            visited.insert(npos);
                            todo.push_back((npos, dist + 1));
                        }
                    }
                }
            }
        }
    }

    fn tour_length(&self, tour: &[usize]) -> u32 {
        let mut prev = 0;
        let mut dist = 0;
        for cur in tour {
            dist += self.adjacency[prev][*cur];
            prev = *cur;
        }
        dist
    }

    fn tour_length2(&self, tour: &[usize]) -> u32 {
        let dist = self.tour_length(tour);
        dist + self.adjacency[*tour.last().unwrap()][0]
    }
}

impl common::Solution for Day24 {

    fn part1(&mut self, input: &mut io::Read) -> String {
        self.read_map(input);
        let mut order: Vec<usize> = (1..(self.max_index + 1)).collect();
        let result = Heap::new(&mut order).map(|x| self.tour_length(&x)).min();
        format!("{}", result.unwrap())
    }

    fn part2(&mut self, input: &mut io::Read) -> String {
        self.read_map(input);
        let mut order: Vec<usize> = (1..(self.max_index + 1)).collect();
        let result = Heap::new(&mut order).map(|x| self.tour_length2(&x)).min();
        format!("{}", result.unwrap())
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use common::Solution;

    const SAMPLE_INPUT: &str = "
###########
#0.1.....2#
#.#######.#
#4.......3#
###########";

    #[test]
    fn sample_part1() {
        let mut instance = Day24::new();
        assert_eq!("14", instance.part1(&mut SAMPLE_INPUT.as_bytes()))
    }

}
