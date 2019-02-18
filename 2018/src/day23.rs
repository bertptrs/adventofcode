use std::collections::HashSet;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use regex::Regex;

use common::Point;
use common::Solution;

type Coordinate = (i64, i64, i64);
type Graph<'a> = &'a [HashSet<usize>];
type NodeSet = HashSet<usize>;

fn bron_kerbosch(graph: Graph) -> Vec<NodeSet> {
    let mut cliques = Vec::new();

    let mut r = HashSet::new();
    let x = HashSet::new();
    let p: NodeSet = (0..graph.len()).collect();

    bron_kerbosch1(graph, &mut cliques, &mut r, p, x);

    cliques
}

fn bron_kerbosch1(
    graph: Graph,
    cliques: &mut Vec<NodeSet>,
    r: &mut NodeSet,
    p: NodeSet,
    mut x: NodeSet,
) {
    if p.is_empty() && x.is_empty() {
        if cliques.is_empty() {
            cliques.push(r.clone());
            return;
        }

        let cur = cliques.first().unwrap().len();
        if cur < r.len() {
            cliques.clear();
        }
        if cur <= r.len() {
            cliques.push(r.clone())
        }
        return;
    }

    let mut p_clone = p.clone();
    let pivot = *p.union(&x).max_by_key(|&&v| graph[v].len()).unwrap();

    for &v in p.difference(&graph[pivot]) {
        r.insert(v);
        let p1: NodeSet = p_clone.intersection(&graph[v]).cloned().collect();
        let x1: NodeSet = x.intersection(&graph[v]).cloned().collect();
        bron_kerbosch1(graph, cliques, r, p1, x1);
        r.remove(&v);

        p_clone.remove(&v);
        x.insert(v);
    }
}

#[derive(Default)]
pub struct Day23 {
    bots: Vec<(i64, Coordinate)>,
}

impl Day23 {
    pub fn new() -> Self {
        Default::default()
    }

    fn read_input(&mut self, input: &mut Read) {
        let matcher = Regex::new(r"-?\d+").unwrap();
        let reader = BufReader::new(input);

        for line in reader.lines() {
            let line = line.unwrap();

            let mut ints = [0i64; 4];
            for (c, i) in matcher.find_iter(&line).zip(ints.iter_mut()) {
                *i = c.as_str().parse().unwrap();
            }

            let pos = (ints[0], ints[1], ints[2]);

            self.bots.push((ints[3], pos));
        }
    }
}

impl Solution for Day23 {
    fn part1(&mut self, input: &mut Read) -> String {
        self.read_input(input);
        self.bots.sort_unstable();
        let (best_range, best_pos) = *self.bots.last().unwrap();

        let result = self
            .bots
            .iter()
            .filter(|(_, pos)| pos.manhattan(best_pos) <= best_range)
            .count();

        result.to_string()
    }

    fn part2(&mut self, input: &mut Read) -> String {
        self.read_input(input);
        let mut neighbours = vec![HashSet::new(); self.bots.len()];

        for (a, &(arad, ap)) in self.bots.iter().enumerate() {
            for (b, &(brad, bp)) in self.bots.iter().enumerate() {
                if b >= a {
                    break;
                }
                if arad + brad >= ap.manhattan(bp) && a != b {
                    neighbours[a].insert(b);
                    neighbours[b].insert(a);
                }
            }
        }

        let cliques = bron_kerbosch(&neighbours);

        let mut best = None;
        for clique in cliques {
            let dist = clique
                .iter()
                .map(|&x| (0, 0, 0).manhattan(self.bots[x].1) - self.bots[x].0)
                .max()
                .unwrap();
            if best.is_none() {
                best = Some(dist);
            } else {
                best = Some(best.unwrap().min(dist));
            }
        }

        best.unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use common::Solution;
    use day23::Day23;

    const SAMPLE1_INPUT: &[u8] = include_bytes!("samples/23.1.txt");
    const SAMPLE2_INPUT: &[u8] = include_bytes!("samples/23.2.txt");

    #[test]
    fn sample_part1() {
        let mut instance = Day23::new();
        assert_eq!("7", instance.part1(&mut SAMPLE1_INPUT));
    }

    #[test]
    fn sample_part2() {
        let mut instance = Day23::new();
        assert_eq!("36", instance.part2(&mut SAMPLE2_INPUT));
    }
}
