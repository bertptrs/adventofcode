use std::fs::File;
use std::io::Read;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;

use aoc_2023::get_implementation;

/// Number of days we have an implementation to benchmark
const DAYS_IMPLEMENTED: u8 = 7;

fn read_input(day: u8) -> std::io::Result<Vec<u8>> {
    let input_path = format!("inputs/{day:02}.txt");

    let mut buffer = Vec::new();
    File::open(input_path)?.read_to_end(&mut buffer)?;

    Ok(buffer)
}

pub fn benchmark_days(c: &mut Criterion) {
    for day in 1..=DAYS_IMPLEMENTED {
        if let Ok(input) = read_input(day) {
            let part1 = get_implementation(day, false).unwrap();

            c.bench_with_input(BenchmarkId::new("part1", day), &input, |b, i| {
                b.iter(|| part1(i));
            });

            if day < 25 {
                let part2 = get_implementation(day, true).unwrap();

                c.bench_with_input(BenchmarkId::new("part2", day), &input, |b, i| {
                    b.iter(|| part2(i));
                });
            }
        }
    }
}

criterion_group!(benches, benchmark_days);
criterion_main!(benches);
