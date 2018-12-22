use std::collections::HashSet;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use common::GroupingCount;
use common::Point;
use common::Solution;

type Coordinate = (usize, usize);

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
            self.points.push((x, y));
            mx = mx.max(x);
            my = my.max(y);
        }

        self.xmax = mx;
        self.ymax = my;
    }

    fn range(&self) -> impl Iterator<Item=Coordinate> {
        iproduct!(0..=self.xmax, 0..=self.ymax)
    }

    fn compute_claim_grid(&self) -> Vec<Vec<Claim>> {
        let mut grid = vec![vec![Claim::None; self.xmax + 1]; self.ymax + 1];

        for (x, y) in self.range() {
            let mut cur_dist = usize::max_value();
            let mut cur_best = None;

            for (i, point) in self.points.iter().enumerate() {
                let dist = point.manhattan((x, y));
                if dist < cur_dist {
                    cur_dist = dist;
                    cur_best = Some(i);
                } else if dist == cur_dist {
                    cur_best = None;
                }
            }

            grid[y][x] = match cur_best {
                Some(id) => Claim::Some(id),
                None => Claim::Multi,
            };
        }
        grid
    }

    pub fn part2_with_limit(&mut self, input: &mut Read, limit: usize) -> usize {
        self.read_points(input);

        self.range()
            .map(|x| self.points.iter().map(|y| y.manhattan(x)).sum::<usize>())
            .filter(|&x| x < limit)
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
        infinite.extend(grid.first().unwrap().iter().filter_map(claim_filter));
        infinite.extend(grid.last().unwrap().iter().filter_map(claim_filter));
        for row in grid.iter().take(self.ymax) {
            infinite.extend([row.first().unwrap(), row.last().unwrap()].iter()
                .cloned().filter_map(claim_filter));
        }

        let counts = grid.iter().flat_map(|x| x.iter())
            .filter_map(claim_filter)
            .filter(|x| !infinite.contains(x)).grouping_count();

        counts.values().max().unwrap().to_string()
    }

    fn part2(&mut self, input: &mut Read) -> String {
        self.part2_with_limit(input, 10_000).to_string()
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
