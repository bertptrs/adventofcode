use std::io;
use common;
use std::collections::HashSet;
use std::error::Error;


fn dist(pos: (i32, i32)) -> i32 {
    let (x, y) = pos;

    return x.abs() + y.abs()
}

pub struct Day1 {
    pos: (i32, i32),
    dir: i32,
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1{
            pos: (0, 0),
            dir: (0),
        }
    }

    fn walk(&mut self) -> (i32, i32) {
        let backwards = if self.dir & 2 == 2 { -1 } else { 1 };
        let (x, y) = self.pos;

        self.pos = (x + backwards * (1 - (self.dir & 1)), y + backwards * (self.dir & 1));
        self.pos
    }

    fn turn(&mut self, action: &str) {
        if action == "R" {
            self.dir += 1;
        } else {
            self.dir += 3;
        }
        self.dir %= 4;
    }
}

impl common::Solution for Day1 {
    fn part1(&mut self, input: &mut io::Read) -> String {
        let mut content = String::new();
        match input.read_to_string(&mut content) {
            Err(why) => panic!("couldn't open input: {}", why.description()),
            Ok(_) => {},
        };

        for instruction in content.split(", ") {
            let turn = &instruction[..1];
            let steps: i32 = instruction[1..].trim().parse().expect("Invalid number of steps");
            self.turn(turn);

            for _ in 0..steps {
                self.walk();
            }
        }

        format!("{}", dist(self.pos))
    }

    fn part2(&mut self, input: &mut io::Read) -> String {
        let mut content = String::new();
        match input.read_to_string(&mut content) {
            Err(why) => panic!("couldn't open input: {}", why.description()),
            Ok(_) => {},
        };

        let mut positions = HashSet::new();
        positions.insert(self.pos);

        for instruction in content.split(", ") {
            let turn = &instruction[..1];
            let steps: i32 = instruction[1..].trim().parse().expect("Invalid number of steps");
            self.turn(turn);

            for _ in 0..steps {
                let pos = self.walk();
                if positions.contains(&pos) {
                    return format!("{}", dist(pos))
                } else {
                    positions.insert(pos);
                }
            }
        }

        panic!("Never visited anything twice!")
    }
}

#[cfg(test)]
mod tests {
    use super::Day1;
    use common::Solution;

    #[test]
    fn sample_part1() {
        let mut instance = Day1::new();

        assert_eq!("8", instance.part1(&mut "R8, R4, R4, R8".as_bytes()))
    }

    #[test]
    fn sample_part2() {
        let mut instance = Day1::new();

        assert_eq!("4", instance.part2(&mut "R8, R4, R4, R8".as_bytes()))
    }
}
