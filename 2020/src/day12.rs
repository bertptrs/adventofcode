use std::io::Read;

use crate::common::Lines;
use crate::Solution;

// Rotate clockwise
fn rotate(amount: i32, (x, y): (i32, i32)) -> (i32, i32) {
    debug_assert!(amount >= 0);
    debug_assert_eq!(amount % 90, 0);

    match amount {
        90 => (y, -x),
        180 => (-x, -y),
        270 => (-y, x),
        360 => (x, y),
        _ => panic!("Invalid rotate manouvre {}", amount),
    }
}

#[derive(Default)]
pub struct Day12;

impl Solution for Day12 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let mut x = 0;
        let mut y = 0;
        let mut v = (1, 0);

        for line in Lines::new(input) {
            let dir = &line[0..1];
            let amount: i32 = line[1..].parse().unwrap();

            match dir {
                "N" => y += amount,
                "E" => x += amount,
                "S" => y -= amount,
                "W" => x -= amount,
                "F" => {
                    let (dx, dy) = v;

                    x += amount * dx;
                    y += amount * dy;
                }
                "R" => v = rotate(amount, v),
                "L" => v = rotate(360 - amount, v),
                _ => panic!("Invalid direction '{}'", dir),
            };
        }

        (x.abs() + y.abs()).to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let mut x = 0;
        let mut y = 0;
        let mut wx = 10;
        let mut wy = 1;

        for line in Lines::new(input) {
            let dir = &line[0..1];
            let amount: i32 = line[1..].parse().unwrap();

            match dir {
                "N" => wy += amount,
                "E" => wx += amount,
                "S" => wy -= amount,
                "W" => wx -= amount,
                "F" => {
                    x += wx * amount;
                    y += wy * amount;
                }
                "R" => {
                    let v = rotate(amount, (wx, wy));
                    wx = v.0;
                    wy = v.1;
                }
                "L" => {
                    let v = rotate(360 - amount, (wx, wy));
                    wx = v.0;
                    wy = v.1;
                }
                _ => panic!("Invalid direction '{}'", dir),
            };
        }

        (x.abs() + y.abs()).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../samples/12.txt");

    #[test]
    fn sample_part1() {
        test_implementation(Day12, 1, SAMPLE, 25);
    }

    #[test]
    fn sample_part2() {
        test_implementation(Day12, 2, SAMPLE, 286);
    }
}
