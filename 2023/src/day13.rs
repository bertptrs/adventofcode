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

fn find_symmetry(grid: Grid<'_>, differences: usize) -> Option<Symmetry> {
    // Attempt to find a vertical line of reflection first
    for c in 1..grid.width() {
        if grid
            .rows()
            .flat_map(|row| row[..c].iter().rev().zip(&row[c..]))
            .filter(|(a, b)| a != b)
            .take(differences + 1)
            .count()
            == differences
        {
            return Some(Symmetry::Vertical(c as u32));
        }
    }

    for r in 1..grid.height() {
        if (0..r)
            .rev()
            .zip(r..grid.height())
            .flat_map(|(a, b)| grid[a].iter().zip(&grid[b]))
            .filter(|(a, b)| a != b)
            .take(differences + 1)
            .count()
            == differences
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

fn parts_common(input: &[u8], differences: usize) -> anyhow::Result<String> {
    let grids = parse_grids(input)?;

    let sum: u32 = grids
        .into_iter()
        .filter_map(|grid| find_symmetry(grid, differences))
        .map(Symmetry::value)
        .sum();
    Ok(sum.to_string())
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    parts_common(input, 0)
}

pub fn part2(input: &[u8]) -> anyhow::Result<String> {
    parts_common(input, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/13.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("405", part1(SAMPLE).unwrap());
    }

    #[test]
    fn sample_part2() {
        assert_eq!("400", part2(SAMPLE).unwrap());
    }
}
