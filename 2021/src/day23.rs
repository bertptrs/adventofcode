use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::Read;
use std::mem::swap;

use crate::common::LineIter;

type Item = (u32, u32, State);
type Todo = BinaryHeap<Reverse<Item>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
enum Pod {
    A,
    B,
    C,
    D,
}

impl Pod {
    pub fn cost(self) -> u32 {
        match self {
            Pod::A => 1,
            Pod::B => 10,
            Pod::C => 100,
            Pod::D => 1000,
        }
    }
}

impl TryFrom<char> for Pod {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Pod::A),
            'B' => Ok(Pod::B),
            'C' => Ok(Pod::C),
            'D' => Ok(Pod::D),
            _ => Err("Invalid pod"),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct State {
    hallway: [Option<Pod>; 7],
    rooms: [[Option<Pod>; 2]; 4],
}

impl State {
    pub fn is_done(&self) -> bool {
        self == &State {
            hallway: [None; 7],
            rooms: [
                [Some(Pod::A); 2],
                [Some(Pod::B); 2],
                [Some(Pod::C); 2],
                [Some(Pod::D); 2],
            ],
        }
    }

    fn add_to_queue(self, cost: u32, todo: &mut Todo, visited: &mut HashMap<Self, u32>) {
        let entry = visited.entry(self.clone());

        if matches!(&entry, Entry::Occupied(entry) if *entry.get() <= cost) {
            // Already got a better one
            return;
        }

        // print!("Next: \n{}", self);

        // nightly only :'(
        // entry.insert(cost);
        *entry.or_default() = cost;

        todo.push(Reverse((cost + self.estimate(), cost, self)))
    }

    fn estimate(&self) -> u32 {
        // A* estimate. For every entry that is not already "at rest", the cost is the cost
        // required to get it to the top of its intended room.
        let mut estimate = 0;

        for (x, &pod) in self.hallway.iter().enumerate() {
            if let Some(pod) = pod {
                let cost = if x == 0 {
                    4 + pod as u32 * 2
                } else if x == 6 {
                    4 + (3 - pod as u32) * 2
                } else if x <= (pod as usize) + 1 {
                    2 + 2 * (pod as u32 + (x as u32 - 1))
                } else {
                    2 + 2 * (x as u32 - pod as u32 - 2)
                };
                estimate += cost * pod.cost();
            }
        }

        for (index, room) in self.rooms.iter().enumerate() {
            if let Some(last) = room
                .iter()
                .rposition(|&pod| !matches!(pod, Some(pod) if pod as usize == index))
            {
                for pos in 0..=last {
                    if let Some(pod) = room[pos] {
                        if pod as usize != index {
                            let abs_diff = index.max(pod as usize) - index.min(pod as usize);
                            estimate += (pos + 2 + 2 * abs_diff) as u32 * pod.cost();
                        }
                    }
                }
            }
        }

        estimate
    }

    pub fn generate_next(&self, cost: u32, todo: &mut Todo, visited: &mut HashMap<Self, u32>) {
        self.generate_hallway(cost, todo, visited);
        self.generate_rooms(cost, todo, visited);
    }

    fn generate_rooms(&self, cost: u32, todo: &mut Todo, visited: &mut HashMap<Self, u32>) {
        for (index, room) in self.rooms.iter().enumerate() {
            // Check what part of the room should still move
            if let Some(last) = room
                .iter()
                .rposition(|&pod| !matches!(pod, Some(pod) if pod as usize == index))
            {
                for pos in 0..=last {
                    let pod = match room[pos] {
                        Some(pod) => pod,
                        None => continue,
                    };

                    // Check if we can move up
                    if pos > 0 && room[pos - 1].is_none() {
                        let mut new_state = self.clone();
                        new_state.rooms[index].swap(pos, pos - 1);
                        let new_cost = cost + pod.cost();
                        new_state.add_to_queue(new_cost, todo, visited);
                    }

                    // Check if we can move down
                    if pos + 1 < room.len() && room[pos + 1].is_none() {
                        let mut new_state = self.clone();
                        new_state.rooms[index].swap(pos, pos + 1);
                        let new_cost = cost + pod.cost();
                        new_state.add_to_queue(new_cost, todo, visited);
                    }
                }

                // Check if we can pop out of the room
                if let Some(pod) = room[0] {
                    for pos in [index + 1, index + 2] {
                        if self.hallway[pos].is_none() {
                            let mut new_state = self.clone();
                            swap(&mut new_state.rooms[index][0], &mut new_state.hallway[pos]);
                            let new_cost = cost + pod.cost();
                            new_state.add_to_queue(new_cost, todo, visited);
                        }
                    }
                }
            }
        }
    }

