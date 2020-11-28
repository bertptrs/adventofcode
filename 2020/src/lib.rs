use std::io::Read;

mod common;
mod day01;

pub trait Solution {
    fn part1(&mut self, input: &mut dyn Read) -> String;

    fn part2(&mut self, _input: &mut dyn Read) -> String {
        unimplemented!("Still working on part 1");
    }
}

pub fn get_implementation(day: usize) -> Box<dyn Solution> {
    match day {
        1 => Box::new(day01::Day01::default()),
        _ => panic!("Unsupported day {}", day),
    }
}
