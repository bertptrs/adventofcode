extern crate aoc_2018;
#[macro_use]
extern crate bencher;

use std::fs::File;
use std::io::Read;

use bencher::Bencher;

use aoc_2018::get_impl;

fn get_input(day: u32) -> Vec<u8> {
    let filename = format!("inputs/{:02}.txt", day);

    let mut buf = Vec::new();
    let mut file = File::open(&filename).unwrap();

    file.read_to_end(&mut buf).unwrap();

    buf
}

fn test_part1(day: u32, bench: &mut Bencher) {
    let input = get_input(day);
    bench.iter(|| {
        let mut instance = get_impl(day);
        instance.part1(&mut input.as_slice())
    })
}

fn test_part2(day: u32, bench: &mut Bencher) {
    let input = get_input(day);
    bench.iter(|| {
        let mut instance = get_impl(day);
        instance.part2(&mut input.as_slice())
    })
}

macro_rules! day_bench {

    ( $name:ident, $day:expr ) => {
            pub mod $name {
                use super::*;
                pub fn part1(bench: & mut Bencher) {
                test_part1( $day, bench);
                }
                pub fn part2(bench: & mut Bencher) {
                test_part2( $day, bench);
                }
            }
            benchmark_group!( $name, $name::part1, $name::part2);
    };
}

day_bench!(day01, 1);
day_bench!(day02, 2);
day_bench!(day03, 3);
day_bench!(day04, 4);
day_bench!(day05, 5);
day_bench!(day06, 6);
day_bench!(day07, 7);
day_bench!(day08, 8);
day_bench!(day09, 9);
day_bench!(day10, 10);
day_bench!(day11, 11);
day_bench!(day12, 12);
day_bench!(day13, 13);
day_bench!(day14, 14);
day_bench!(day15, 15);
day_bench!(day16, 16);
day_bench!(day17, 17);
day_bench!(day18, 18);
day_bench!(day19, 19);
day_bench!(day20, 20);
day_bench!(day21, 21);
day_bench!(day22, 22);
day_bench!(day23, 23);
day_bench!(day24, 24);
day_bench!(day25, 25);

benchmark_main!(day01, day02, day03, day04, day05,
                day06, day07, day08, day09, day10,
                day11, day12, day13, day14, day15,
                day16, day17, day18, day19, day20,
                day21, day22, day23, day24, day25);
