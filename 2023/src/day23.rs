use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::Grid;
use crate::common::IndexSet;

#[derive(Clone, Copy, Debug)]
enum Slope {
    Level,
    Up,
    Down,
}

impl Slope {
    fn invert(self) -> Self {
        match self {
            Slope::Level => Self::Level,
            Slope::Up => Self::Down,
            Slope::Down => Self::Up,
        }
    }
    fn combine(self, other: Self) -> Self {
        match (self, other) {
            (Slope::Level, next) | (next, Slope::Level) => next,
            (Slope::Up, Slope::Up) => Slope::Up,
            (Slope::Up, Slope::Down) | (Slope::Down, Slope::Up) => {
                debug_assert!(false, "Inverted slope somehow");
                Slope::Level
            }
            (Slope::Down, Slope::Down) => Slope::Down,
        }
    }
}

fn simplify_graph(input: &[u8]) -> anyhow::Result<Vec<Vec<(Slope, usize, u32)>>> {
    let map = Grid::new(input)?;
    let width = map.width();
    let height = map.height();

    let is_junction = |x, y| {
        let mut entrance_count = 0;

        if map[(y, x + 1)] != b'#' {
            entrance_count += 1;
        }
        if map[(y, x - 1)] != b'#' {
            entrance_count += 1;
        }
        if map[(y - 1, x)] != b'#' {
            entrance_count += 1;
        }
        if map[(y + 1, x)] != b'#' {
            entrance_count += 1;
        }

        entrance_count >= 3
    };

    let mut visited = IndexSet::with_capacity(width * height);
    let mut graph = vec![vec![]; 3];
    visited.insert(1); // x = 1, y = 0;
    visited.insert(1 + width); // x = 1, y = 1
    visited.insert(width * height - 2);

    let mut nodes = HashMap::new();
    nodes.insert((1, 0), 0usize);
    nodes.insert((width - 2, height - 1), 1);
    nodes.insert((1, 1), 2);

    let mut todo_junctions = vec![(2, 1, 1)];

    let mut todo_positions = Vec::new();

    while let Some((id, start_x, start_y)) = todo_junctions.pop() {
        todo_positions.push((0, Slope::Level, start_x, start_y));

        while let Some((dist, slope, x, y)) = todo_positions.pop() {
            let mut enqueue = |x: usize, y: usize, up, down| {
                if map[(y, x)] == b'#' {
                } else if let Some(&other) = nodes.get(&(x, y)) {
                    if other == id {
                        return;
                    }

                    graph[id].push((slope, other, dist + 1));
                    graph[other].push((slope.invert(), id, dist + 1));
                } else if visited.insert(x + y * width) {
                    let tile_slope = if map[(y, x)] == up {
                        Slope::Up
                    } else if map[(y, x)] == down {
                        Slope::Down
                    } else {
                        Slope::Level
                    };
                    let resulting_slope = slope.combine(tile_slope);

                    if is_junction(x, y) {
                        let new_id = graph.len();
                        nodes.insert((x, y), new_id);
                        graph.push(Vec::new());
                        graph[id].push((slope, new_id, dist + 1));
                        graph[new_id].push((slope.invert(), id, dist + 1));

                        todo_junctions.push((new_id, x, y));
                    } else {
                        todo_positions.push((dist + 1, resulting_slope, x, y));
                    }
                }
            };
            enqueue(x - 1, y, b'>', b'<');
            enqueue(x + 1, y, b'<', b'>');
            enqueue(x, y - 1, b'v', b'^');
            enqueue(x, y + 1, b'^', b'v');
        }
    }

    Ok(graph)
}

fn longest_path(
    pos: usize,
    travelled: u32,
    graph: &[Vec<(Slope, usize, u32)>],
    visited: &mut HashSet<usize>,
) -> u32 {
    if pos == 1 {
        return travelled;
    }

    let mut best = 0;

    for &(slope, other, dist) in &graph[pos] {
        if !matches!(slope, Slope::Up) && visited.insert(other) {
            best = Ord::max(best, longest_path(other, travelled + dist, graph, visited));
            visited.remove(&other);
        }
    }

    best
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let graph = simplify_graph(input)?;

    Ok(longest_path(0, 0, &graph, &mut HashSet::new()).to_string())
}

fn longer_longest_path(
    pos: usize,
    travelled: u32,
    graph: &[Vec<(Slope, usize, u32)>],
    visited: &mut HashSet<usize>,
) -> u32 {
    if pos == 1 {
        return travelled;
    }

    let mut best = 0;

    for &(_, other, dist) in &graph[pos] {
        if visited.insert(other) {
            best = Ord::max(
                best,
                longer_longest_path(other, travelled + dist, graph, visited),
            );
            visited.remove(&other);
        }
    }

    best
}

pub fn part2(input: &[u8]) -> anyhow::Result<String> {
    let graph = simplify_graph(input)?;

    Ok(longer_longest_path(0, 0, &graph, &mut HashSet::new()).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/23.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("94", part1(SAMPLE).unwrap());
    }

    #[test]
    fn sample_part2() {
        assert_eq!("154", part2(SAMPLE).unwrap());
    }
}
