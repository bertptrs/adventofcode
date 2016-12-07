use std::collections::VecDeque;
use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn is_tls(line: &str) -> bool
{
    let mut last = VecDeque::new();
    let mut in_brackets = false;
    let mut found = false;

    for c in line.chars() {

        match c {
            '[' => {
                in_brackets = true;
                last = VecDeque::new();
            },
            ']' => {
                in_brackets = false;
                last = VecDeque::new();
            },
            _ => {
                last.push_back(c);
                if last.len() >= 4 {
                    if last[0] == last[3] && last[1] == last[2] && last[0] != last[1] {
                        found = true;
                        if in_brackets {
                            return false;
                        }
                    }
                    last.pop_front();
                }
            },
        }
    }

    return found;
}

fn is_ssl(line: &str) -> bool
{
    let parts: Vec<&str> = line.split(|c| c == '[' || c == ']').collect();

    for (i, part) in parts.iter().enumerate() {
        if i % 2 != 0 {
            continue;
        }

        let mut last = VecDeque::new();
        for c in part.chars() {
            last.push_back(c);
            if last.len() >= 3 {
                if last[0] == last[2] && last[0] != last[1] {
                    let mut identifier = String::new();
                    identifier.push(last[1]);
                    identifier.push(last[0]);
                    identifier.push(last[1]);

                    for (j, bracket) in parts.iter().enumerate() {
                        if j % 2 == 0 {
                            continue;
                        }

                        if bracket.contains(&identifier) {
                            return true;
                        }
                    }
                }

                last.pop_front();
            }
        }
    }

    return false;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).expect("Could not open file");
    let reader = BufReader::new(f);

    let mut tls = 0;
    let mut ssl = 0;
    for line in reader.lines() {
        let content = line.unwrap();
        if is_tls(&content) {
            tls += 1;
        }
        if is_ssl(&content) {
            ssl += 1;
        }
    }

    println!("{} addresses are tls", tls);
    println!("{} addresses are ssl", ssl);
}
