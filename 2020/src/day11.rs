use std::io::Read;
use std::mem::swap;

use crate::common::read_char_grid;
use crate::Solution;

fn neighbours(grid: &[Vec<u8>], r: usize, c: usize) -> usize {
    let mut n = 0;

    if r > 0 {
        let range = c.saturating_sub(1)..grid[r - 1].len().min(c + 2);
        n += grid[r - 1][range].iter().filter(|&&s| s == b'#').count();
    }

    if r < grid.len() - 1 {
        let range = c.saturating_sub(1)..grid[r + 1].len().min(c + 2);
        n += grid[r + 1][range].iter().filter(|&&s| s == b'#').count();
    }

    if c > 0 && grid[r][c - 1] == b'#' {
        n += 1;
    }

    if c < grid[r].len() - 1 && grid[r][c + 1] == b'#' {
        n += 1;
    }

    n
}

fn neighbours2(grid: &[Vec<u8>], r: usize, c: usize) -> usize {
    let mut n = 0;

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let r = r as isize;
    let c = c as isize;

    for &(dr, dc) in &directions {
        let mut nr = r + dr;
        let mut nc = c + dc;

        // Don't write conditions like this kids
        while nr >= 0
            && nr < (grid.len() as isize)
            && nc >= 0
            && nc < (grid[nr as usize].len() as isize)
        {
            match grid[nr as usize][nc as usize] {
                b'#' => {
                    n += 1;
                    break;
                }
                b'L' => break,
                _ => {
                    nr += dr;
                    nc += dc;
                }
            }
        }
    }

    n
}

#[allow(unused)]
fn print(grid: &[Vec<u8>]) {
    for line in grid {
        let s: String = line.iter().map(|&c| c as char).collect();
        println!("{}", s);
    }
}

#[derive(Default)]
pub struct Day11;

impl Solution for Day11 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let mut state = read_char_grid(input);
        let mut next = state.clone();

        loop {
            for (r, row) in state.iter().enumerate() {
                for (c, s) in row.iter().enumerate() {
                    let e = match *s {
                        b'#' if neighbours(&state, r, c) >= 4 => b'L',
                        b'L' if neighbours(&state, r, c) == 0 => b'#',
                        s => s,
                    };

                    next[r][c] = e;
                }
            }

            if state == next {
                return state
                    .iter()
                    .flatten()
                    .filter(|&&s| s == b'#')
                    .count()
                    .to_string();
            }

            swap(&mut state, &mut next);
        }
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let mut state = read_char_grid(input);
        let mut next = state.clone();

        loop {
            for (r, row) in state.iter().enumerate() {
                for (c, s) in row.iter().enumerate() {
                    let e = match *s {
                        b'#' if neighbours2(&state, r, c) >= 5 => b'L',
                        b'L' if neighbours2(&state, r, c) == 0 => b'#',
                        s => s,
                    };

                    next[r][c] = e;
                }
            }

            if state == next {
                return state
                    .iter()
                    .flatten()
                    .filter(|&&s| s == b'#')
                    .count()
                    .to_string();
            }

            swap(&mut state, &mut next);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../samples/11.txt");

    #[test]
    fn sample_part1() {
        test_implementation!(Day11, 1, SAMPLE, 37);
    }

    #[test]
    fn sample_part2() {
        test_implementation!(Day11, 2, SAMPLE, 26);
    }
}
