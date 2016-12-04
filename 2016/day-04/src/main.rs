extern crate regex;

use regex::Regex;
use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

fn is_valid(name: &str, checksum: &str) -> bool
{
    let mut freqs: HashMap<char, i32> = HashMap::new();
    for c in name.chars()
    {
        if c == '-' {
            continue;
        }

        if freqs.contains_key(&c) {
            let curval = freqs[&c];
            freqs.insert(c, curval + 1);
        } else {
            freqs.insert(c, 1);
        }
    }

    let mut order: Vec<(i32, char)> = Vec::new();
    for item in freqs {
        let (c, count) = item;
        let record = (-count, c);
        order.push(record);
    }
    order.sort();

    let mut code = String::new();
    for item in order {
        let (_, c) = item;
        code.push(c);
    }

    checksum == &code[0..5]
}

fn shift_char(c: char) -> char
{
    match c {
        'z' => 'a',
        '-' => ' ',
        ' ' => ' ',
        _ => ((c as u8) + 1) as char,
    }
}

fn shift_times(mut c: char, times: i32) -> char
{
    for _ in 0..(times % 26) {
        c = shift_char(c);
    }

    return c;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).expect("Could not open file");
    let reader = BufReader::new(f);

    let room_pattern = Regex::new(r"((\w+-)+)(\d+)\[(\w+)\]").unwrap();

    let mut cur_sum = 0;

    for line in reader.lines() {
        let door_label = line.unwrap();
        let caps = room_pattern.captures(&door_label).unwrap();

        let name = caps.at(1).unwrap();
        let checksum = caps.at(4).unwrap();
        if is_valid(name, checksum) {
            let sector_id = caps.at(3).unwrap().parse().unwrap();
            cur_sum += sector_id;

            let decoded: String = name.chars()
                .map(|c| shift_times(c, sector_id))
                .collect();

            if decoded.contains("northpole object storage") {
                println!("Objects are in sector {}", sector_id);
            }
        }
    }

    println!("Sum is {}", cur_sum);
}
