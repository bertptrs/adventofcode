use anyhow::Context;
use anyhow::Result;

#[inline]
fn stripe<'a>(
    values: impl IntoIterator<Item = &'a u8>,
    visible: impl IntoIterator<Item = &'a mut bool>,
) {
    let mut max = 0;

    for (&val, visible) in values.into_iter().zip(visible) {
        if val > max {
            max = val;
            *visible = true;

            if val == b'9' {
                return;
            }
        }
    }
}

pub fn part1(input: &[u8]) -> Result<String> {
    let width = input
        .iter()
        .position(|&b| b == b'\n')
        .context("Single row field")?;
    let height = input.len() / (width + 1); // Include newlines

    let mut visible = vec![false; width * height];

    // Horizontal striping
    for (y, row) in input.chunks_exact(width + 1).enumerate() {
        // First, left to right
        stripe(&row[..width], &mut visible[(y * width)..]);

        // Then right to left
        stripe(
            row[..width].iter().rev(),
            visible[(y * width)..(y * width + width)].iter_mut().rev(),
        );
    }

    // Vertical striping
    for x in 0..width {
        // Top to bottom
        stripe(
            input[x..].iter().step_by(width + 1),
            visible[x..].iter_mut().step_by(width),
        );

        // Bottom to top
        stripe(
            input[x..].iter().step_by(width + 1).rev(),
            visible[x..].iter_mut().step_by(width).rev(),
        )
    }

    Ok(visible.into_iter().filter(|&b| b).count().to_string())
}

pub fn part2(_input: &[u8]) -> Result<String> {
    anyhow::bail!("not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/08.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "21");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "8");
    }
}
