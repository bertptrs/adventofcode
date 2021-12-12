use std::collections::HashMap;
use std::io::Read;

use crate::common::LineIter;

type EdgeMap = HashMap<String, Vec<String>>;

fn read_edges(input: &mut dyn Read) -> EdgeMap {
    let mut reader = LineIter::new(input);
    let mut edges = EdgeMap::new();

    while let Some(line) = reader.next() {
        let (from, to) = line.split_once('-').unwrap();

        edges
            .entry(from.to_owned())
            .or_default()
            .push(to.to_owned());

        edges
            .entry(to.to_owned())
            .or_default()
            .push(from.to_owned());
    }

    edges
}

fn is_small(cave: &str) -> bool {
    cave.chars().all(|c| c.is_ascii_lowercase())
}

fn dfs_routes<'a>(
    edges: &'a EdgeMap,
    route: &'_ mut Vec<&'a str>,
    pos: &'a str,
    mut small_twice: bool,
) -> usize {
    match pos {
        "end" => return 1,
        "start" if !route.is_empty() => return 0,
        pos if is_small(pos) && route.contains(&pos) => {
            if small_twice {
                small_twice = false;
            } else {
                return 0;
            }
        }
        _ => (),
    }

    route.push(pos);

    let routes = edges[pos]
        .iter()
        .map(|new_pos| dfs_routes(edges, route, new_pos, small_twice))
        .sum();

    route.pop();

    routes
}

fn parts_common(input: &mut dyn Read, small_twice: bool) -> String {
    let edges = read_edges(input);
    let mut route = Vec::new();

    dfs_routes(&edges, &mut route, "start", small_twice).to_string()
}

pub fn part1(input: &mut dyn Read) -> String {
    parts_common(input, false)
}

pub fn part2(input: &mut dyn Read) -> String {
    parts_common(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_implementation;

    const SAMPLE1: &[u8] = include_bytes!("samples/12.1.txt");
    const SAMPLE2: &[u8] = include_bytes!("samples/12.2.txt");
    const SAMPLE3: &[u8] = include_bytes!("samples/12.3.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE1, 10);
        test_implementation(part1, SAMPLE2, 19);
        test_implementation(part1, SAMPLE3, 226);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE1, 36);
        test_implementation(part2, SAMPLE2, 103);
        test_implementation(part2, SAMPLE3, 3509);
    }
}
