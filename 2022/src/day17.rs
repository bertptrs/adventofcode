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

pub fn part1(input: &[u8]) -> Result<String> {
    // Poor man's trim()
    let input = if input[input.len() - 1] == b'\n' {
        &input[..input.len() - 1]
    } else {
        input
    };

    let mut cavern = IndexSet::default();
    let mut max_height = 0;

    let mut gusts = input
        .iter()
        .cycle()
        .map(|&b| if b == b'<' { -1 } else { 1 });

    for &shape in SHAPES.iter().cycle().take(2022) {
        let mut x = 2usize;
        let mut y = max_height + shape.len() + 2;

        // Acquire gust of wind
        'falling: for offset in gusts.by_ref() {
            if let Some(nx) = x.checked_add_signed(offset) {
                let mut should_move = true;

                'collision: for (row, line) in shape.iter().enumerate() {
                    if nx + line.len() > WIDTH {
                        should_move = false;
                        break 'collision;
                    }

                    for (col, &on) in line.iter().enumerate() {
                        if on && cavern.contains((y - row) * WIDTH + nx + col) {
                            should_move = false;
                            break 'collision;
                        }
                    }
                }

                if should_move {
                    x = nx;
                }
            } else {
                // Hit the left wall
            }

            // Move down if possible
            if y >= shape.len() {
                let ny = y - 1;
                for (row, line) in shape.iter().enumerate() {
                    // No width check, should not hit that on the way down.
                    for (col, &on) in line.iter().enumerate() {
                        if on && cavern.contains((ny - row) * WIDTH + x + col) {
                            break 'falling;
                        }
                    }
                }
                y = ny;
            } else {
                break 'falling;
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

    Ok(max_height.to_string())
}

pub fn part2(_input: &[u8]) -> Result<String> {
    anyhow::bail!("not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/17.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "3068");
    }
}
