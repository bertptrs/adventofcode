use std::collections::HashMap;
use std::ops::Index;

use anyhow::Context;

struct Grid<'a> {
    width: usize,
    data: &'a [u8],
}

impl<'a> Grid<'a> {
    pub fn new(data: &'a [u8]) -> anyhow::Result<Self> {
        let width = 1 + data
            .iter()
            .position(|&c| c == b'\n')
            .context("Failed to find end of line in grid")?;

        anyhow::ensure!(
            data.len() % width == 0,
            "Grid should divide equally into rows"
        );

        Ok(Self { width, data })
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }

    pub fn width(&self) -> usize {
        self.width - 1
    }

    pub fn rows(&self) -> impl Iterator<Item = &'a [u8]> {
        let width = self.width();
        self.data
            .chunks_exact(self.width)
            .map(move |row| &row[..width])
    }
}

impl<'a> Index<usize> for Grid<'a> {
    type Output = [u8];

    fn index(&self, y: usize) -> &Self::Output {
        let offset = y * self.width;
        &self.data[offset..(offset + self.width())]
    }
}

fn is_surrounded(grid: &Grid<'_>, y: usize, start: usize, last: usize) -> bool {
    fn is_symbol(c: u8) -> bool {
        !matches!(c, b'0'..=b'9' | b'.' | b'\n')
    }
    let x_min = start.saturating_sub(1);
    let x_max = Ord::min(grid.width(), last + 2);

    if y > 0 {
        for nx in x_min..x_max {
            if is_symbol(grid[y - 1][nx]) {
                return true;
            }
        }
    }

    if y + 1 < grid.height() {
        for nx in x_min..x_max {
            if is_symbol(grid[y + 1][nx]) {
                return true;
            }
        }
    }

    is_symbol(grid[y][x_min]) || is_symbol(grid[y][x_max - 1])
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let grid = Grid::new(input)?;
    let mut sum = 0;

    for (y, row) in grid.rows().enumerate() {
        let mut it = row.iter().enumerate();
        while let Some((start, &c)) = it.find(|&(_, c)| c.is_ascii_digit()) {
            let mut end = start;
            let mut num = u32::from(c - b'0');

            for (x, &c) in it.by_ref().take_while(|&(_, c)| c.is_ascii_digit()) {
                end = x;
                num *= 10;
                num += u32::from(c - b'0');
            }

            if is_surrounded(&grid, y, start, end) {
                sum += num;
            }
        }
    }

    Ok(sum.to_string())
}

fn find_gear(grid: &Grid<'_>, y: usize, start: usize, end: usize) -> Option<(usize, usize)> {
    let x_min = start.saturating_sub(1);
    let x_max = Ord::min(grid.width(), end + 2);

    if y > 0 {
        for nx in x_min..x_max {
            if grid[y - 1][nx] == b'*' {
                return Some((nx, y - 1));
            }
        }
    }

    if y + 1 < grid.height() {
        for nx in x_min..x_max {
            if grid[y + 1][nx] == b'*' {
                return Some((nx, y + 1));
            }
        }
    }

    for nx in [x_min, x_max - 1] {
        if grid[y][nx] == b'*' {
            return Some((nx, y));
        }
    }

    None
}

pub fn part2(input: &[u8]) -> anyhow::Result<String> {
    let grid = Grid::new(input)?;
    let mut gears: HashMap<(usize, usize), (u32, u32)> = HashMap::new();

    for (y, row) in grid.rows().enumerate() {
        let mut it = row.iter().enumerate();
        while let Some((start, &c)) = it.find(|&(_, c)| c.is_ascii_digit()) {
            let mut end = start;
            let mut num = u32::from(c - b'0');

            for (x, &c) in it.by_ref().take_while(|&(_, c)| c.is_ascii_digit()) {
                end = x;
                num *= 10;
                num += u32::from(c - b'0');
            }

            // Assumption: there is only one gear per number. This turns out to be true.
            if let Some((gear_x, gear_y)) = find_gear(&grid, y, start, end) {
                let entry = gears.entry((gear_x, gear_y)).or_insert((0, 1));
                entry.0 += 1;
                entry.1 *= num;
            }
        }
    }

    let sum: u32 = gears
        .into_values()
        .filter_map(|(count, ratio)| if count == 2 { Some(ratio) } else { None })
        .sum();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/03.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "4361");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "467835");
    }
}
