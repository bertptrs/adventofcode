use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use common::Solution;

#[derive(Copy, Clone, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    pub fn manhattan(&self, other: &Coordinate) -> usize {
        self.x.max(other.x) + self.y.max(other.y)
            - self.x.min(other.x) - self.y.min(other.y)
    }
}

impl From<(usize, usize)> for Coordinate {
    fn from((x, y): (usize, usize)) -> Self {
        Coordinate {
            x,
            y,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Claim {
    None,
    Some(usize),
    Multi,
}

#[derive(Default, Debug)]
pub struct Day06 {
    points: Vec<Coordinate>,
    xmax: usize,
    ymax: usize,
}

impl Day06 {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn read_points(&mut self, input: &mut Read) {
        let reader = BufReader::new(input);
        self.points.clear();

        let mut mx = 0;
        let mut my = 0;

        for line in reader.lines() {
            let line = line.unwrap();
            let mut parts = line.split(", ");
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            self.points.push(Coordinate { x, y });
            mx = mx.max(x);
            my = my.max(y);
        }

        self.xmax = mx;
        self.ymax = my;
    }

    fn compute_claim_grid(&self) -> Vec<Vec<Claim>> {
        let mut grid = vec![vec![Claim::None; self.xmax + 1]; self.ymax + 1];

        for coordinate in iproduct!(0..=self.xmax, 0..=self.ymax) {
            let mut cur_dist = usize::max_value();
            let mut cur_best = None;

            let coordinate = Coordinate::from(coordinate);

            for (i, point) in self.points.iter().enumerate() {
                let dist = point.manhattan(&coordinate);
                if dist < cur_dist {
                    cur_dist = dist;
                    cur_best = Some(i);
                } else if dist == cur_dist {
                    cur_best = None;
                }
            }

            grid[coordinate.y][coordinate.x] = match cur_best {
                Some(id) => Claim::Some(id),
                None => Claim::Multi,
            };
        }
        grid
    }

    pub fn part2_with_limit(&mut self, input: &mut Read, limit: usize) -> usize {
        self.read_points(input);

        iproduct!(0..=self.xmax, 0..=self.ymax)
            .map(|x| Coordinate::from(x))
            .map(|x| self.points.iter().map(|y| y.manhattan(&x)).sum::<usize>())
            .filter(|x| x < &limit)
            .count()
    }
}

fn claim_filter(claim: &Claim) -> Option<usize> {
    match claim {
        Claim::Some(id) => Some(*id),
        _ => None,
    }
}

impl Solution for Day06 {
    fn part1(&mut self, input: &mut Read) -> String {
        self.read_points(input);
        let grid = self.compute_claim_grid();
        let mut infinite: HashSet<usize> = HashSet::new();
        infinite.extend(grid[0].iter().filter_map(claim_filter));
        infinite.extend(grid[self.ymax].iter().filter_map(claim_filter));
        for y in 0..=self.ymax {
            infinite.extend([grid[y][0], grid[y][self.xmax]].iter().filter_map(claim_filter));
        }

        let mut counts = HashMap::new();

        for instance in grid.iter().flat_map(|x| x.iter())
            .filter_map(claim_filter)
            .filter(|x| !infinite.contains(x)) {
            *counts.entry(instance).or_insert(0) += 1;
        }

        format!("{}", counts.values().max().unwrap())
    }

    fn part2(&mut self, input: &mut Read) -> String {
        format!("{}", self.part2_with_limit(input, 10_000))
    }
}

#[cfg(test)]
mod tests {
    use common::Solution;
    use day06::Day06;

    const SAMPLE_INPUT: &[u8] = include_bytes!("samples/06.txt");

    #[test]
    fn sample_part1() {
        let mut instance = Day06::new();
        assert_eq!("17", instance.part1(&mut SAMPLE_INPUT));
    }

    #[test]
    fn sample_part2() {
        let mut instance = Day06::new();
        assert_eq!(16, instance.part2_with_limit(&mut SAMPLE_INPUT, 32));
    }
}
