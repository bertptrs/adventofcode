use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;

use crate::common::Lines;
use crate::Solution;

struct Tile {
    grid: Vec<Vec<bool>>,
    id: u64,
    sides: [Vec<bool>; 4],
}

fn read_input(input: &mut dyn Read) -> HashMap<u64, Tile> {
    let mut tiles = HashMap::new();
    let mut lines = Lines::new(input);

    while let Some(line) = lines.next() {
        let id = line[5..(line.len() - 1)].parse().unwrap();
        drop(line);

        let mut grid: Vec<Vec<_>> = Vec::new();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            grid.push(line.chars().map(|c| c == '#').collect());
        }

        let sides = [
            grid[0].clone(),
            grid.iter().map(|v| *v.last().unwrap()).collect(),
            grid.last().cloned().unwrap(),
            grid.iter().map(|v| v[0]).collect(),
        ];

        let tile = Tile { grid, sides, id };

        tiles.insert(id, tile);
    }

    tiles
}

fn compute_matching<'a>(tiles: impl IntoIterator<Item = &'a Tile>) -> HashMap<Vec<bool>, Vec<u64>> {
    let mut matching: HashMap<Vec<_>, Vec<u64>> = HashMap::new();

    for tile in tiles {
        for side in &tile.sides {
            let neighbours = matching.entry(side.clone()).or_default();

            if !neighbours.contains(&tile.id) {
                neighbours.push(tile.id);
            }

            let mut rev = side.clone();
            rev.reverse();

            let neighbours = matching.entry(rev).or_default();

            if !neighbours.contains(&tile.id) {
                neighbours.push(tile.id);
            }
        }
    }

    // Check if my tile assumption is violated
    debug_assert!(matching.values().map(Vec::len).max().unwrap_or(0) <= 2);

    matching
}

fn compute_neighbours<'a>(
    cliques: impl IntoIterator<Item = &'a Vec<u64>>,
) -> HashMap<u64, Vec<u64>> {
    let mut neighbours: HashMap<u64, Vec<u64>> = HashMap::new();

    for clique in cliques {
        for &a in clique {
            let related = neighbours.entry(a).or_default();

            for &b in clique {
                if a != b && !related.contains(&b) {
                    related.push(b);
                }
            }
        }
    }

    neighbours
}

fn rev_eq(a: &[bool], b: &[bool]) -> bool {
    a.iter().zip(b.iter().rev()).all(|(&a, &b)| a == b)
}

#[derive(Default)]
pub struct Day20;

impl Solution for Day20 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let tiles = read_input(input);
        let matching = compute_matching(tiles.values());
        let neighbours = compute_neighbours(matching.values());

        neighbours
            .into_iter()
            .filter_map(|(i, n)| if n.len() == 2 { Some(i) } else { None })
            .fold(1, |a, b| a * b)
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../samples/20.txt");

    #[test]
    fn sample_part1() {
        test_implementation!(Day20, 1, SAMPLE, 20899048083289u64);
    }
}
