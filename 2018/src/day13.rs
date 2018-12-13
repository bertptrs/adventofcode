use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use common::Solution;

type Coordinate = (usize, usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn from_char(c: char) -> Self {
        match c {
            '^' => Direction::North,
            '<' => Direction::West,
            '>' => Direction::East,
            'v' => Direction::South,
            _ => panic!("Invalid direction {}", c),
        }
    }
    pub fn clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn counter_clockwise(&self) -> Self {
        return self.clockwise().clockwise().clockwise();
    }

    pub fn run(&self, pos: Coordinate) -> Coordinate {
        let (x, y) = pos;
        match self {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
            Direction::East => (x + 1, y),
        }
    }
}

#[derive(Default, Debug)]
pub struct Day13 {
    grid: Vec<Vec<char>>,
    carts: Vec<(Coordinate, Direction, usize, bool)>,
}

impl Day13 {
    pub fn new() -> Self {
        Default::default()
    }

    fn alive(&self) -> usize {
        self.carts.iter().filter(|(_, _, _, x)| *x).count()
    }

    fn read_input(&mut self, input: &mut Read) {
        let reader = BufReader::new(input);
        for (y, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let mut current = Vec::new();
            for (x, c) in line.chars().enumerate() {
                match c {
                    '^' | 'v' => {
                        current.push('|');
                        let direction = Direction::from_char(c);
                        self.carts.push(((x, y), direction, 0, true));
                    }
                    '>' | '<' => {
                        current.push('-');
                        let direction = Direction::from_char(c);
                        self.carts.push(((x, y), direction, 0, true));
                    }
                    other => current.push(other),
                }
            }
            self.grid.push(current);
        }
    }

    fn simulate(&mut self) -> Option<Coordinate> {
        let mut collision = None;
        self.carts.sort_unstable_by_key(|&((x, y), _, _ , _)| (y, x));
        for i in 0..self.carts.len() {
            {
                let (coordinate, direction, turns, alive) = self.carts.get_mut(i).unwrap();
                if !*alive {
                    continue;
                }
                *coordinate = direction.run(*coordinate);
                let &mut (x, y) = coordinate;
                *direction = match self.grid[y][x] {
                    '|' | '-' => *direction,
                    '+' => {
                        let new_dir = match turns {
                            0 => direction.counter_clockwise(),
                            1 => *direction,
                            2 => direction.clockwise(),
                            _ => unreachable!(),
                        };

                        *turns = (*turns + 1) % 3;
                        new_dir
                    },
                    '/' => match *direction {
                        Direction::North | Direction::South => direction.clockwise(),
                        Direction::West | Direction::East => direction.counter_clockwise(),
                    },
                    '\\' => match *direction {
                        Direction::North | Direction::South => direction.counter_clockwise(),
                        Direction::West | Direction::East => direction.clockwise(),
                    },
                    val => panic!("Invalid tile to be on: {}", val),
                };
            }
            for j in 0..self.carts.len() {
                if i == j {
                    continue;
                }
                if !self.carts[i].3 {
                    break;
                }
                if !self.carts[j].3 {
                    continue;
                }

                if self.carts[i].0 == self.carts[j].0 {
                    self.carts[i].3 = false;
                    self.carts[j].3 = false;
                    collision = Some(self.carts[i].0);
                }
            }
        }

        collision
    }
}

impl Solution for Day13 {
    fn part1(&mut self, input: &mut Read) -> String {
        self.read_input(input);
        let mut collision = None;
        while collision == None {
            collision = self.simulate();
        }
        let (x, y) = collision.unwrap();
        format!("{},{}", x, y)
    }

    fn part2(&mut self, input: &mut Read) -> String {
        self.read_input(input);
        while self.alive() > 1 {
            self.simulate();
        }

        for &((x, y), _, _, alive) in &self.carts {
            if alive {
                return format!("{},{}", x, y)
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use common::Solution;
    use day13::Day13;
    use day13::Direction;

    const SAMPLE1_INPUT: &[u8] = include_bytes!("samples/13.1.txt");
    const SAMPLE2_INPUT: &[u8] = include_bytes!("samples/13.2.txt");

    #[test]
    fn test_turning() {
        assert_eq!(Direction::East, Direction::South.counter_clockwise());
    }

    #[test]
    fn sample_part1() {
        let mut instance = Day13::new();
        assert_eq!("7,3", instance.part1(&mut SAMPLE1_INPUT))
    }


    #[test]
    fn sample_part2() {
        let mut instance = Day13::new();
        assert_eq!("6,4", instance.part2(&mut SAMPLE2_INPUT))
    }
}
