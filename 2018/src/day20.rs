use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::Read;

use common::Solution;

type Coordinate = (i32, i32);

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn walk(self, (x, y): Coordinate) -> Coordinate {
        use self::Direction::*;
        match self {
            North => (x, y - 1),
            West => (x - 1, y),
            South => (x, y + 1),
            East => (x + 1, y),
        }
    }
}

impl From<usize> for Direction {
    fn from(idx: usize) -> Self {
        use self::Direction::*;
        match idx {
            0 => North,
            1 => South,
            2 => West,
            3 => East,
            val => panic!("Invalid direction: {}", val),
        }
    }
}

impl From<u8> for Direction {
    fn from(b: u8) -> Self {
        use self::Direction::*;
        match b {
            b'N' => North,
            b'S' => South,
            b'W' => West,
            b'E' => East,
            val => panic!("Invalid direction: {}", val),
        }
    }
}

#[derive(Default)]
pub struct Day20 {
    paths: HashMap<Coordinate, [bool; 4]>,
}

impl Day20 {
    pub fn new() -> Self {
        Default::default()
    }

    fn follow_directions(
        &mut self,
        initial: &[Coordinate],
        data: &[u8],
    ) -> (Vec<Coordinate>, usize) {
        let mut pos = Vec::from(initial);
        let mut positions = Vec::new();

        let mut index = 0;

        while index < data.len() {
            let b = data[index];
            match b {
                b'|' => {
                    positions.extend_from_slice(&pos);
                    pos = Vec::from(initial);
                }
                b'$' | b')' => {
                    positions.extend_from_slice(&pos);
                    positions.sort_unstable();
                    positions.dedup();
                    return (positions, index);
                }

                b'(' => {
                    let (new_pos, read) = self.follow_directions(&pos, &data[index + 1..]);
                    pos = new_pos;
                    index += read + 1;
                }

                b'N' | b'S' | b'W' | b'E' => {
                    let dir = Direction::from(b);
                    for p in pos.iter_mut() {
                        let entry = self.paths.entry(*p).or_insert([false; 4]);
                        entry[dir as usize] = true;
                        *p = dir.walk(*p);
                    }
                }
                val => panic!("Invalid input character: {}", val),
            }
            index += 1;
        }
        unreachable!();
    }

    fn distances(&self) -> HashMap<Coordinate, usize> {
        let mut todo = VecDeque::new();
        let mut visited = HashMap::new();
        todo.push_back((0, (0, 0)));
        visited.insert((0, 0), 0);

        while let Some((dist, pos)) = todo.pop_front() {
            if let Some(dirs) = self.paths.get(&pos) {
                let dirs = dirs.iter().enumerate().filter_map(|(idx, state)| {
                    if *state {
                        Some(Direction::from(idx))
                    } else {
                        None
                    }
                });
                for dir in dirs {
                    let new_pos = dir.walk(pos);
                    if let Entry::Vacant(entry) = visited.entry(new_pos) {
                        entry.insert(dist + 1);
                        todo.push_back((dist + 1, new_pos));
                    }
                }
            }
        }

        visited
    }
}

impl Solution for Day20 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let mut data = Vec::new();
        input.read_to_end(&mut data).unwrap();
        let pos = (0, 0);

        self.follow_directions(&[pos], &data[1..]);
        self.distances().values().max().unwrap().to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let mut data = Vec::new();
        input.read_to_end(&mut data).unwrap();
        let pos = (0, 0);

        self.follow_directions(&[pos], &data[1..]);
        self.distances()
            .values()
            .filter(|&&x| x >= 1000)
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use common::Solution;

    use super::*;

    #[test]
    fn sane_enums() {
        for idx in 0..4 {
            let direction = Direction::from(idx);
            let back = direction as usize;
            assert_eq!(idx, back);
        }
    }

    #[test]
    fn sample_part1() {
        let samples: [&[u8]; 5] = [
            b"^WNE$",
            b"^ENWWW(NEEE|SSE(EE|N))$",
            b"^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$",
            b"^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$",
            b"^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$",
        ];

        let outputs = ["3", "10", "18", "23", "31"];

        for (input, output) in samples.iter().zip(outputs.iter()) {
            println!("{}", String::from_utf8_lossy(input));
            let mut instance = Day20::new();
            assert_eq!(*output, instance.part1(&mut input.as_ref()));
        }
    }
}
