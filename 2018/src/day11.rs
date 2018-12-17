use std::i32;
use std::io::Read;

use common::Solution;
use common::read_single_input;

fn power_at(serial: i32, (x, y): (i32, i32)) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial;
    power_level *= rack_id;
    power_level = (power_level % 1000) / 100;
    power_level - 5
}

pub struct Day11 {
    power_grid: [[i32; 300]; 300],
}

impl Day11 {
    pub fn new() -> Self {
        Day11 {
            power_grid: [[0i32; 300]; 300],
        }
    }

    fn compute_summed_area(&mut self, serial: i32) {
        self.power_grid[0][0] = power_at(serial, (1, 1));
        for x in 1..300 {
            self.power_grid[0][x] = self.power_grid[0][x - 1] + power_at(serial, (x as i32 + 1, 1));
        }

        for y in 1..300 {
            self.power_grid[y][0] = self.power_grid[y - 1][0] + power_at(serial, (y as i32 + 1, 1));

            for x in 1..300 {
                let mut power = power_at(serial, (x as i32 + 1, y as i32 + 1));
                power += self.power_grid[y - 1][x];
                power += self.power_grid[y][x - 1];
                power -= self.power_grid[y - 1][x - 1];
                self.power_grid[y][x] = power;
            }
        }
    }

    fn best(&self, size: usize) -> (usize, usize, i32) {
        let mut best_coordinates = (1, 1);
        let mut best_result = self.power_grid[size - 1][size - 1];

        // First row
        for x in 0..(300 - size) {
            let score = self.power_grid[size - 1][x + size] - self.power_grid[size - 1][x];
            if score > best_result {
                best_result = score;
                best_coordinates = (x + 1, 1);
            }
        }

        // First column
        for y in 0..(300 - size) {
            let score = self.power_grid[y + size][size - 1] - self.power_grid[y][size - 1];
            if score > best_result {
                best_result = score;
                best_coordinates = (1, y + 1);
            }
        }

        // Remaining tiles
        for y in 0..(300 - size) {
            for x in 0..(300 - size) {
                let a = self.power_grid[y][x];
                let b = self.power_grid[y][x + size];
                let c = self.power_grid[y + size][x];
                let d = self.power_grid[y + size][x + size];
                let score = d + a - b - c;
                if score > best_result {
                    best_result = score;
                    best_coordinates = (x + 2, y + 2);
                }
            }
        }

        let (x, y) = best_coordinates;

        (x, y, best_result)
    }
}

impl Default for Day11 {
    fn default() -> Self {
        Self::new()
    }
}

impl Solution for Day11 {
    fn part1(&mut self, input: &mut Read) -> String {
        let serial = read_single_input(input);
        self.compute_summed_area(serial);
        let (x, y, _) = self.best(3);

        format!("{},{}", x, y)
    }

    fn part2(&mut self, input: &mut Read) -> String {
        let serial = read_single_input(input);
        self.compute_summed_area(serial);
        let mut best_result = 0;
        let mut best_option = None;
        for size in 1..=300 {
            let (x, y, result) = self.best(size);
            if result > best_result {
                best_result = result;
                best_option = Some((x, y, size));
            }
        }
        let (x, y, size) = best_option.unwrap();
        format!("{},{},{}", x, y, size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_at() {
        assert_eq!(4, power_at(8, (3, 5)));
        assert_eq!(-5, power_at(57, (122, 79)));
        assert_eq!(0, power_at(39, (217, 196)));
        assert_eq!(4, power_at(71, (101, 153)));
    }

    #[test]
    fn sample_part1() {
        let mut instance = Day11::new();
        assert_eq!("33,45", instance.part1(&mut b"18".as_ref()));
        assert_eq!("21,61", instance.part1(&mut b"42".as_ref()));
    }

    #[test]
    fn sample_part2() {
        let mut instance = Day11::new();
        assert_eq!("90,269,16", instance.part2(&mut b"18".as_ref()));
        assert_eq!("232,251,12", instance.part2(&mut b"42".as_ref()));
    }
}
