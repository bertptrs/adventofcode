#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::env;
use std::io::prelude::*;
use std::fs::File;

fn decoded_len(content: &str, recurse: bool) -> i64
{
    lazy_static!{
        static ref R: Regex = Regex::new(r"\((\d+)x(\d+)\)").expect("Failed to compile regex");
    }

    let mut len = 0;
    let mut pos = 0;

    loop {
        let todo = &content[pos..];
        match R.captures(todo) {
            Some(captures) => {
                let start = todo.find(&captures[0]).unwrap();
                pos += start;
                len += start as i64;

                let (length, amount) = (captures[1].parse::<i64>().unwrap(), captures[2].parse::<i64>().unwrap());

                let part = &todo[start + captures[0].len()..start + captures[0].len() + length as usize];

                let part_length = if recurse { decoded_len(part, true) } else { length };

                pos += captures[0].len() as usize + length as usize;
                len += part_length * amount;
            },
            None => {
                // No more markers remaining, just append the rest
                len += content.len() as i64 - pos as i64;
                break;
            },
        }
    }

    return len;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).expect("Could not open file");

    let mut data = String::new();
    f.read_to_string(&mut data).expect("How can I not read this?");

    let content = data.trim();

    println!("Decoded v1 is {} long", decoded_len(content, false));
    println!("Decoded v2 is {} long", decoded_len(content, true));

}
