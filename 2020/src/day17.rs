use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;
use std::mem::swap;

use crate::common::Lines;
use crate::Solution;

type Space = HashSet<(i32, i32, i32)>;
type Space4 = HashSet<(i32, i32, i32, i32)>;

fn read_input(input: &mut dyn Read) -> Space {
    let mut space = HashSet::new();

    for (y, line) in Lines::new(input).enumerate() {
        space.extend(line.chars().enumerate().filter_map(|(x, c)| {
            if c == '#' {
                Some((x as i32, y as i32, 0))
            } else {
                None
            }
        }))
    }

    space
}

fn advance(current: &Space, next: &mut Space) {
    let mut live_count = HashMap::new();

    for &(x, y, z) in current {
        for nx in (x - 1)..=(x + 1) {
            for ny in (y - 1)..=(y + 1) {
                for nz in (z - 1)..=(z + 1) {
                    if x != nx || y != ny || nz != z {
                        *live_count.entry((nx, ny, nz)).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    next.clear();

    for (pos, neighbours) in live_count {
        if (neighbours == 2 && current.contains(&pos)) || neighbours == 3 {
            next.insert(pos);
        }
    }
}

fn read_input2(input: &mut dyn Read) -> Space4 {
    let mut space = HashSet::new();

    for (y, line) in Lines::new(input).enumerate() {
        space.extend(line.chars().enumerate().filter_map(|(x, c)| {
            if c == '#' {
                Some((0, x as i32, y as i32, 0))
            } else {
                None
            }
        }))
    }

    space
}

fn advance2(current: &Space4, next: &mut Space4) {
    let mut live_count = HashMap::new();

    for &(w, x, y, z) in current {
        for nw in (w - 1)..=(w + 1) {
            for nx in (x - 1)..=(x + 1) {
                for ny in (y - 1)..=(y + 1) {
                    for nz in (z - 1)..=(z + 1) {
                        if nw != w || x != nx || y != ny || nz != z {
                            *live_count.entry((nw, nx, ny, nz)).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }

    next.clear();

    for (pos, neighbours) in live_count {
        if (neighbours == 2 && current.contains(&pos)) || neighbours == 3 {
            next.insert(pos);
        }
    }
}

#[derive(Default)]
pub struct Day17;

impl Solution for Day17 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let mut space = read_input(input);
        let mut scratch_pad = Space::new();

        for _ in 0..6 {
            advance(&space, &mut scratch_pad);
            swap(&mut space, &mut scratch_pad);
        }

        space.len().to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let mut space = read_input2(input);
        let mut scratch_pad = Space4::new();

        for _ in 0..6 {
            advance2(&space, &mut scratch_pad);
            swap(&mut space, &mut scratch_pad);
        }

        space.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../samples/17.txt");

    #[test]
    fn sample_part1() {
        test_implementation!(Day17, 1, SAMPLE, 112);
    }

    #[test]
    fn sample_part2() {
        test_implementation!(Day17, 2, SAMPLE, 848);
    }
}
