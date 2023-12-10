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

fn find_cycle(map: &Grid<'_>) -> anyhow::Result<(usize, bool, IndexSet)> {
    let (start_x, start_y) = map.find(b'S').context("Couldn't find starting point")?;
    let mut visited = IndexSet::with_capacity(map.width() * map.height());
    let mut todo = VecDeque::new();

    visited.insert(start_y * map.width() + start_x);

    let mut start_up = false;

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
        start_up = true;
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

    Ok((max_dist, start_up, visited))
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let map = Grid::new(input)?;

    find_cycle(&map).map(|(max_dist, _, _)| max_dist.to_string())
}

pub fn part2(input: &[u8]) -> anyhow::Result<String> {
    let map = Grid::new(input)?;
    let (_, s_up, visited) = find_cycle(&map)?;

    let mut inside = 0;

    for (y, row) in map.rows().enumerate() {
        let y_offset = y * map.width();
        let mut pipes = 0;

        for (x, &c) in row.iter().enumerate() {
            if visited.contains(y_offset + x) {
                let is_up = match c {
                    b'|' | b'J' | b'L' => true,
                    b'S' => s_up,
                    _ => false,
                };

                if is_up {
                    pipes += 1;
                }
            } else {
                inside += pipes % 2;
            }
        }
    }

    Ok(inside.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/10.1.txt");
    const SAMPLE2: &[u8] = include_bytes!("samples/10.2.txt");
    const SAMPLE3: &[u8] = include_bytes!("samples/10.3.txt");
    const SAMPLE4: &[u8] = include_bytes!("samples/10.4.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("8", part1(SAMPLE).unwrap());
    }

    #[test]
    fn sample_part2() {
        assert_eq!("4", part2(SAMPLE2).unwrap());
        assert_eq!("8", part2(SAMPLE3).unwrap());
        assert_eq!("10", part2(SAMPLE4).unwrap());
    }
}
