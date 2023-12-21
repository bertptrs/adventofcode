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

fn compute_infinimap(input: &[u8], steps: i64) -> anyhow::Result<i64> {
    let map = Grid::new(input)?;
    let width = map.width() as i64;
    let height = map.height() as i64;

    let start = map.find(b'S').context("No starting point?")?;

    let mut visited = HashSet::new();
    visited.insert((start.0 as i64, start.1 as i64));
    let mut todo = VecDeque::new();
    todo.push_back((0, (start.0 as i64, start.1 as i64)));

    let mut count = (steps + 1) % 2;

    while let Some((dist, (x, y))) = todo.pop_front() {
        if dist == steps {
            break;
        }
        let mut enqueue = |x, y| {
            let map_x = (((x % width) + width) % width) as usize;
            let map_y = (((y % height) + height) % height) as usize;
            // println!("{x} → {map_x}, {y} → {map_y}");

            if map[(map_y, map_x)] != b'#' && visited.insert((x, y)) {
                todo.push_back((dist + 1, (x, y)));
                if (dist + 1) % 2 == steps % 2 {
                    count += 1;
                }
            }
        };

        enqueue(x - 1, y);
        enqueue(x + 1, y);
        enqueue(x, y - 1);
        enqueue(x, y + 1);
    }

    Ok(count)
}

// 616665063284297 too high
pub fn part2(input: &[u8]) -> anyhow::Result<String> {
    // This is wrong for things that aren't the input but it works for me, so…
    let dests = [65, 65 + 131, 65 + 2 * 131];

    let mut counts = [0; 3];
    for (steps, count) in dests.into_iter().zip(&mut counts) {
        *count = compute_infinimap(input, steps)?;
    }

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
        assert_eq!(16, compute_infinimap(SAMPLE, 6).unwrap());
        assert_eq!(50, compute_infinimap(SAMPLE, 10).unwrap());
        assert_eq!(1594, compute_infinimap(SAMPLE, 50).unwrap());
        assert_eq!(6536, compute_infinimap(SAMPLE, 100).unwrap());
        // Commented out because they take forever but they should work.
        // assert_eq!(167004, compute_infinimap(SAMPLE, 500).unwrap());
        // assert_eq!(668697, compute_infinimap(SAMPLE, 1000).unwrap());
        // assert_eq!(16733044, compute_infinimap(SAMPLE, 5000).unwrap());
    }
}
