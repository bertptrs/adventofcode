use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;
use std::mem::swap;

use crate::common::Lines;
use crate::Solution;

type Pos = (i32, i32);

const DIRECTIONS: [Pos; 6] = [(-2, 0), (2, 0), (-1, -1), (-1, 1), (1, -1), (1, 1)];

fn convert_pos(s: &str) -> Pos {
    let mut s = s.chars();

    let mut x = 0;
    let mut y = 0;

    while let Some(c) = s.next() {
        match c {
            'e' => x += 2,
            'w' => x -= 2,
            'n' | 's' => {
                if c == 'n' {
                    y += 1;
                } else {
                    y -= 1;
                }

                match s.next() {
                    Some('e') => x += 1,
                    Some('w') => x -= 1,
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    (x, y)
}

fn step(current: &HashSet<Pos>, target: &mut HashSet<Pos>) {
    let mut black_count: HashMap<Pos, u32> = current.iter().map(|&pos| (pos, 0)).collect();

    for &(x, y) in current {
        for &(dx, dy) in &DIRECTIONS {
            let pos = (x + dx, y + dy);

            *black_count.entry(pos).or_default() += 1;
        }
    }

    target.clear();

    for (pos, neighbours) in black_count {
        let is_black = current.contains(&pos);

        let going_black = match neighbours {
            1 | 2 if is_black => true,
            2 if !is_black => true,
            _ => false,
        };

        if going_black {
            target.insert(pos);
        }
    }
}

fn get_black_tiles(input: &mut dyn Read) -> HashSet<Pos> {
    let mut black = HashSet::new();

    for line in Lines::new(input) {
        let pos = convert_pos(&line);

        if !black.insert(pos) {
            black.remove(&pos);
        }
    }

    black
}

#[derive(Default)]
pub struct Day24;

impl Solution for Day24 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        get_black_tiles(input).len().to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let mut state = get_black_tiles(input);
        let mut scratch_pad = state.clone();

        for _ in 0..100 {
            step(&state, &mut scratch_pad);
            swap(&mut state, &mut scratch_pad);
        }

        state.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../samples/24.txt");

    #[test]
    fn test_convert_pos() {
        assert_eq!((1, -1), convert_pos("esew"));
        assert_eq!((0, 0), convert_pos("nwwswee"));
    }

    #[test]
    fn test_step() {
        let state = get_black_tiles(&mut SAMPLE.clone());
        let mut target = state.clone();

        step(&state, &mut target);

        assert_eq!(15, target.len());
    }

    #[test]
    fn sample_part1() {
        test_implementation!(Day24, 1, SAMPLE, 10);
    }

    #[test]
    fn sample_part2() {
        test_implementation!(Day24, 2, SAMPLE, 2208);
    }
}
