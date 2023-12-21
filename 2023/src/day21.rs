use std::collections::HashSet;
use std::collections::VecDeque;

use anyhow::Context;

use crate::common::Grid;
use crate::common::IndexSet;

fn part1_parametrized(input: &[u8], steps: usize) -> anyhow::Result<String> {
    let map = Grid::new(input)?;
    let mut visited = IndexSet::with_capacity(map.width() * map.height());
    let mut todo = VecDeque::new();
    let start = map.find(b'S').context("Couldn't find starting point")?;
    todo.push_back((0, start));
    visited.insert(start.1 * map.width() + start.1);
    let mut count = (steps + 1) % 2;

    while let Some((dist, (x, y))) = todo.pop_front() {
        let mut enqueue = |x, y| {
            if map[(y, x)] != b'#' && visited.insert(y * map.width() + x) {
                if dist < steps {
                    todo.push_back((dist + 1, (x, y)));
                }

                if (dist + 1) % 2 == steps % 2 {
                    count += 1;
                }
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

    Ok(count.to_string())
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    part1_parametrized(input, 64)
}

fn compute_infinimap<const L: usize>(
    input: &[u8],
    steps_groups: [i64; L],
) -> anyhow::Result<[i64; L]> {
    let map = Grid::new(input)?;
    let width = map.width() as i64;
    let height = map.height() as i64;

    let start = map.find(b'S').context("No starting point?")?;

    let mut visited = HashSet::new();
    visited.insert((start.0 as i64, start.1 as i64));
    let mut todo = VecDeque::new();
    todo.push_back((0, (start.0 as i64, start.1 as i64)));

    let mut final_counts = [0; L];
    let mut counts = [1, 0];
    let mut i = 0;

    while let Some((dist, (x, y))) = todo.pop_front() {
        if dist == steps_groups[i] {
            final_counts[i] = counts[(steps_groups[i] % 2) as usize];
            i += 1;
            if i >= steps_groups.len() {
                break;
            }
        }
        let mut enqueue = |x, y| {
            let map_x = (((x % width) + width) % width) as usize;
            let map_y = (((y % height) + height) % height) as usize;
            // println!("{x} → {map_x}, {y} → {map_y}");

            if map[(map_y, map_x)] != b'#' && visited.insert((x, y)) {
                todo.push_back((dist + 1, (x, y)));
                counts[((dist + 1) % 2) as usize] += 1;
            }
        };

        enqueue(x - 1, y);
        enqueue(x + 1, y);
        enqueue(x, y - 1);
        enqueue(x, y + 1);
    }

    Ok(final_counts)
}

// 616665063284297 too high
pub fn part2(input: &[u8]) -> anyhow::Result<String> {
    // This is wrong for things that aren't the input but it works for me, so…
    let dests = [65, 65 + 131, 65 + 2 * 131];

    let counts = compute_infinimap(input, dests)?;

    // Stolen set of equations, this fits a quadratic equation to a three points at x = 0, 1, 2
    let a = 0;
    let b = 0;
    let c = 1;
    let d = counts[0];
    let e = 1;
    let f = 1;
    let g = 1;
    let h = counts[1];
    let i = 2 * 2;
    let j = 2;
    let k = 1;
    let l = counts[2];

    let delta = (a * f * k) + (b * g * i) + (c * e * j) - (c * f * i) - (a * g * j) - (b * e * k);
    let a_numerator =
        (d * f * k) + (b * g * l) + (c * h * j) - (c * f * l) - (d * g * j) - (b * h * k);
    let b_numerator =
        (a * h * k) + (d * g * i) + (c * e * l) - (c * h * i) - (a * g * l) - (d * e * k);
    let c_numerator =
        (a * f * l) + (b * h * i) + (d * e * j) - (d * f * i) - (a * h * j) - (b * e * l);

    let a = a_numerator / delta;
    let b = b_numerator / delta;
    let c = c_numerator / delta;

    let x = 26501365 / 131;

    let result = a * (x * x) + b * x + c;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/21.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("16", part1_parametrized(SAMPLE, 6).unwrap());
    }

    #[test]
    fn sample_infinimap() {
        let inputs = [
            6, 10, 50, 100,
            // 500
            // 1000
            // 5000
        ];
        let expected = [
            16, 50, 1594, 6536,
            // 167004,
            // 668697,
            // 16733044,
        ];

        assert_eq!(expected, compute_infinimap(SAMPLE, inputs).unwrap());
    }
}
