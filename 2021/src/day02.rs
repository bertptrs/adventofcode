use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

enum Dir {
    Up,
    Down,
    Forward,
}

fn parse_input(input: &mut dyn Read) -> Vec<(Dir, i32)> {
    let mut reader = BufReader::new(input);
    let mut buffer = String::new();

    let mut moves = Vec::new();

    while matches!(reader.read_line(&mut buffer), Ok(n) if n > 0) {
        let (dir, amount) = buffer.trim().split_once(" ").unwrap();

        let dir = match dir {
            "up" => Dir::Up,
            "down" => Dir::Down,
            "forward" => Dir::Forward,
            _ => panic!("Invalid direction '{}'", dir),
        };

        moves.push((dir, amount.parse().unwrap()));

        buffer.clear();
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
