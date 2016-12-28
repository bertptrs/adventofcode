extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::collections::VecDeque;

const INPUT: &'static str = "pxxbnzuo";

const DIRECTIONS: [char; 4] = [
    'U',
    'D',
    'L',
    'R',
];

const DELTAS: [(i32, i32); 4] = [
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
];

fn open_doors(route: &str) -> [bool; 4]
{
    let mut hasher = Md5::new();
    hasher.input_str(INPUT);
    hasher.input_str(route);

    let mut output = [0u8; 16];
    hasher.result(&mut output);

    return [
        (output[0] >> 4) > 0xa,
        (output[0] & 0xf) > 0xa,
        (output[1] >> 4) > 0xa,
        (output[1] & 0xf) > 0xa,
    ];
}

fn main() {
    let mut todo = VecDeque::new();
    todo.push_back(((0i32, 0i32), String::new()));

    let mut last = String::new();

    while !todo.is_empty() {
        let ((x, y), path) = todo.pop_front().unwrap();
        if x == 3 && y == 3 {
            if &last == "" {
                println!("Found route out! {}", path);
            }

            last = path;

            continue;
        }

        let doors = open_doors(&path);

        for (index, &(x_d, y_d)) in DELTAS.iter().enumerate() {
            if !doors[index] || x + x_d < 0 || x + x_d >= 4 || y + y_d < 0 || y + y_d >= 4 {
                continue;
            }

            let new_pos = (x + x_d, y + y_d);
            let mut new_path = path.clone();
            new_path.push(DIRECTIONS[index]);

            todo.push_back((new_pos, new_path));
        }
    }

    println!("Longest path length: {}", last.len());
}
