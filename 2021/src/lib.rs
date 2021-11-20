use std::io::Read;

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

#[cfg(test)]
fn test_implementation(mut day: impl Solution, part: u8, mut input: &[u8], answer: impl ToString) {
    let result = match part {
        1 => day.part1(&mut input),
        2 => day.part2(&mut input),
        _ => panic!("Invalid part: {}", part),
    };

    assert_eq!(answer.to_string(), result);
}
