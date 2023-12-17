use std::cmp;
use std::collections::hash_map::Entry;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use crate::common::Direction;
use crate::common::Grid;

#[derive(PartialEq, Eq)]
struct State {
    x: usize,
    y: usize,
    dir: Direction,
    heat_loss: u32,
    estimate: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // N.B. estimate and heat loss are compared in reverse for heap purposes, since BinaryHeap is a max heap.
        other
            .estimate
            .cmp(&self.estimate)
            .then_with(|| other.heat_loss.cmp(&self.heat_loss))
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
            .then_with(|| self.dir.cmp(&other.dir))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let map = Grid::new(input)?;
    let mut visited = HashMap::new();
    let width = map.width();
    let height = map.height();
    let mut todo = BinaryHeap::new();
    todo.push(State {
        x: 0,
        y: 0,
        dir: Direction::Right,
        heat_loss: 0,
        estimate: (width + height - 2) as u32,
    });
    todo.push(State {
        x: 0,
        y: 0,
        dir: Direction::Down,
        heat_loss: 0,
        estimate: (width + height - 2) as u32,
    });
    visited.insert((0usize, 0usize, true), 0u32);
    visited.insert((0usize, 0usize, false), 0u32);

    while let Some(State {
        x,
        y,
        dir,
        heat_loss,
        ..
    }) = todo.pop()
    {
        if x == map.width() - 1 && y == map.height() - 1 {
            return Ok(heat_loss.to_string());
        } else if visited[&(x, y, dir.is_vertical())] < heat_loss {
            continue;
        }

        let next_is_vertical = !dir.is_vertical();
        let mut new_loss = heat_loss;
        let mut enqueue = |x, y, heat_loss| {
            match visited.entry((x, y, next_is_vertical)) {
                Entry::Occupied(mut entry) => {
                    if entry.get() <= &heat_loss {
                        return;
                    } else {
                        entry.insert(heat_loss);
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(heat_loss);
                }
            }
            let estimate = (width + height - 2 - x - y) as u32 + heat_loss;

            if next_is_vertical {
                todo.push(State {
                    x,
                    y,
                    heat_loss,
                    dir: Direction::Up,
                    estimate,
                });
                todo.push(State {
                    x,
                    y,
                    heat_loss,
                    dir: Direction::Down,
                    estimate,
                });
            } else {
                todo.push(State {
                    x,
                    y,
                    heat_loss,
                    dir: Direction::Left,
                    estimate,
                });
                todo.push(State {
                    x,
                    y,
                    heat_loss,
                    dir: Direction::Right,
                    estimate,
                });
            }
        };

        match dir {
            Direction::Up => {
                for y in (0..y).rev().take(3) {
                    new_loss += u32::from(map[(y, x)] - b'0');
                    enqueue(x, y, new_loss);
                }
            }
            Direction::Left => {
                for x in (0..x).rev().take(3) {
                    new_loss += u32::from(map[(y, x)] - b'0');
                    enqueue(x, y, new_loss);
                }
            }
            Direction::Down => {
                for y in ((y + 1)..map.height()).take(3) {
                    new_loss += u32::from(map[(y, x)] - b'0');
                    enqueue(x, y, new_loss);
                }
            }
            Direction::Right => {
                for x in ((x + 1)..map.width()).take(3) {
                    new_loss += u32::from(map[(y, x)] - b'0');
                    enqueue(x, y, new_loss);
                }
            }
        }
    }
    anyhow::bail!("Did not find a solution")
}

pub fn part2(_input: &[u8]) -> anyhow::Result<String> {
    anyhow::bail!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/17.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("102", part1(SAMPLE).unwrap());
    }
}
