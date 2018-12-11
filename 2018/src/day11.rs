use std::io::Read;

use common::Solution;

fn power_at(serial: i32, (x, y): (i32, i32)) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial;
    power_level *= rack_id;
    power_level = (power_level % 1000) / 100;
    power_level - 5
}

#[derive(Default)]
pub struct Day11 {}

impl Day11 {
    pub fn new() -> Self {
        Default::default()
    }

    fn read_serial(&self, input: &mut Read) -> i32 {
        let mut data = String::new();
        input.read_to_string(&mut data).unwrap();
        data.trim().parse().unwrap()
    }

    fn best(&self, serial: i32, size: i32) -> (i32, i32, i32) {
        let mut best_coordinates: Option<(i32, i32)> = None;
        let mut best_result = 0;

        let mut slide = vec![0i32;size as usize];
        let mut running_sum = 0;
        for y in 1..=(301 - size) {
            for x in 1..=300 {
                let new_sum = (y..(y+size)).map(|y| power_at(serial, (x, y))).sum();
                running_sum -= slide[(x % size) as usize];
                running_sum += new_sum;
                slide[(x % size) as usize] = new_sum;

                if x >= size {
                    if running_sum > best_result {
                        best_result = running_sum;
                        best_coordinates = Some((x + 1 - size, y));
                    }
                }
            }
        }

        let (x, y) = best_coordinates.unwrap();

        (x, y, best_result)
    }
}

impl Solution for Day11 {
    fn part1(&mut self, input: &mut Read) -> String {
        let serial = self.read_serial(input);
        let (x, y, _) = self.best(serial, 3);

        format!("{},{}", x, y)
    }

    fn part2(&mut self, input: &mut Read) -> String {
        let serial = self.read_serial(input);
        let mut best_result = 0;
        let mut best_option = None;
        for size in 1..=300 {
            let (x, y, result) = self.best(serial, size);
            if result > best_result {
                best_result = result;
                best_option = Some((x, y, size));
            } else if result < best_result {
                break;
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
