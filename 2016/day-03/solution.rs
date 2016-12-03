use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn is_triangle(mut triangle: Vec<i32>) -> bool
{
    triangle.sort();

    return triangle[0] + triangle[1] > triangle[2];
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).expect("Could not open file");
    let reader = BufReader::new(f);

    let mut horizontal = 0;
    let mut all_nums: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let numbers: Vec<i32> = line.unwrap()
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        for num in &numbers {
            all_nums.push(*num);
        }

        if is_triangle(numbers) {
            horizontal += 1;
        }

    }

    let mut vertical = 0;

    for i in 0..(all_nums.len() / 9) {
        let offset = i * 9;
        for j in 0..3 {
            let triangle = vec![all_nums[offset + j + 0], all_nums[offset + j + 3], all_nums[offset + j + 6]];

            if is_triangle(triangle) {
                vertical += 1;
            }
        }
    }

    println!("{} triangles possible horizontally", horizontal);
    println!("{} triangles possible vertically", vertical);
}
