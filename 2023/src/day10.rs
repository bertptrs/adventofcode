use std::collections::VecDeque;

use anyhow::Context;

use crate::common::Grid;
use crate::common::IndexSet;

const LEFT: u8 = 1;
const RIGHT: u8 = 2;
const UP: u8 = 4;
const DOWN: u8 = 8;

fn get_connections(c: u8) -> u8 {
    match c {
        b'|' => UP | DOWN,
        b'-' => LEFT | RIGHT,
        b'F' => DOWN | RIGHT,
        b'J' => LEFT | UP,
        b'L' => RIGHT | UP,
        b'7' => LEFT | DOWN,
        _ => 0,
    }
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let map = Grid::new(input)?;
    let (start_x, start_y) = map.find(b'S').context("Couldn't find starting point")?;
    let mut visited = IndexSet::with_capacity(map.width() * map.height());
    let mut todo = VecDeque::new();

    visited.insert(start_y * map.width() + start_x);

    if start_x > 0 && (get_connections(map[start_y][start_x - 1]) & RIGHT) != 0 {
        todo.push_back((1, (start_x - 1, start_y)));
        visited.insert(start_y * map.width() + start_x - 1);
    }

    if start_x + 1 < map.width() && (get_connections(map[start_y][start_x + 1]) & LEFT) != 0 {
        todo.push_back((1, (start_x + 1, start_y)));
        visited.insert(start_y * map.width() + start_x + 1);
    }

    if start_y > 0 && (get_connections(map[start_y - 1][start_x]) & DOWN) != 0 {
        todo.push_back((1, (start_x, start_y - 1)));
        visited.insert((start_y - 1) * map.width() + start_x);
    }

    if start_y + 1 < map.height() && (get_connections(map[start_y + 1][start_x]) & DOWN) != 0 {
        todo.push_back((1, (start_x, start_y + 1)));
        visited.insert((start_y + 1) * map.width() + start_x);
    }

    let mut max_dist = 1;

    while let Some((dist, (x, y))) = todo.pop_front() {
        let mut enqueue = |x, y| {
            if visited.insert(y * map.width() + x) {
                todo.push_back((dist + 1, (x, y)));
                // Can elide comparison because we do a BFS and length is strictly increasing
                max_dist = dist + 1;
            }
        };

        let connections = get_connections(map[y][x]);

        if (connections & LEFT) != 0 {
            enqueue(x - 1, y);
        }

        if (connections & RIGHT) != 0 {
            enqueue(x + 1, y);
        }

        if (connections & UP) != 0 {
            enqueue(x, y - 1);
        }

        if (connections & DOWN) != 0 {
            enqueue(x, y + 1);
        }
    }

    Ok(max_dist.to_string())
}

pub fn part2(_input: &[u8]) -> anyhow::Result<String> {
    anyhow::bail!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/10.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("8", part1(SAMPLE).unwrap());
    }
}
