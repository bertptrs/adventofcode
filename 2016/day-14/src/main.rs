extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::collections::HashMap;

const INPUT: &'static str =  "ihaygndm";

fn get_hash(n: i32, sieve: &mut HashMap<i32, String>) -> String
{
    if !sieve.contains_key(&n) {
        let mut hasher = Md5::new();
        hasher.input_str(INPUT);
        hasher.input_str(&n.to_string());

        let result = String::from(hasher.result_str());
        sieve.insert(n, result);
    }

    return sieve[&n].clone();
}

fn contains_3(data: &str) -> Option<char>
{
    let mut cur = 1;
    let mut prev = '\0';
    for c in data.chars() {
        if c == prev {
            cur += 1;
            if cur == 3 {
                return Some(c);
            }
        } else {
            prev = c;
            cur = 1;
        }
    }

    return None;
}

fn get_hash2(n: i32, sieve: &mut HashMap<i32, String>) -> String
{
    if !sieve.contains_key(&n) {
        let mut cur = String::from(INPUT);
        cur += &n.to_string();
        for _ in 0..2017 {
            let mut hasher = Md5::new();
            hasher.input_str(&cur);
            cur = String::from(hasher.result_str());
        }
        sieve.insert(n, cur);
    }

    return sieve[&n].clone();
}

fn part1()
{
    let mut n = 0;
    let mut found = 0;
    let mut sieve = HashMap::new();

    loop {
        let cur = get_hash(n, &mut sieve);

        match contains_3(&cur) {
            Some(c) => {
                let search: String = (0..5).map(|_| c).collect();
                for i in 1..1001 {
                    let opt = get_hash(n + i, &mut sieve);

                    if opt.contains(&search) {
                        found += 1;
                        break;
                    }
                }

                if found == 64 {
                    println!("Found 64 keys at index {}", n);
                    return;
                }

            },
            _ => {},
        }

        n += 1;
    }
}

fn part2()
{
    let mut n = 0;
    let mut found = 0;
    let mut sieve = HashMap::new();

    loop {
        let cur = get_hash2(n, &mut sieve);

        match contains_3(&cur) {
            Some(c) => {
                let search: String = (0..5).map(|_| c).collect();
                for i in 1..1001 {
                    let opt = get_hash2(n + i, &mut sieve);

                    if opt.contains(&search) {
                        found += 1;
                        break;
                    }
                }

                if found == 64 {
                    println!("Found 64 keys at index {}", n);
                    return;
                }

            },
            _ => {},
        }

        n += 1;
    }
}


fn main() {
    part1();
    part2();
}
