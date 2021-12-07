use std::io::Read;

type Solution = fn(&mut dyn Read) -> String;

mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub fn get_implementation(day: usize, part2: bool) -> Solution {
    if !part2 {
        match day {
            1 => day01::part1,
            2 => day02::part1,
            3 => day03::part1,
            4 => day04::part1,
            5 => day05::part1,
            6 => day06::part1,
            7 => day07::part1,
            8 => day08::part1,
            9 => day09::part1,
            10 => day10::part1,
            11 => day11::part1,
            12 => day12::part1,
            13 => day13::part1,
            14 => day14::part1,
            15 => day15::part1,
            16 => day16::part1,
            17 => day17::part1,
            18 => day18::part1,
            19 => day19::part1,
            20 => day20::part1,
            21 => day21::part1,
            22 => day22::part1,
            23 => day23::part1,
            24 => day24::part1,
            25 => day25::part1,
            _ => panic!("Unsupported part one for day {}", day),
        }
    } else {
        match day {
            1 => day01::part2,
            2 => day02::part2,
            3 => day03::part2,
            4 => day04::part2,
            5 => day05::part2,
            6 => day06::part2,
            7 => day07::part2,
            8 => day08::part2,
            9 => day09::part2,
            10 => day10::part2,
            11 => day11::part2,
            12 => day12::part2,
            13 => day13::part2,
            14 => day14::part2,
            15 => day15::part2,
            16 => day16::part2,
            17 => day17::part2,
            18 => day18::part2,
            19 => day19::part2,
            20 => day20::part2,
            21 => day21::part2,
            22 => day22::part2,
            23 => day23::part2,
            24 => day24::part2,
            _ => panic!("Unsupported part two for day {}", day),
        }
    }
}

#[cfg(test)]
fn test_implementation(solution: Solution, data: &[u8], answer: impl ToString) {
    let result = solution(&mut data.as_ref());

    assert_eq!(answer.to_string(), result);
}
