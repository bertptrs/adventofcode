use crate::common::Grid;

#[derive(Debug)]
enum Symmetry {
    Horizontal(u32),
    Vertical(u32),
}

impl Symmetry {
    fn value(self) -> u32 {
        match self {
            Symmetry::Horizontal(value) => 100 * value,
            Symmetry::Vertical(value) => value,
        }
    }
}

fn find_symmetry(grid: Grid<'_>) -> Option<Symmetry> {
    // Attempt to find a vertical line of reflection first
    for c in 1..grid.width() {
        if grid
            .rows()
            .all(|row| row[..c].iter().rev().zip(&row[c..]).all(|(a, b)| a == b))
        {
            return Some(Symmetry::Vertical(c as u32));
        }
    }

    for r in 1..grid.height() {
        if (0..r)
            .rev()
            .zip(r..grid.height())
            .all(|(a, b)| grid[a] == grid[b])
        {
            return Some(Symmetry::Horizontal(r as u32));
        }
    }

    println!("Suspiciously did not find an axis of symmetry:\n\n{grid}\n");
    None
}

fn parse_grids(input: &[u8]) -> anyhow::Result<Vec<Grid<'_>>> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut last_newline = 0;

    for i in input
        .iter()
        .enumerate()
        .filter_map(|(i, c)| (*c == b'\n').then_some(i))
    {
        if last_newline == i - 1 {
            result.push(Grid::new(&input[start..i])?);
            start = i + 1;
        }
        last_newline = i;
    }

    result.push(Grid::new(&input[start..])?);

    Ok(result)
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let grids = parse_grids(input)?;

    let sum: u32 = grids
        .into_iter()
        .filter_map(find_symmetry)
        .map(Symmetry::value)
        .sum();
    Ok(sum.to_string())
}

pub fn part2(_input: &[u8]) -> anyhow::Result<String> {
    anyhow::bail!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/13.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("405", part1(SAMPLE).unwrap());
    }

    // #[test]
    // fn sample_part2() {
    //     assert_eq!("525152", part2(SAMPLE).unwrap());
    // }
}
