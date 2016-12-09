extern crate regex;

use regex::Regex;
use std::env;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).expect("Could not open file");

    let mut data = String::new();
    f.read_to_string(&mut data).expect("How can I not read this?");

    let content = data.trim();

    let r = Regex::new(r"\((\d+)x(\d+)\)").expect("Failed to compile regex");

    let mut len = 0;
    let mut pos = 0;

    loop {
        let todo = &content[pos..];
        match r.captures(todo) {
            Some(captures) => {
                let (start, _) = r.find(todo).unwrap();
                let (length, amount) = (captures[1].parse::<i64>().unwrap(), captures[2].parse::<i64>().unwrap());
                pos += start + captures[0].len() as usize + length as usize;
                len += start as i64 + length as i64 * amount;
            },
            None => {
                // No more markers remaining, just append the rest
                len += content.len() as i64 - pos as i64;
                break;
            },
        }
    }

    println!("Decoded text is {} long", len);

}
