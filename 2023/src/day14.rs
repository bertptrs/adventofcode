use crate::common::Grid;
use crate::common::OwnedGrid;

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

fn advance(grid: &mut OwnedGrid, stack_heights: &mut [usize]) {
    // Tilt north
    stack_heights.fill(0);
    for y in 0..grid.height() {
        for (x, stack_height) in stack_heights.iter_mut().enumerate() {
            let c = grid[y][x];
            match c {
                b'#' => *stack_height = y + 1,
                b'O' => {
                    grid[y][x] = b'.';
                    grid[*stack_height][x] = b'O';

                    *stack_height += 1;
                }
                _ => continue,
            }
        }
    }

    // Tilt west
    for row in grid.rows_mut() {
        let mut stack_height = 0;
        for x in 0..row.len() {
            let c = row[x];
            match c {
                b'#' => stack_height = x + 1,
                b'O' => {
                    row[x] = b'.';
                    row[stack_height] = b'O';

                    stack_height += 1;
                }
                _ => continue,
            }
        }
    }

    // Tilt south
    stack_heights.fill(grid.height() - 1);
    for y in (0..grid.height()).rev() {
        for (x, stack_height) in stack_heights.iter_mut().enumerate() {
            let c = grid[y][x];
            match c {
                b'#' => *stack_height = y.saturating_sub(1),
                b'O' => {
                    grid[y][x] = b'.';
                    grid[*stack_height][x] = b'O';

                    // Saturating because possible underflow
                    *stack_height = stack_height.saturating_sub(1);
                }
                _ => continue,
            }
        }
    }

    // Tilt east
    for row in grid.rows_mut() {
        let mut stack_height = row.len() - 1;
        for x in (0..row.len()).rev() {
            let c = row[x];
            match c {
                b'#' => stack_height = x.saturating_sub(1),
                b'O' => {
                    row[x] = b'.';
                    row[stack_height] = b'O';

                    stack_height = stack_height.saturating_sub(1);
                }
                _ => continue,
            }
        }
    }
}

fn find_cycle(
    it: impl Iterator<Item = usize>,
    hare: &mut OwnedGrid,
    stack_heights: &mut [usize],
) -> Option<usize> {
    let mut tortoise = hare.clone();
    let mut last_sync = 0;

    for cycle in it {
        advance(hare, stack_heights);

        if tortoise == *hare {
            return Some(cycle - last_sync);
        } else if cycle.count_ones() == 1 {
            // New power of two, sync up the tortoise and the hare
            tortoise.clone_from(&hare);
            last_sync = cycle;
        }
    }
    None
}

pub fn part2(input: &[u8]) -> anyhow::Result<String> {
    const GOAL: usize = 1000000000;
    let mut hare = OwnedGrid::new(input.to_owned())?;
    let mut stack_heights = vec![0; hare.width()];

    let mut it = 1..=GOAL;

    // If there is a cycle found, skip ahead
    if let Some(len) = find_cycle(&mut it, &mut hare, &mut stack_heights) {
        let remaining = it.size_hint().0;
        let steps = remaining % len;

        for _ in 0..steps {
            advance(&mut hare, &mut stack_heights);
        }
    }

    let height = hare.height();
    let load = hare
        .rows()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .filter_map(move |&c| (c == b'O').then_some(height - y))
        })
        .sum::<usize>();

    Ok(load.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/14.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("136", part1(SAMPLE).unwrap());
    }

    #[test]
    fn sample_part2() {
        assert_eq!("64", part2(SAMPLE).unwrap());
    }
}
