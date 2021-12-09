use std::cmp::Reverse;
use std::io::Read;

pub fn part1(input: &mut dyn Read) -> String {
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer).unwrap();

    let lines: Vec<_> = buffer
        .split(|&s| s == b'\n')
        .filter(|s| !s.is_empty())
        .collect();

    let mut total_danger = 0;

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if x > 0 && lines[y][x - 1] <= lines[y][x] {
                continue;
            }

            if x + 1 < lines[y].len() && lines[y][x + 1] <= lines[y][x] {
                continue;
            }

            if y > 0 && lines[y - 1][x] <= lines[y][x] {
                continue;
            }

            if y + 1 < lines.len() && lines[y + 1][x] <= lines[y][x] {
                continue;
            }

            total_danger += 1 + (lines[y][x] - b'0') as i32;
        }
    }

    total_danger.to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer).unwrap();

    let mut visited_buffer = vec![false; buffer.len()];

    let lines: Vec<_> = buffer
        .split(|&s| s == b'\n')
        .filter(|s| !s.is_empty())
        .collect();

    let mut visited: Vec<_> = visited_buffer.chunks_exact_mut(lines[0].len()).collect();

    let mut todo = Vec::new();

    let mut sizes = Vec::with_capacity(4);

    for y in 0..lines.len() {
        for x in 0..lines[0].len() {
            if visited[y][x] || lines[y][x] == b'9' {
                continue;
            }

            todo.push((x, y));
            let mut size = 1;
            visited[y][x] = true;

            while let Some((x, y)) = todo.pop() {
                let mut add = |x: usize, y: usize| {
                    if lines[y][x] != b'9' && !visited[y][x] {
                        size += 1;
                        visited[y][x] = true;
                        todo.push((x, y));
                    }
                };

                if x > 0 {
                    add(x - 1, y);
                }

                if x + 1 < lines[y].len() {
                    add(x + 1, y);
                }

                if y > 0 {
                    add(x, y - 1)
                }

                if y + 1 < lines.len() {
                    add(x, y + 1);
                }
            }

            sizes.push(Reverse(size));
            sizes.sort_unstable();
            sizes.truncate(3);
        }
    }

    sizes.into_iter().fold(1, |a, Reverse(b)| a * b).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_implementation;

    const SAMPLE: &[u8] = include_bytes!("samples/09.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 15);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 1134);
    }
}