    fn generate_hallway(&self, cost: u32, todo: &mut Todo, visited: &mut HashMap<Self, u32>) {
        for index in 0..self.hallway.len() {
            let pod = if let Some(pod) = self.hallway[index] {
                pod
            } else {
                continue;
            };

            // Check if we can move right
            if index + 1 < self.hallway.len() && self.hallway[index + 1].is_none() {
                let mut new_state = self.clone();
                new_state.hallway.swap(index, index + 1);
                let added_cost = if index == 0 || index == 5 {
                    pod.cost()
                } else {
                    2 * pod.cost()
                };

                let new_cost = cost + added_cost;
                new_state.add_to_queue(new_cost, todo, visited);
            }

            // Check if we can move left
            if index > 1 && self.hallway[index - 1].is_none() {
                let mut new_state = self.clone();
                new_state.hallway.swap(index, index - 1);
                let added_cost = if index == 1 || index == 6 {
                    pod.cost()
                } else {
                    2 * pod.cost()
                };

                let new_cost = cost + added_cost;
                new_state.add_to_queue(new_cost, todo, visited);
            }

            // Check if we can pop into a room to the right
            if (1..=4).contains(&index) && self.rooms[index - 1][0].is_none() {
                let mut new_state = self.clone();
                swap(
                    &mut new_state.hallway[index],
                    &mut new_state.rooms[index - 1][0],
                );

                let new_cost = cost + 2 * pod.cost();
                new_state.add_to_queue(new_cost, todo, visited);
            }

            if (2..=5).contains(&index) && self.rooms[index - 2][0].is_none() {
                let mut new_state = self.clone();
                swap(
                    &mut new_state.hallway[index],
                    &mut new_state.rooms[index - 2][0],
                );

                let new_cost = cost + 2 * pod.cost();
                new_state.add_to_queue(new_cost, todo, visited);
            }
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let helper = |opt_pod| match opt_pod {
            Some(Pod::A) => 'A',
            Some(Pod::B) => 'B',
            Some(Pod::C) => 'C',
            Some(Pod::D) => 'D',
            None => '.',
        };
        writeln!(f, "#############")?;
        write!(f, "#{}{}", helper(self.hallway[0]), helper(self.hallway[1]))?;
        for i in 2..=5 {
            write!(f, ".{}", helper(self.hallway[i]))?;
        }
        writeln!(f, "{}#", helper(self.hallway[6]))?;

        for i in 0..(self.rooms[0].len()) {
            writeln!(
                f,
                "  #{}#{}#{}#{}#",
                helper(self.rooms[0][i]),
                helper(self.rooms[1][i]),
                helper(self.rooms[2][i]),
                helper(self.rooms[3][i])
            )?;
        }

        write!(f, "  #########")
    }
}

fn read_input(input: &mut dyn Read) -> State {
    let mut reader = LineIter::new(input);
    let mut state = State::default();

    let _ = reader.next();
    let _ = reader.next();

    let mut helper = |idx: usize| {
        reader
            .next()
            .unwrap()
            .chars()
            .filter_map(|c| Pod::try_from(c).ok())
            .zip(&mut state.rooms)
            .for_each(|(pod, room)| room[idx] = Some(pod))
    };

    helper(0);
    helper(1);

    state
}

pub fn part1(input: &mut dyn Read) -> String {
    let state = read_input(input);
    let mut todo = Todo::new();

    let mut visited = HashMap::new();
    visited.insert(state.clone(), 0);

    todo.push(Reverse((state.estimate(), 0, state)));

    while let Some(Reverse((_, cost, state))) = todo.pop() {
        if state.is_done() {
            return cost.to_string();
        }

        // println!("\nExpanding:\n{}", state);

        state.generate_next(cost, &mut todo, &mut visited);
    }

    panic!("No route found!")
}

pub fn part2(_input: &mut dyn Read) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_implementation;

    const SAMPLE: &[u8] = include_bytes!("samples/23.txt");

    #[test]
    fn test_is_done() {
        let state = State {
            hallway: Default::default(),
            rooms: [
                [Some(Pod::A); 2],
                [Some(Pod::B), Some(Pod::B)],
                [Some(Pod::C), Some(Pod::C)],
                [Some(Pod::D), Some(Pod::D)],
            ],
        };

        assert!(state.is_done());
    }

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 12521);
    }
}
