use std::fs::File;
use std::io::Read;

use aoc_2021::get_implementation;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;

const DAYS_IMPLEMENTED: usize = 14;

fn read_input(day: usize) -> Vec<u8> {
    let input_path = format!("inputs/{:02}.txt", day);

    let mut buffer = Vec::new();
    File::open(input_path)
        .expect("Failed to open input file")
        .read_to_end(&mut buffer)
        .expect("Failed to read input file");

    buffer
}

pub fn benchmark_days(c: &mut Criterion) {
    for day in 1..=DAYS_IMPLEMENTED {
        let input = read_input(day);

        let part1 = get_implementation(day, false);
        let part2 = get_implementation(day, true);

        c.bench_with_input(BenchmarkId::new("part1", day), &input, |b, i| {
            b.iter(|| part1(&mut &i[..]));
        });

        c.bench_with_input(BenchmarkId::new("part2", day), &input, |b, i| {
            b.iter(|| part2(&mut &i[..]));
        });
    }
}

criterion_group!(benches, benchmark_days);
criterion_main!(benches);
