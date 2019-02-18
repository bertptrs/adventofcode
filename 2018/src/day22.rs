use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use common::Point;
use common::Solution;

type Coordinate = (usize, usize);

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone, Debug)]
struct State {
    pos: Coordinate,
    climbing: bool,
    torch: bool,
}

impl State {
    pub fn is_valid(&self, terrain: usize) -> bool {
        match terrain {
            0 => self.torch || self.climbing,
            1 => !self.torch,
            2 => !self.climbing,
            _ => panic!(),
        }
    }
}

const MOD_BASE: usize = 20183;

#[derive(Default)]
pub struct Day22 {}

impl Day22 {
    pub fn new() -> Self {
        Default::default()
    }
}

fn compute_table((x, y): Coordinate, depth: usize) -> Vec<Vec<usize>> {
    let mut table = vec![vec![0usize; x + 1]; y + 1];
    table[0][0] = 0;
    for x in 1..=x {
        table[0][x] = (16807 * x + depth) % MOD_BASE;
    }

    for y in 1..=y {
        table[y][0] = (48271 * y + depth) % MOD_BASE;
    }

    for y in 1..=y {
        for x in 1..=x {
            table[y][x] = (table[y - 1][x] * table[y][x - 1] + depth) % MOD_BASE;
        }
    }

    for c in table.iter_mut().flat_map(|x| x.iter_mut()) {
        *c %= 3;
    }

    table
}

fn read_input(input: &mut Read) -> (usize, Coordinate) {
    let mut buf = String::new();
    let mut reader = BufReader::new(input);
    reader.read_line(&mut buf).unwrap();

    let depth: usize;
    {
        let mut parts = buf.trim().split(' ');
        depth = parts.nth(1).unwrap().parse().unwrap();
    }
    buf.clear();
    reader.read_line(&mut buf).unwrap();

    let target: Coordinate;
    {
        let mut parts = buf.trim().split(|c| c == ',' || c == ' ');
        let x: usize = parts.nth(1).unwrap().parse().unwrap();
        let y: usize = parts.next().unwrap().parse().unwrap();
        target = (x, y);
    }

    (depth, target)
}

impl Solution for Day22 {
    fn part1(&mut self, input: &mut Read) -> String {
        let (depth, target) = read_input(input);
        let mut table = compute_table(target, depth);
        table[target.1][target.0] = 0;

        let result: usize = table.iter().flat_map(|x| x.iter()).sum();
        result.to_string()
    }

    fn part2(&mut self, input: &mut Read) -> String {
        let (depth, target) = read_input(input);
        let mut table = compute_table((target.0 + 200, target.1 + 200), depth);
        table[target.1][target.0] = 0;

        let mut todo = BinaryHeap::new();
        let mut visited: HashSet<State> = HashSet::new();
        let target_state = State {
            pos: target,
            climbing: false,
            torch: true,
        };

        todo.push((
            Reverse((0, 0).manhattan(target)),
            Reverse(0),
            State {
                pos: (0, 0),
                climbing: false,
                torch: true,
            },
        ));

        while let Some((Reverse(approx), Reverse(dist), state)) = todo.pop() {
            if visited.contains(&state) {
                continue;
            }
            visited.insert(state);
            if state == target_state {
                return dist.to_string();
            }

            let (x, y) = state.pos;

            // Handle equipment changes
            let changes = [
                State {
                    pos: state.pos,
                    climbing: state.climbing,
                    torch: !state.torch,
                },
                State {
                    pos: state.pos,
                    climbing: !state.climbing,
                    torch: state.torch,
                },
                State {
                    pos: state.pos,
                    climbing: !state.climbing,
                    torch: !state.torch,
                },
            ];

            for state in changes.iter().cloned() {
                if visited.contains(&state) || !state.is_valid(table[y][x]) {
                    continue;
                }

                todo.push((Reverse(approx + 7), Reverse(dist + 7), state));
            }

            let xmin = if x == 0 { 0 } else { x - 1 };
            let ymin = if y == 0 { 0 } else { y - 1 };

            for xn in xmin..=(x + 1) {
                for yn in ymin..=(y + 1) {
                    let new_state = State {
                        pos: (xn, yn),
                        torch: state.torch,
                        climbing: state.climbing,
                    };

                    if !visited.contains(&new_state)
                        && new_state.is_valid(table[yn][xn])
                        && (x == xn || y == yn)
                    {
                        todo.push((
                            Reverse(dist + 1 + target.manhattan(new_state.pos)),
                            Reverse(dist + 1),
                            new_state,
                        ));
                    }
                }
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use common::Solution;

    use super::*;

    const SAMPLE_INPUT: &[u8] = include_bytes!("samples/22.txt");

    #[test]
    fn sample_part1() {
        let mut instance = Day22::new();
        assert_eq!("114", instance.part1(&mut SAMPLE_INPUT));
    }

    #[test]
    fn sample_part2() {
        let mut instance = Day22::new();
        assert_eq!("45", instance.part2(&mut SAMPLE_INPUT));
    }
}
