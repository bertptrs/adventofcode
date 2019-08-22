use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::mem::swap;

use common::GroupingCount;
use common::Solution;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum Tile {
    Tree,
    Lumber,
    Open,
    Invalid,
}

impl Tile {
    pub fn next(self, counts: &[usize; 4]) -> Self {
        match self {
            Tile::Open => {
                if counts[Tile::Tree as usize] >= 3 {
                    Tile::Tree
                } else {
                    Tile::Open
                }
            }
            Tile::Tree => {
                if counts[Tile::Lumber as usize] >= 3 {
                    Tile::Lumber
                } else {
                    Tile::Tree
                }
            }
            Tile::Lumber => {
                if counts[Tile::Tree as usize] >= 1 && counts[Tile::Lumber as usize] >= 1 {
                    Tile::Lumber
                } else {
                    Tile::Open
                }
            }
            Tile::Invalid => Tile::Invalid,
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '|' => Tile::Tree,
            '#' => Tile::Lumber,
            '.' => Tile::Open,
            _ => Tile::Invalid,
        }
    }
}

impl From<Tile> for char {
    fn from(t: Tile) -> Self {
        match t {
            Tile::Tree => '|',
            Tile::Lumber => '#',
            Tile::Open => '.',
            Tile::Invalid => 'X',
        }
    }
}

#[derive(Default)]
pub struct Day18 {
    grid: Vec<Vec<Tile>>,
    buf: Vec<Vec<Tile>>,
    width: usize,
}

impl Day18 {
    pub fn new() -> Self {
        Default::default()
    }

    fn read_input(&mut self, input: &mut dyn Read) {
        let reader = BufReader::new(input);
        self.grid.clear();

        for line in reader.lines() {
            let line = line.unwrap();

            self.grid.push(line.chars().map(From::from).collect());
        }

        self.width = self.grid.first().unwrap().len();
        self.buf = vec![vec![Tile::Tree; self.width]; self.grid.len()];
    }

    fn simulate(&mut self) {
        let height = self.grid.len();
        let width = self.width;

        for y in 0..height {
            let ymin = if y > 0 { y - 1 } else { y };
            let ymax = if y < height - 1 { y + 1 } else { y };

            for x in 0..self.width {
                let mut counts = [0; 4];
                let xmin = if x > 0 { x - 1 } else { x };
                let xmax = if x < width - 1 { x + 1 } else { x };

                for ys in ymin..=ymax {
                    for xs in xmin..=xmax {
                        if ys != y || xs != x {
                            counts[self.grid[ys][xs] as usize] += 1;
                        }
                    }
                }

                self.buf[y][x] = self.grid[y][x].next(&counts);
            }
        }

        swap(&mut self.buf, &mut self.grid);
    }

    fn print(&self) -> String {
        let mut buf = String::with_capacity(self.width * self.grid.len());
        for row in &self.grid {
            buf.extend(row.iter().cloned().map(Into::<char>::into));
        }
        buf
    }

    fn score(&self) -> String {
        let result = self
            .grid
            .iter()
            .flat_map(|x| x.iter())
            .cloned()
            .grouping_count();

        (result[&Tile::Tree] * result[&Tile::Lumber]).to_string()
    }
}

impl Solution for Day18 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        self.read_input(input);

        for _ in 0..10 {
            self.simulate();
        }

        self.score()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        self.read_input(input);
        let limit = 1_000_000_000;

        let mut seen = HashMap::new();
        seen.insert(self.print(), 0);

        for i in 1..=limit {
            self.simulate();
            let summary = self.print();
            if let Some(first) = seen.get(&summary) {
                let period = i - *first;
                let remaining = (limit - *first) % period;

                for _ in 0..remaining {
                    self.simulate();
                }

                return self.score();
            }

            seen.insert(summary, i);
        }

        // I encourage everyone to hit this line of code.
        self.score()
    }
}

#[cfg(test)]
mod tests {
    use common::Solution;
    use day18::Day18;

    const SAMPLE_INPUT: &[u8] = include_bytes!("samples/18.txt");

    #[test]
    fn sample_part1() {
        let mut instance = Day18::new();
        assert_eq!("1147", instance.part1(&mut SAMPLE_INPUT));
    }
}
