use std::io::Read;

use crate::common::LineIter;

enum Dir {
    Up,
    Down,
    Forward,
}

fn parse_input(input: &mut dyn Read) -> Vec<(Dir, i32)> {
    let mut reader = LineIter::new(input);
    let mut moves = Vec::new();

    while let Some(line) = reader.next() {
        let (dir, amount) = line.split_once(' ').unwrap();

        let dir = match dir {
            "up" => Dir::Up,
            "down" => Dir::Down,
            "forward" => Dir::Forward,
            _ => panic!("Invalid direction '{}'", dir),
        };

        moves.push((dir, amount.parse().unwrap()));
    }

    moves
}

pub fn part1(input: &mut dyn Read) -> String {
    let moves = parse_input(input);

    let mut x = 0;
    let mut depth = 0;

    for (dir, amount) in moves {
        match dir {
            Dir::Up => depth -= amount,
            Dir::Down => depth += amount,
            Dir::Forward => x += amount,
        }
    }

    (x * depth).to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let moves = parse_input(input);

    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (dir, amount) in moves {
        match dir {
            Dir::Up => aim -= amount,
            Dir::Down => aim += amount,
            Dir::Forward => {
                x += amount;
                depth += aim * amount;
            }
        }
    }

    (x * depth).to_string()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/02.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 150);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part1, SAMPLE, 150);
    }
}
