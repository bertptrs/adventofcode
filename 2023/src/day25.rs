use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::bytes::complete::take_while1;
use nom::combinator::opt;
use nom::multi::fold_many1;
use nom::sequence::terminated;
use nom::IResult;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::common::minmax;
use crate::common::parse_input;

type Node<'a> = &'a [u8];
type Graph<'a> = HashMap<Node<'a>, HashSet<Node<'a>>>;

fn parse_graph(mut i: &[u8]) -> IResult<&[u8], Graph> {
    fn parse_line<'a>(i: &'a [u8], graph: &mut Graph<'a>) -> IResult<&'a [u8], ()> {
        let (i, name) = terminated(take_until(":"), tag(": "))(i)?;

        terminated(
            fold_many1(
                terminated(take_while1(|c: u8| c.is_ascii_alphabetic()), opt(tag(" "))),
                || (),
                move |_, other| {
                    graph.entry(name).or_default().insert(other);
                    graph.entry(other).or_default().insert(name);
                },
            ),
            tag("\n"),
        )(i)
    }
    let mut graph = HashMap::new();

    while !i.is_empty() {
        let (remain, _) = parse_line(i, &mut graph)?;
        i = remain;
    }

    Ok((i, graph))
}

fn find_path<'a>(
    s: &'a [u8],
    t: &'a [u8],
    graph: &Graph<'a>,
    counts: &mut HashMap<(&'a [u8], &'a [u8]), usize>,
) {
    let mut todo = VecDeque::new();
    let mut prev = HashMap::new();
    todo.push_back(s);
    prev.insert(s, s);

    'outer: while let Some(node) = todo.pop_front() {
        for &other in graph[node].iter() {
            if let Entry::Vacant(entry) = prev.entry(other) {
                entry.insert(node);
                if other == t {
                    break 'outer;
                } else {
                    todo.push_back(other);
                }
            }
        }
    }

    let mut cur = t;
    while let Some(ancestor) = prev.get(cur).copied() {
        if cur == ancestor {
            return;
        }
        let (a, b) = minmax(ancestor, cur);
        *counts.entry((a, b)).or_default() += 1;
        cur = ancestor;
    }
    unreachable!("Should not get here");
}

fn component_size(s: &[u8], graph: &Graph) -> usize {
    let mut visited = HashSet::new();
    visited.insert(s);

    let mut todo = Vec::new();
    todo.push(s);

    while let Some(node) = todo.pop() {
        todo.extend(graph[node].iter().filter(|&&v| visited.insert(v)));
    }

    visited.len()
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let mut graph = parse_input(input, parse_graph)?;
    let nodes: Vec<_> = graph.keys().copied().collect();
    let mut rng = thread_rng();
    let mut counts = HashMap::new();

    for _ in 0..1000 {
        let mut it = nodes.choose_multiple(&mut rng, 2);
        let first = *it.next().unwrap();
        let second = *it.next().unwrap();

        find_path(first, second, &graph, &mut counts);
    }

    let mut weighted_edges: Vec<_> = counts.into_iter().collect();
    weighted_edges.sort_unstable_by_key(|c| Reverse(c.1));

    let copy = graph.clone();

    for &((a, b), _) in &weighted_edges[..3] {
        graph.get_mut(&a).unwrap().remove(&b);
        graph.get_mut(&b).unwrap().remove(&a);
    }

    assert_ne!(graph, copy);

    let first_size = component_size(nodes[0], &graph);

    Ok((first_size * (graph.len() - first_size)).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/25.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("54", part1(SAMPLE).unwrap());
    }
}
