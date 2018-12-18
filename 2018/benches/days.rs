extern crate aoc_2018;
#[macro_use]
extern crate bencher;

use bencher::Bencher;

use aoc_2018::get_impl;

const INPUTS: &[&[u8]] = &[
    include_bytes!("../inputs/01.txt"),
    include_bytes!("../inputs/02.txt"),
];

fn test_part1(day: u32, bench: &mut Bencher) {
    bench.iter(|| {
        let input = INPUTS[day as usize - 1];
        let mut instance = get_impl(day);
        instance.part1(&mut input.as_ref())
    })
}

fn test_part2(day: u32, bench: &mut Bencher) {
    bench.iter(|| {
        let input = INPUTS[day as usize - 1];
        let mut instance = get_impl(day);
        instance.part2(&mut input.as_ref())
    })
}

fn day1_part1(bench: &mut Bencher) {
    test_part1(1, bench)
}

fn day1_part2(bench: &mut Bencher) {
    test_part2(1, bench)
}

fn day2_part1(bench: &mut Bencher) {
    test_part1(2, bench)
}

fn day2_part2(bench: &mut Bencher) {
    test_part2(2, bench)
}

benchmark_group!(day1, day1_part1, day1_part2);
benchmark_group!(day2, day2_part1, day2_part2);

benchmark_main!(day1, day2);
