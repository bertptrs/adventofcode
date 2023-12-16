use crate::common::Grid;

#[derive(Clone, Copy)]
enum Direction {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3,
}

impl Direction {
    fn bit(self) -> u8 {
        1 << self as u8
    }
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let map = Grid::new(input)?;
    let mut state = Grid::zeroed(map.width(), map.height());
    state[0][0] = Direction::Right.bit();

    let mut todo = Vec::new();
    todo.push((Direction::Right, 0, 0));

    let mut energized = 1;

    while let Some((dir, x, y)) = todo.pop() {
        let mut enqueue = |dir: Direction, x: usize, y| {
            let state = &mut state[y][x];
            if state == &0 {
                energized += 1;
            }
            if *state & dir.bit() == 0 {
                *state |= dir.bit();
                todo.push((dir, x, y));
            }
        };

        let new_dir = match (map[y][x], dir) {
            (b'/', Direction::Left) => Direction::Down,
            (b'/', Direction::Up) => Direction::Right,
            (b'/', Direction::Down) => Direction::Left,
            (b'/', Direction::Right) => Direction::Up,
            (b'\\', Direction::Left) => Direction::Up,
            (b'\\', Direction::Up) => Direction::Left,
            (b'\\', Direction::Down) => Direction::Right,
            (b'\\', Direction::Right) => Direction::Down,
            (b'|', Direction::Right) | (b'|', Direction::Left) => {
                if y > 0 {
                    enqueue(Direction::Up, x, y - 1);
                }
                if y + 1 < map.height() {
                    enqueue(Direction::Down, x, y + 1);
                }
                continue;
            }
            (b'-', Direction::Up) | (b'-', Direction::Down) => {
                if x > 0 {
                    enqueue(Direction::Left, x - 1, y);
                }
                if x + 1 < map.width() {
                    enqueue(Direction::Right, x + 1, y);
                }
                continue;
            }
            (_, dir) => dir,
        };

        match new_dir {
            Direction::Up => {
                if y > 0 {
                    enqueue(new_dir, x, y - 1);
                }
            }
            Direction::Left => {
                if x > 0 {
                    enqueue(new_dir, x - 1, y);
                }
            }
            Direction::Down => {
                if y + 1 < map.height() {
                    enqueue(new_dir, x, y + 1);
                }
            }
            Direction::Right => {
                if x + 1 < map.width() {
                    enqueue(new_dir, x + 1, y);
                }
            }
        }
    }

    Ok(energized.to_string())
}

pub fn part2(_input: &[u8]) -> anyhow::Result<String> {
    anyhow::bail!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/16.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("46", part1(SAMPLE).unwrap());
    }

    // #[test]
    // fn sample_part2() {
    //     assert_eq!("64", part2(SAMPLE).unwrap());
    // }
}
