use anyhow::Result;

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

type Solution = fn(&[u8]) -> Result<String>;

pub fn get_implementation(day: u8, part2: bool) -> Result<Solution> {
    if !part2 {
        match day {
            1 => Ok(day01::part1),
            2 => Ok(day02::part1),
            3 => Ok(day03::part1),
            4 => Ok(day04::part1),
            5 => Ok(day05::part1),
            6 => Ok(day06::part1),
            7 => Ok(day07::part1),
            8 => Ok(day08::part1),
            9 => Ok(day09::part1),
            10 => Ok(day10::part1),
            11 => Ok(day11::part1),
            12 => Ok(day12::part1),
            13 => Ok(day13::part1),
            14 => Ok(day14::part1),
            15 => Ok(day15::part1),
            16 => Ok(day16::part1),
            17 => Ok(day17::part1),
            18 => Ok(day18::part1),
            19 => Ok(day19::part1),
            20 => Ok(day20::part1),
            21 => Ok(day21::part1),
            22 => Ok(day22::part1),
            23 => Ok(day23::part1),
            24 => Ok(day24::part1),
            25 => Ok(day25::part1),
            _ => anyhow::bail!("Invalid day for part 1: {day}"),
        }
    } else {
        match day {
            1 => Ok(day01::part2),
            2 => Ok(day02::part2),
            3 => Ok(day03::part2),
            4 => Ok(day04::part2),
            5 => Ok(day05::part2),
            6 => Ok(day06::part2),
            7 => Ok(day07::part2),
            8 => Ok(day08::part2),
            9 => Ok(day09::part2),
            10 => Ok(day10::part2),
            11 => Ok(day11::part2),
            12 => Ok(day12::part2),
            13 => Ok(day13::part2),
            14 => Ok(day14::part2),
            15 => Ok(day15::part2),
            16 => Ok(day16::part2),
            17 => Ok(day17::part2),
            18 => Ok(day18::part2),
            19 => Ok(day19::part2),
            20 => Ok(day20::part2),
            21 => Ok(day21::part2),
            22 => Ok(day22::part2),
            23 => Ok(day23::part2),
            24 => Ok(day24::part2),
            _ => anyhow::bail!("Invalid day for part 2: {day}"),
        }
    }
}
