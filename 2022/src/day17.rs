use anyhow::Result;

use crate::common::IndexSet;

const SHAPES: [&[&[bool]]; 5] = [
    &[&[true; 4]],
    &[&[false, true, false], &[true; 3], &[false, true, false]],
    &[&[false, false, true], &[false, false, true], &[true; 3]],
    &[&[true], &[true], &[true], &[true]],
    &[&[true; 2], &[true; 2]],
];

const WIDTH: usize = 7;

#[allow(unused)]
fn print_cavern(cavern: &IndexSet, max_height: usize) {
    for y in (0..=max_height).rev() {
        for x in 0..7 {
            print!(
                "{}",
                if cavern.contains(y * WIDTH + x) {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!();
    }
}

fn collides(shape: &[&[bool]], cavern: &IndexSet, x: usize, y: usize) -> bool {
    for (row, line) in shape.iter().enumerate() {
        if x + line.len() > WIDTH {
            return true;
        }

        for (col, &on) in line.iter().enumerate() {
            if on && cavern.contains((y - row) * WIDTH + x + col) {
                return true;
            }
        }
    }

    false
}

fn simulate(
    mut cavern: IndexSet,
    shapes: impl Iterator<Item = &'static &'static [&'static [bool]]>,
    mut gusts: impl Iterator<Item = isize>,
    mut max_height: usize,
) -> usize {
    for &shape in shapes {
        let mut x = 2usize;
        let mut y = max_height + shape.len() + 2;

        // Acquire gust of wind
        for offset in gusts.by_ref() {
            if let Some(nx) = x.checked_add_signed(offset) {
                if !collides(shape, &cavern, nx, y) {
                    x = nx;
                }
            } else {
                // Hit the left wall
            }

            // Move down if possible
            if y >= shape.len() && !collides(shape, &cavern, x, y - 1) {
                y -= 1;
            } else {
                break;
            }
        }

        // If we get here we've successfully stopped falling
        max_height = max_height.max(y + 1);

        for (row, line) in shape.iter().enumerate() {
            for (col, &on) in line.iter().enumerate() {
                if on {
                    cavern.insert((y - row) * WIDTH + x + col);
                }
            }
        }
    }

    max_height
}

pub fn part1(input: &[u8]) -> Result<String> {
    // Poor man's trim()
    let input = if input[input.len() - 1] == b'\n' {
        &input[..input.len() - 1]
    } else {
        input
    };

    let gusts = input
        .iter()
        .cycle()
        .map(|&b| if b == b'<' { -1 } else { 1 });

    Ok(simulate(
        IndexSet::default(),
        SHAPES.iter().cycle().take(2022),
        gusts,
        0,
    )
    .to_string())
}

#[derive(Clone, Copy, Debug)]
struct HeightMap([usize; 7]);

impl HeightMap {
    fn max(&self) -> &usize {
        self.0.iter().max().unwrap()
    }
}

impl PartialEq for HeightMap {
    fn eq(&self, other: &Self) -> bool {
        let self_min = *self.0.iter().min().unwrap();
        let other_min = *other.0.iter().min().unwrap();

        self.0
            .iter()
            .zip(&other.0)
            .all(|(&a, &b)| a - self_min == b - other_min)
    }
}

pub fn part2(input: &[u8]) -> Result<String> {
    // Poor man's trim()
    let input = if input[input.len() - 1] == b'\n' {
        &input[..input.len() - 1]
    } else {
        input
    };

    let mut cavern = IndexSet::default();

    let mut height_map = HeightMap([0; 7]);

    let mut shapes = SHAPES.iter().enumerate().cycle();
    let mut gusts = input
        .iter()
        .map(|&b| if b == b'<' { -1 } else { 1 })
        .enumerate()
        .cycle();

    let mut tortoise = (0, 0, height_map);
    let mut tortoise_time = 0;
    let mut last_gust = 0;

    const TARGET: usize = 1000000000000;

    for (it, (shape_id, &shape)) in shapes.by_ref().enumerate() {
        let mut x = 2usize;
        let mut y = height_map.max() + shape.len() + 2;

        // Acquire gust of wind
        for (gust_id, offset) in gusts.by_ref() {
            last_gust = gust_id;
            if let Some(nx) = x.checked_add_signed(offset) {
                if !collides(shape, &cavern, nx, y) {
                    x = nx;
                }
            } else {
                // Hit the left wall
            }

            // Move down if possible
            if y >= shape.len() && !collides(shape, &cavern, x, y - 1) {
                y -= 1;
            } else {
                break;
            }
        }

        // If we get here we've successfully stopped falling
        for (row, line) in shape.iter().enumerate() {
            for (col, &on) in line.iter().enumerate() {
                if on {
                    height_map.0[x + col] = height_map.0[x + col].max(y - row + 1);
                    cavern.insert((y - row) * WIDTH + x + col);
                }
            }
        }

        // See if we found a circle
        let hare_time = it + 1;
        let hare = (shape_id, last_gust, height_map);

        if hare == tortoise {
            let cycle_len = hare_time - tortoise_time;
            let remaining = TARGET - hare_time;
            let to_skip = remaining / cycle_len;
            let remainder = remaining % cycle_len;

            let current_max = *height_map.max();

            // All of them rose by the same amount so we just need to compare the first one
            let delta = height_map.0[0] - tortoise.2 .0[0];

            let result = simulate(
                cavern,
                shapes.map(|(_, shape)| shape).take(remainder),
                gusts.map(|(_, offset)| offset),
                current_max,
            ) + delta * to_skip;

            return Ok(result.to_string());
        } else if it.count_ones() == 1 {
            tortoise = hare;
            tortoise_time = hare_time;
        }
    }

    Ok(height_map.max().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/17.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "3068");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "1514285714288");
    }
}
