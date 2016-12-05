extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn find_hash(word: &str, mut start: i32) -> (char, char, i32)
{
    let mut hasher = Md5::new();
    loop {
        let num = start.to_string();
        hasher.reset();
        hasher.input_str(word);
        hasher.input_str(&num);

        let hash = hasher.result_str();
        if &hash[0..5] == "00000" {
            return (hash.chars().nth(5).unwrap(), hash.chars().nth(6).unwrap(), start);
        }

        start += 1;
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

    let mut start = 0;
    let mut password = ['-'; 8];
    let mut used = [false; 8];

    let mut found = 0;
    let mut printed = 0;

    while found < 8 {
        let (c1, c2, num) = find_hash(input, start);
        start = num + 1;

        if valid_pos(c1) {
            let p = pos(c1) as usize;
            if !used[p] {
                found += 1;
                password[p] = c2;
                used[p] = true;
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
