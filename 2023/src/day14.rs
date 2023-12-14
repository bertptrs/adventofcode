use crate::common::Grid;

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let grid = Grid::new(input)?;
    let mut stack_heights = vec![0; grid.width()];
    let mut load = 0;
    let height = grid.height();

    for (y, row) in grid.rows().enumerate() {
        for (&c, stack_height) in row.iter().zip(&mut stack_heights) {
            match c {
                b'#' => *stack_height = y + 1,
                b'O' => {
                    load += height - *stack_height;
                    *stack_height += 1;
                }
                _ => continue,
            }
        }
    }

    Ok(load.to_string())
}

pub fn part2(_input: &[u8]) -> anyhow::Result<String> {
    anyhow::bail!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/14.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("136", part1(SAMPLE).unwrap());
    }
}
