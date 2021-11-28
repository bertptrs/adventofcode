use std::io::Read;

type Solution = fn(&mut dyn Read) -> String;

mod day01;

pub fn get_implementation(day: usize, part2: bool) -> Solution {
    if !part2 {
        match day {
            1 => day01::part1,
            _ => panic!("Unsupported part one for day {}", day),
        }
    } else {
        match day {
            1 => day01::part2,
            _ => panic!("Unsupported part two for day {}", day),
        }
    }
}
