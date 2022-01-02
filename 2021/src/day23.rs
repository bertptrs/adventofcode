use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::Read;
use std::mem::swap;

use crate::common::ordered;
use crate::common::LineIter;

type Item = (u32, u32, State);
type Todo = BinaryHeap<Reverse<Item>>;
type Visited = HashMap<State, u32>;

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
    hallway: [Option<Pod>; 11],
    rooms: [[Option<Pod>; 2]; 4],
}

fn room_hallway_pos(room: usize) -> usize {
    room * 2 + 2
}

fn hallway_room_pos(hallway: usize) -> Option<usize> {
    match hallway {
        2 => Some(0),
        4 => Some(1),
        6 => Some(2),
        8 => Some(3),
        _ => None,
    }
}

impl State {
    const VALID_HALLWAY_POS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

    pub fn is_done(&self) -> bool {
        self == &State {
            hallway: Default::default(),
            rooms: [
                [Some(Pod::A); 2],
                [Some(Pod::B); 2],
                [Some(Pod::C); 2],
                [Some(Pod::D); 2],
            ],
        }
    }

    fn add_to_queue(self, cost: u32, todo: &mut Todo, visited: &mut Visited) {
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

        // Cost to enter the hole for all pods that still need to
        let enter_estimate: u32 = self
            .rooms
            .iter()
            .enumerate()
            .map(|(index, room)| {
                let pod = match index {
                    0 => Pod::A,
                    1 => Pod::B,
                    2 => Pod::C,
                    3 => Pod::D,
                    _ => unreachable!(),
                };

                room.iter()
                    .enumerate()
                    .filter_map(|(index, &entry)| {
                        if Some(pod) != entry {
                            Some(index as u32 + 1)
                        } else {
                            None
                        }
                    })
                    .sum::<u32>()
                    * pod.cost()
            })
            .sum();

        // Cost for all of the hallway to move to above their intended rooms
        let hallway_estimate: u32 = self
            .hallway
            .iter()
            .enumerate()
            .filter_map(|(pos, &pod)| {
                let pod = pod?;

                let destination_pos = room_hallway_pos(pod as usize);

                let (a, b) = ordered(pos, destination_pos);

                Some((b - a) as u32 * pod.cost())
            })
            .sum();

        // Cost to move out of the room and above the correct rooms
        let rooms_estimate: u32 = self
            .rooms
            .iter()
            .enumerate()
            .map(|(room_index, room)| {
                let hallway_pos = room_hallway_pos(room_index);

                room.iter()
                    .enumerate()
                    .filter_map(|(room_pos, &pod)| {
                        let pod = pod?;

                        if pod as usize == room_index {
                            return None;
                        }

                        let destination_pos = room_hallway_pos(pod as usize);
                        let (a, b) = ordered(hallway_pos, destination_pos);

                        let steps = 1 + room_pos + b - a;

                        Some(steps as u32 * pod.cost())
                    })
                    .sum::<u32>()
            })
            .sum();

        enter_estimate + hallway_estimate + rooms_estimate
    }

    pub fn generate_next(&self, cost: u32, todo: &mut Todo, visited: &mut Visited) {
        self.room_to_hallway(cost, todo, visited);
        self.hallway_to_room(cost, todo, visited);
    }

    fn room_to_hallway(&self, cost: u32, todo: &mut Todo, visited: &mut Visited) {
        for (index, room) in self.rooms.iter().enumerate() {
            // Check if we even want to move anything out of this room
            if room
                .iter()
                .all(|entry| entry.map(|pod| pod as usize == index).unwrap_or(true))
            {
                continue;
            }

            let (pos, pod) = room
                .iter()
                .enumerate()
                .find_map(|(pos, entry)| entry.map(|pod| (pos, pod)))
                .unwrap(); // Safe unwrap, we know it exists from above.

            let base_cost = 1 + pos;
            let hallway_pos = room_hallway_pos(index);

            let mut queue_new = |new_pos, new_cost| {
                let mut new_state = self.clone();
                swap(
                    &mut new_state.hallway[new_pos],
                    &mut new_state.rooms[index][pos],
                );

                new_state.add_to_queue(new_cost + cost, todo, visited)
            };

            // Check positions to the left
            for new_pos in (0..hallway_pos).rev() {
                if self.hallway[new_pos].is_some() {
                    // Hit an occupied room
                    break;
                }

                if !Self::VALID_HALLWAY_POS.contains(&new_pos) {
                    // Not allowed to stop here
                    continue;
                }

                let new_cost = (base_cost + hallway_pos - new_pos) as u32 * pod.cost();
                queue_new(new_pos, new_cost);
            }

            // And to the right
            for new_pos in hallway_pos..self.hallway.len() {
                if self.hallway[new_pos].is_some() {
                    // Hit an occupied room
                    break;
                }

                if !Self::VALID_HALLWAY_POS.contains(&new_pos) {
                    // Not allowed to stop here
                    continue;
                }

                let new_cost = (base_cost + new_pos - hallway_pos) as u32 * pod.cost();
                queue_new(new_pos, new_cost);
            }
        }
    }

    fn hallway_to_room(&self, cost: u32, todo: &mut Todo, visited: &mut Visited) {
        for (pos, pod) in self
            .hallway
            .iter()
            .enumerate()
            .filter_map(|(pos, pod)| pod.map(|pod| (pos, pod)))
        {
            let mut enqueue_room = |base_cost, room_index: usize| {
                for (room_pos, entry) in self.rooms[room_index].iter().enumerate() {
                    if entry.is_some() {
                        // Occupied
                        break;
                    }

                    let new_cost = (room_pos + base_cost) as u32 * pod.cost();
                    let mut new_state = self.clone();
                    swap(
                        &mut new_state.hallway[pos],
                        &mut new_state.rooms[room_index][room_pos],
                    );

                    new_state.add_to_queue(new_cost + cost, todo, visited)
                }
            };

            // Try moving into a room to the left
            for new_hallway_pos in (2..pos).rev() {
                if self.hallway[new_hallway_pos].is_some() {
                    // Blocked
                    break;
                }

                let room_index = if let Some(room) = hallway_room_pos(new_hallway_pos) {
                    room
                } else {
                    continue;
                };

                let base_cost = pos - new_hallway_pos + 1;
                enqueue_room(base_cost, room_index);
            }

            // And the right
            for new_hallway_pos in (pos + 1)..=8 {
                if self.hallway[new_hallway_pos].is_some() {
                    // Blocked
                    break;
                }

                let room_index = if let Some(room) = hallway_room_pos(new_hallway_pos) {
                    room
                } else {
                    continue;
                };

                let base_cost = new_hallway_pos - pos + 1;
                enqueue_room(base_cost, room_index);
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
        write!(f, "#")?;

        for entry in self.hallway {
            write!(f, "{}", helper(entry))?;
        }
        writeln!(f, "#")?;

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
