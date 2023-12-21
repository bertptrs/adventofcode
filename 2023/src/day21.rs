use std::collections::HashSet;
use std::collections::VecDeque;

use anyhow::Context;

use crate::common::Grid;

fn part1_parametrized(input: &[u8], steps: usize) -> anyhow::Result<String> {
    let map = Grid::new(input)?;
    let mut visited = HashSet::new();
    let mut todo = VecDeque::new();
    let start = map.find(b'S').context("Couldn't find starting point")?;
    todo.push_back((0, start));
    visited.insert(start);

    while let Some((dist, (x, y))) = todo.pop_front() {
        let mut enqueue = |x, y| {
            if map[(y, x)] != b'#' && visited.insert((x, y)) && dist < steps {
                todo.push_back((dist + 1, (x, y)));
            }
        };

        if x > 0 {
            enqueue(x - 1, y);
        }

        if y > 0 {
            enqueue(x, y - 1);
        }

        if x + 1 < map.width() {
            enqueue(x + 1, y);
        }

        if y + 1 < map.height() {
            enqueue(x, y + 1);
        }
    }

    Ok(visited
        .iter()
        .filter(|&&(x, y)| {
            let dist = start.0.abs_diff(x) + start.1.abs_diff(y);

            dist % 2 == steps % 2
        })
        .count()
        .to_string())
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    part1_parametrized(input, 64)
}

pub fn part2(_input: &[u8]) -> anyhow::Result<String> {
    anyhow::bail!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/21.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("16", part1_parametrized(SAMPLE, 6).unwrap());
    }
}
