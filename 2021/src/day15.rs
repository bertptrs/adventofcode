use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::Read;

use crate::common::LineIter;

type Point = (i32, i32);

struct Map {
    width: usize,
    data: Vec<u8>,
}

impl Map {
    pub fn from_input(input: &mut dyn Read) -> Self {
        let mut reader = LineIter::new(input);

        let mut data = reader.next().unwrap().as_bytes().to_owned();
        let width = data.len();

        while let Some(line) = reader.next() {
            let line = line.as_bytes();
            debug_assert_eq!(line.len(), width);

            data.extend_from_slice(line);
        }

        data.iter_mut().for_each(|b| *b -= b'0');

        Self { width, data }
    }

    pub fn from_input2(input: &mut dyn Read) -> Self {
        let mut reader = LineIter::new(input);

        let mut lines = Vec::new();

        while let Some(line) = reader.next() {
            let mut line = line.as_bytes().to_owned();
            line.iter_mut().for_each(|b| *b -= b'0');
            lines.push(line);
        }

        let mut data = Vec::new();
        let width = lines[0].len();

        for _y_repeat in 0..5 {
            for line in &mut lines {
                data.extend_from_slice(line);

                for _ in 0..4 {
                    let starting_pos = data.len() - width;
                    data.extend_from_within(starting_pos..);
                    let starting_pos = data.len() - width;
                    data[starting_pos..]
                        .iter_mut()
                        .for_each(|b| *b = (*b % 9) + 1);
                }

                line.iter_mut().for_each(|b| *b = (*b % 9) + 1);
            }
        }

        Self {
            width: width * 5,
            data,
        }
    }

    pub fn shortest_path(&self, start: Point, end: Point) -> u32 {
        let mut todo = BinaryHeap::new();
        todo.push(Reverse((Self::manhattan(start, end), 0, start)));

        let mut visited = vec![false; self.data.len()];

        let height = self.height() as i32;

        while let Some(Reverse((_, distance, pos))) = todo.pop() {
            if pos == end {
                return distance;
            }

            if visited[self.index(pos)] {
                continue;
            }

            visited[self.index(pos)] = true;

            let (x, y) = pos;

            for dy in -1..=1 {
                if y + dy < 0 || y + dy >= height {
                    continue;
                }

                for dx in -1..=1 {
                    if x + dx < 0 || (x + dx) >= self.width as i32 || dx * dy != 0 {
                        continue;
                    }

                    let new = (x + dx, y + dy);
                    let index = self.index(new);

                    if visited[index] {
                        continue;
                    }

                    let new_distance = distance + self.data[index] as u32;
                    let new_guess = Self::manhattan(new, end) + new_distance;

                    todo.push(Reverse((new_guess, new_distance, new)));
                }
            }
        }

        panic!("No route found from {:?} to {:?}", start, end);
    }

    fn manhattan((xa, ya): Point, (xb, yb): Point) -> u32 {
        (xa - xb).abs() as u32 + (ya - yb).abs() as u32
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }

    pub fn index(&self, (x, y): Point) -> usize {
        y as usize * self.width + x as usize
    }
}

pub fn part1(input: &mut dyn Read) -> String {
    let map = Map::from_input(input);

    map.shortest_path((0, 0), (map.width as i32 - 1, map.height() as i32 - 1))
        .to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let map = Map::from_input2(input);

    map.shortest_path((0, 0), (map.width as i32 - 1, map.height() as i32 - 1))
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_implementation;

    const SAMPLE: &[u8] = include_bytes!("samples/15.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 40);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 315);
    }
}
