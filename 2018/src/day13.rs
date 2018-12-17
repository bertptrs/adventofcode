use std::cmp::Ordering;
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
    pub fn clockwise(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn counter_clockwise(self) -> Self {
        self.clockwise().clockwise().clockwise()
    }

    pub fn run(self, pos: Coordinate) -> Coordinate {
        let (x, y) = pos;
        match self {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
            Direction::East => (x + 1, y),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Cart {
    coordinate: Coordinate,
    direction: Direction,
    turns: u8,
    alive: bool,
}

impl Cart {
    pub fn new(coordinate: Coordinate, direction: Direction) -> Self {
        Cart {
            coordinate,
            direction,
            turns: 0,
            alive: true,
        }
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        let (sx, sy) = self.coordinate;
        let (ox, oy) = other.coordinate;

        (sy, sx).cmp(&(oy, ox))
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Default)]
pub struct Day13 {
    grid: Vec<Vec<char>>,
    carts: Vec<Cart>,
}

impl Day13 {
    pub fn new() -> Self {
        Default::default()
    }

    fn alive(&self) -> usize {
        self.carts.iter().filter(|x| x.alive).count()
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
                        self.carts.push(Cart::new((x, y), direction));
                    }
                    '>' | '<' => {
                        current.push('-');
                        let direction = Direction::from_char(c);
                        self.carts.push(Cart::new((x, y), direction));
                    }
                    other => current.push(other),
                }
            }
            self.grid.push(current);
        }
    }

    fn simulate(&mut self) -> Option<Coordinate> {
        let mut collision = None;
        self.carts.sort_unstable();
        for i in 0..self.carts.len() {
            if !self.carts[i].alive {
                continue;
            }
            let current_direction = self.carts[i].direction;

            self.carts[i].coordinate = current_direction.run(self.carts[i].coordinate);
            let (x, y) = self.carts[i].coordinate;
            let current_direction = self.carts[i].direction;
            self.carts[i].direction = match self.grid[y][x] {
                '|' | '-' => current_direction,
                '+' => {
                    let turns = self.carts[i].turns;
                    let new_dir = match turns {
                        0 => current_direction.counter_clockwise(),
                        1 => current_direction,
                        2 => current_direction.clockwise(),
                        _ => unreachable!(),
                    };

                    self.carts[i].turns = (turns + 1) % 3;
                    new_dir
                }
                '/' => match current_direction {
                    Direction::North | Direction::South => current_direction.clockwise(),
                    Direction::West | Direction::East => current_direction.counter_clockwise(),
                },
                '\\' => match current_direction {
                    Direction::North | Direction::South => current_direction.counter_clockwise(),
                    Direction::West | Direction::East => current_direction.clockwise(),
                },
                val => panic!("Invalid tile to be on: {}", val),
            };

            for j in 0..self.carts.len() {
                if i == j {
                    continue;
                }
                if !self.carts[i].alive {
                    break;
                }
                if !self.carts[j].alive {
                    continue;
                }

                if self.carts[i].coordinate == self.carts[j].coordinate {
                    self.carts[i].alive = false;
                    self.carts[j].alive = false;
                    collision = Some(self.carts[i].coordinate);
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

        for cart in &self.carts {
            if cart.alive {
                let (x, y) = cart.coordinate;
                return format!("{},{}", x, y);
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
