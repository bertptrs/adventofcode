use std::collections::{HashMap, HashSet};
use std::io::Read;

use regex::Regex;

use crate::common::Lines;
use crate::Solution;

fn read_graph(input: &mut dyn Read) -> HashMap<String, Vec<String>> {
    let regex = Regex::new(r"(\w+ \w+) bag").unwrap();

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in Lines::new(input) {
        let mut captures = regex.captures_iter(&line);

        let container = &captures.next().unwrap()[1];

        for cap in captures {
            let contained = cap[1].to_owned();
            graph
                .entry(contained)
                .or_default()
                .push(container.to_owned());
        }
    }

    graph
}

fn read_graph2(input: &mut dyn Read) -> HashMap<String, Vec<(usize, String)>> {
    let regex = Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();

    let mut graph: HashMap<String, Vec<(usize, String)>> = HashMap::new();

    for line in Lines::new(input) {
        let second_space = line
            .chars()
            .enumerate()
            .filter(|&(_, c)| ' ' == c)
            .nth(1)
            .unwrap()
            .0;

        let container = line[..second_space].to_owned();

        let contained = regex
            .captures_iter(&line)
            .map(|cap| (cap[1].parse().unwrap(), cap[2].to_owned()))
            .collect();

        graph.insert(container, contained);
    }

    graph
}

fn compute_contained(
    color: &str,
    graph: &HashMap<String, Vec<(usize, String)>>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if let Some(amount) = memo.get(color) {
        return *amount;
    }

    let total = if let Some(contained) = graph.get(color) {
        contained
            .iter()
            .map(|(count, color)| count * compute_contained(color, graph, memo))
            .sum::<usize>()
            + 1
    } else {
        1
    };

    memo.insert(color.to_owned(), total);

    total
}

#[derive(Default)]
pub struct Day07;

impl Solution for Day07 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let graph = read_graph(input);
        let mut todo = vec!["shiny gold"];
        let mut routes = 0;

        let mut done = HashSet::new();
        done.insert("shiny gold");

        while let Some(color) = todo.pop() {
            if let Some(neighbours) = graph.get(color) {
                for neighbour in neighbours {
                    if !done.contains(neighbour.as_str()) {
                        routes += 1;
                        done.insert(neighbour);
                        todo.push(neighbour);
                    }
                }
            }
        }

        routes.to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let graph = read_graph2(input);

        let mut count_memo = HashMap::with_capacity(graph.len());

        (compute_contained("shiny gold", &graph, &mut count_memo) - 1).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../samples/07.txt");
    const SAMPLE2: &[u8] = include_bytes!("../samples/07.2.txt");

    #[test]
    fn sample_part1() {
        test_implementation!(Day07, 1, SAMPLE, 4);
    }

    #[test]
    fn sample_part2() {
        test_implementation!(Day07, 2, SAMPLE, 32);
        test_implementation!(Day07, 2, SAMPLE2, 126);
    }
}
