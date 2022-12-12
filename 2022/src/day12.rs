use std::collections::VecDeque;

use anyhow::Context;
use anyhow::Result;

use crate::common::IndexSet;

fn can_travel(from: u8, to: u8) -> bool {
    match (from, to) {
        (b'S', b'a'..=b'z') => true,
        (b'y'..=b'z', b'E') => true,
        (b'a'..=b'z', b'a'..=b'z') => to <= from || to - from == 1,
        _ => false,
    }
}

pub fn part1(input: &[u8]) -> Result<String> {
    let width = input
        .iter()
        .position(|&c| c == b'\n')
        .context("No newlines in input")?
        + 1;

    let starting_pos = input
        .iter()
        .position(|&c| c == b'S')
        .context("Could not find starting position")?;

    let mut visited = IndexSet::with_capacity(input.len());

    let mut todo = VecDeque::new();
    todo.push_back((0, starting_pos));

    while let Some((dist, pos)) = todo.pop_front() {
        if input[pos] == b'E' {
            return Ok(dist.to_string());
        }

        let mut add_todo = |new: usize| {
            if can_travel(input[pos], input[new]) && visited.insert(new) {
                todo.push_back((dist + 1, new));
            }
        };

        if pos % width != 0 {
            add_todo(pos - 1);
        }

        if pos % width != width - 1 {
            add_todo(pos + 1)
        }

        if pos >= width {
            add_todo(pos - width);
        }

        if pos + width < input.len() {
            add_todo(pos + width);
        }
    }

    anyhow::bail!("Did not find a valid route")
}

pub fn part2(input: &[u8]) -> Result<String> {
    let width = input
        .iter()
        .position(|&c| c == b'\n')
        .context("No newlines in input")?
        + 1;

    let starting_pos = input
        .iter()
        .position(|&c| c == b'E')
        .context("Could not find starting position")?;

    let mut visited = IndexSet::with_capacity(input.len());

    let mut todo = VecDeque::new();
    todo.push_back((0, starting_pos));

    while let Some((dist, pos)) = todo.pop_front() {
        if input[pos] == b'a' || input[pos] == b'S' {
            return Ok(dist.to_string());
        }

        let mut add_todo = |new: usize| {
            if can_travel(input[new], input[pos]) && visited.insert(new) {
                todo.push_back((dist + 1, new));
            }
        };

        if pos % width != 0 {
            add_todo(pos - 1);
        }

        if pos % width != width - 1 {
            add_todo(pos + 1);
        }

        if pos >= width {
            add_todo(pos - width);
        }

        if pos + width < input.len() {
            add_todo(pos + width);
        }
    }

    anyhow::bail!("Did not find a valid route")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/12.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "31")
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "29")
    }
}
