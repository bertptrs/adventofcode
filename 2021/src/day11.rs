use std::io::Read;

fn read_grid<'a>(input: &'_ mut dyn Read, buffer: &'a mut Vec<u8>) -> Vec<&'a mut [u8]> {
    input.read_to_end(buffer).unwrap();

    let mut grid: Vec<&mut [u8]> = buffer.split_mut(|&b| b == b'\n').collect();

    grid.iter_mut()
        .flat_map(|line| line.iter_mut())
        .for_each(|b| *b -= b'0');

    grid
}

fn advance(grid: &mut [&mut [u8]], todo: &mut Vec<(i8, i8)>) -> usize {
    let mut flashes = 0;

    grid.iter_mut()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter_mut()
                .enumerate()
                .map(move |(x, value)| (x, y, value))
        })
        .for_each(|(x, y, value)| {
            *value += 1;
            if *value > 9 {
                todo.push((x as i8, y as i8));
            }
        });

    while let Some((x, y)) = todo.pop() {
        flashes += 1;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = usize::try_from(x + dx);
                let ny = usize::try_from(y + dy);

                if let (Ok(nx), Ok(ny)) = (nx, ny) {
                    if let Some(value) = grid.get_mut(ny).and_then(|line| line.get_mut(nx)) {
                        *value += 1;
                        if *value == 10 {
                            todo.push((nx as i8, ny as i8));
                        }
                    }
                }
            }
        }
    }

    grid.iter_mut()
        .flat_map(|line| line.iter_mut())
        .filter(|b| **b > 9)
        .for_each(|b| *b = 0);

    flashes
}

pub fn part1(input: &mut dyn Read) -> String {
    let mut buffer = Vec::new();

    let mut grid = read_grid(input, &mut buffer);
    let mut todo = Vec::new();

    (0..100)
        .map(|_| advance(&mut grid, &mut todo))
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let mut buffer = Vec::new();

    let mut grid = read_grid(input, &mut buffer);
    let mut todo = Vec::new();

    let target: usize = grid.iter().map(|line| line.len()).sum();

    (1..)
        .find(|_| advance(&mut grid, &mut todo) == target)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_implementation;

    const SAMPLE: &[u8] = include_bytes!("samples/11.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 1656);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 195);
    }
}
