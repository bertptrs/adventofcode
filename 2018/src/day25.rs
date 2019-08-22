use std::io::Read;

use regex::Regex;

use common::Point;
use common::Solution;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Default)]
pub struct Day25 {
    edges: Vec<Vec<usize>>,
}

impl Day25 {
    pub fn new() -> Self {
        Default::default()
    }

    fn read_input(&mut self, input: &mut dyn Read) {
        let matcher = Regex::new(r"-?\d+").unwrap();
        let reader = BufReader::new(input);

        let mut points = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();

            let mut coords = [0; 4];
            for (dest, coord) in coords.iter_mut().zip(matcher.find_iter(&line)) {
                *dest = coord.as_str().parse().unwrap();
            }

            points.push(coords);
        }

        let points = points;
        self.edges.resize(points.len(), Vec::new());

        for (x, a) in points.iter().enumerate() {
            for (y, b) in points.iter().enumerate() {
                if y >= x {
                    break;
                }

                if a.manhattan(*b) <= 3 {
                    self.edges[x].push(y);
                    self.edges[y].push(x);
                }
            }
        }
    }

    fn connected_components(&self) -> usize {
        let mut components = 0;

        let mut visited = vec![false; self.edges.len()];
        let mut todo = Vec::new();

        for i in 0..visited.len() {
            if !visited[i] {
                components += 1;
                todo.push(i);
                visited[i] = true;

                while let Some(i) = todo.pop() {
                    for &neighbour in &self.edges[i] {
                        if !visited[neighbour] {
                            visited[neighbour] = true;
                            todo.push(neighbour);
                        }
                    }
                }
            }
        }

        components
    }
}

impl Solution for Day25 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        self.read_input(input);
        self.connected_components().to_string()
    }

    fn part2(&mut self, _input: &mut dyn Read) -> String {
        // As always, no part 2 for day 25.
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use common::Solution;
    use day25::Day25;

    #[test]
    fn sample_part1() {
        let inputs: [&[u8]; 4] = [
            include_bytes!("samples/25.1.txt"),
            include_bytes!("samples/25.2.txt"),
            include_bytes!("samples/25.3.txt"),
            include_bytes!("samples/25.4.txt"),
        ];
        let outputs = ["2", "4", "3", "8"];

        for (&input, &output) in inputs.iter().zip(outputs.iter()) {
            let mut instance = Day25::new();
            assert_eq!(output, instance.part1(&mut input.as_ref()));
        }
    }
}
