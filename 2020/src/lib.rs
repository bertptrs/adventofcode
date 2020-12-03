use std::io::Read;

mod common;
mod day01;
mod day02;
mod day03;
mod day04;

pub trait Solution {
    fn part1(&mut self, input: &mut dyn Read) -> String;

    fn part2(&mut self, _input: &mut dyn Read) -> String {
        unimplemented!("Still working on part 1");
    }
}

pub fn get_implementation(day: usize) -> Box<dyn Solution> {
    match day {
        1 => Box::new(day01::Day01::default()),
        2 => Box::new(day02::Day02::default()),
        3 => Box::new(day03::Day03::default()),
        _ => panic!("Unsupported day {}", day),
    }
}

#[macro_export]
macro_rules! test_implementation {
    ($impl:ident, 1, $source:ident, $output:expr) => {
        let mut implementation = $impl::default();

        let result = implementation.part1(&mut $source.as_ref());
        assert_eq!($output.to_string(), result);
    };

    ($impl:ident, 2, $source:ident, $output:expr) => {
        let mut implementation = $impl::default();

        let result = implementation.part2(&mut $source.as_ref());
        assert_eq!($output.to_string(), result);
    };
}
