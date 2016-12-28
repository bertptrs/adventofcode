use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).expect("Could not open file");
    let reader = BufReader::new(f);

    let mut ranges = Vec::new();

    // Assumption: No overlapping ranges
    for line in reader.lines() {
        let contents = line.unwrap();
        let parts: Vec<&str> = contents.split("-").collect();
        let lo: u32 = parts[0].parse().unwrap();
        let hi: u32 = parts[1].parse().unwrap();

        ranges.push((lo, hi));
    }

    ranges.sort();

    let mut min_blocked = 0;
    let mut unblocked = 0;
    for (start, end) in ranges {
        if start > min_blocked && start - min_blocked > 1 {
            if unblocked == 0 {
                println!("First non-blocked at {}", min_blocked + 1);
            }
            unblocked += start - min_blocked - 1;
        }

        if end > min_blocked {
            min_blocked = end;
        }
    }

    println!("{} allowed ips", unblocked);
}
