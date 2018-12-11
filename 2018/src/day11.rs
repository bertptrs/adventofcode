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
}

impl Solution for Day11 {
    fn part1(&mut self, input: &mut Read) -> String {
        let mut data = String::new();
        input.read_to_string(&mut data).unwrap();
        let serial = data.trim().parse().unwrap();

        let mut best_coordinates: Option<(i32, i32)> = None;
        let mut best_result = 0;

        for y in 1..=298 {
            let mut running_sum = 0;
            let mut slide = [0i32;3];

            for x in 1..298 {
                let new_sum = (y..=(y+2)).map(|y| power_at(serial, (x, y))).sum();
                running_sum += new_sum;

                if x >= 3 {
                    running_sum -= slide[(x as usize) % 3];
                    if running_sum > best_result {
                        best_result = running_sum;
                        best_coordinates = Some((x - 2, y));
                    }
                }

                slide[(x as usize) % 3] = new_sum;
            }
        }

        let (x, y) = best_coordinates.unwrap();

        format!("{},{}", x, y)
    }

    fn part2(&mut self, _input: &mut Read) -> String {
        unimplemented!()
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
}
