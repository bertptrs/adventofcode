use std::cmp::min;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::env;

fn move_pos1(pos: i32, instruction: char) -> i32
{
    match instruction {
        'U' => if pos > 3 { pos - 3 } else { pos },
        'D' => if pos < 7 { pos + 3 } else { pos },
        'L' => if pos % 3 != 1 { pos - 1 } else { pos },
        'R' => if pos % 3 != 0 { pos + 1 } else { pos },
        _   => panic!("Unsupported direction {}", instruction),
    }
}

fn row_width(y: i32) -> i32
{
    min(2 * y + 1, 9 - 2 * y)
}

fn row_offset(y: i32) -> i32
{
    (5 - row_width(y)) / 2
}

fn is_legal(pos: (i32, i32)) -> bool {
    let (x, y) = pos;
    let width = row_width(y);
    let offset = row_offset(y);

    y >= 0 && y < 5 && x >= offset && x < offset + width
}

fn move_pos2(pos: (i32, i32), instruction: char) -> (i32, i32)
{
    let (x, y) = pos;
    let new_pos = match instruction {
        'U' => (x, y - 1),
        'D' => (x, y + 1),
        'L' => (x - 1, y),
        'R' => (x + 1, y),
        _   => panic!("Unsupported direction {}", instruction),
    };

    if is_legal(new_pos) { new_pos } else { pos }
}

fn pos2char(pos: (i32, i32)) -> String
{
    let (x, y) = pos;
    let mut num = x + 1 - row_offset(y);
    for i in 0..y {
        num += row_width(i);
    }

    format!("{:X}", num)
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let f = File::open(&path).expect("Could not open file");
    let reader = BufReader::new(f);

    let mut pos1 = 5;
    let mut pos2 = (0, 2);
    let mut code1 = String::new();
    let mut code2 = String::new();
    for line in reader.lines() {
        for instruction in line.unwrap().trim().chars() {
            pos1 = move_pos1(pos1, instruction);
            pos2 = move_pos2(pos2, instruction);
        }

        code1 += &pos1.to_string();
        code2 += &pos2char(pos2);
    }

    println!("Code 1 is {}", code1);
    println!("Code 2 is {}", code2);
}
