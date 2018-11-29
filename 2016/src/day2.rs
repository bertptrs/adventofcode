use std::cmp::min;
use std::io::prelude::*;
use std::io;
use common;


#[derive(Default)]
pub struct Day2 {
}

impl Day2 {

    pub fn new() -> Day2 {
        Default::default()
    }
}

impl common::Solution for Day2 {

    fn part1(&mut self, input: &mut io::Read) -> String {
        let reader = io::BufReader::new(input);
        let mut pos = 5;
        let mut code = String::new();

        for line in reader.lines() {
            for instruction in line.unwrap().trim().chars() {
                pos = move_pos1(pos, instruction);
            }
            code += &pos.to_string();
        }

        code
    }

    fn part2(&mut self, input: &mut io::Read) -> String {
        let reader = io::BufReader::new(input);
        let mut pos = (0, 2);
        let mut code = String::new();

        for line in reader.lines() {
            for instruction in line.unwrap().trim().chars() {
                pos = move_pos2(pos, instruction);
            }
            code += &pos2char(pos);
        }

        code
    }
}


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

#[cfg(test)]
mod tests {
    use super::*;
    use common::Solution;

    const SAMPLE: &str = "ULL
RRDDD
LURDL
UUUUD";

    #[test]
    fn test_part1() {
        let mut instance = Day2::new();
        assert_eq!("1985", instance.part1(&mut SAMPLE.as_bytes()));
    }

    #[test]
    fn test_part2() {
        let mut instance = Day2::new();
        assert_eq!("5DB3", instance.part2(&mut SAMPLE.as_bytes()));
    }

}
