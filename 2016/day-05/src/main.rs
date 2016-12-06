extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;


fn five_zeroes(a: u8, b: u8, c: u8) -> bool
{
    (a | b | (c >> 4)) == 0
}

struct Hasher {
    index: i32,
    base: String,
    hasher: Md5,
}

impl Hasher {
    fn new(start: &str) -> Hasher { Hasher {index: 0, base: String::from(start), hasher: Md5::new() } }
}

impl Iterator for Hasher {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut digest = [0u8; 16];
        loop {
            let num = self.index.to_string();
            self.index += 1;
            self.hasher.reset();

            self.hasher.input_str(&self.base);
            self.hasher.input_str(&num);

            self.hasher.result(&mut digest);

            if five_zeroes(digest[0], digest[1], digest[2]) {
                return Some(self.hasher.result_str());
            }
        }
    }
}

fn valid_pos(c: char) -> bool
{
    '0' <= c && c <= '7'
}

fn pos(c: char) -> i32
{
    ((c as u8) - ('0' as u8)) as i32
}

fn main() {
    let input = "cxdnnyjw";

    let mut password = ['-'; 8];
    let mut used = [false; 8];

    let mut found = 0;
    let mut printed = 0;

    for hash in Hasher::new(input) {
        let c1 = hash.chars().nth(5).unwrap();
        let c2 = hash.chars().nth(6).unwrap();

        if valid_pos(c1) {
            let p = pos(c1) as usize;
            if !used[p] {
                found += 1;
                password[p] = c2;
                used[p] = true;

                if found >= 8 {
                    break;
                }
            }
        }

        if printed < 8 {
            print!("{}", c1);
            printed += 1;
        }
    }

    let result: String = password.iter().cloned().collect();
    println!("\n{}", result);
}
