use common;
use regex;
use std::io;
use std::io::prelude::*;
use std::collections::{HashSet,HashMap,VecDeque};

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    pub elevator: usize,
    pub generators: [u8;4],
    pub chips: [u8;4],
}

impl State {
    pub fn is_valid(&self) -> bool {
        for (&generators, &chips) in self.generators.iter().zip(self.chips.iter()) {
            let matched = generators & chips;

            if (chips & !matched) != 0 && generators != 0 {
                return false;
            }
        }

        true
    }

    pub fn is_done(&self) -> bool {
        for i in 0..3usize {
            if self.generators[i] != 0 || self.chips[i] != 0 {
                return false;
            }
        }
        true
    }
}


#[derive(Default)]
pub struct Day11 {
}

impl Day11 {
    pub fn new() -> Day11 {
        Default::default()
    }

    fn add_modifications(new_states: &mut Vec<State>, cur: &State,
                         chip_modifications: u8, generator_modifications: u8) {
        if cur.elevator > 0 {
            let mut copy = cur.clone();
            copy.chips[cur.elevator] &= !chip_modifications;
            copy.chips[cur.elevator - 1] |= chip_modifications;
            copy.generators[cur.elevator] &= !generator_modifications;
            copy.generators[cur.elevator - 1] |= generator_modifications;
            copy.elevator -= 1;
            if copy.is_valid() {
                new_states.push(copy);
            }
        }
        if cur.elevator < 3 {
            let mut copy= cur.clone();
            copy.chips[cur.elevator] &= !chip_modifications;
            copy.chips[cur.elevator + 1] |= chip_modifications;
            copy.generators[cur.elevator] &= !generator_modifications;
            copy.generators[cur.elevator + 1] |= generator_modifications;
            copy.elevator += 1;
            if copy.is_valid() {
                new_states.push(copy);
            }
        }
    }

    fn read_state(input: &mut io::Read) -> State {
        let reader = io::BufReader::new(input);
        let mut state: State = Default::default();
        let mut elements = HashMap::new();

        let matcher = regex::Regex::new(r"a (\w+)(-compatible)? (microchip|generator)").unwrap();

        for (i, line) in reader.lines().enumerate() {
            let contents = line.unwrap();
            for result in matcher.captures_iter(&contents) {
                let element = result.get(1).unwrap().as_str();
                if elements.get(element) == None {
                    let new_id = elements.len();
                    elements.insert(element.to_string(), new_id);
                }
                let id = *elements.get(element).unwrap();

                let index = 1 << id;
                match result.get(3).unwrap().as_str() {
                    "microchip" => {state.chips[i] |= index},
                    "generator" => {state.generators[i] |= index},
                    _ => panic!("Invalid component type."),
                };
            }
        }

        state
    }

    fn solve(initial: State) -> String {
        let mut todo = VecDeque::new();
        let mut visited = HashSet::new();

        todo.push_back((0, initial));
        visited.insert(initial);
        assert!(initial.is_valid());

        while let Some((dist, state)) = todo.pop_front() {
            if state.is_done() {
                return format!("{}", dist);
            }
            let new_dist = dist + 1;
            let chips: u8 = state.chips[state.elevator];
            let generators: u8 = state.generators[state.elevator];
            let mut new_states = Vec::new();

            // Move two chips
            for i in 0..8u8 {
                for j in 0..=i {
                    if ((1 << i) & chips) == 0 || ((1 << j) & chips) == 0 {
                        continue;
                    }
                    let modification = (1 << i) | (1 << j);
                    Day11::add_modifications(&mut new_states, &state, modification, 0);
                }
            }
            // Move two generators
            for i in 0..8u8 {
                for j in 0..=i {
                    if ((1 << i) & generators) == 0 || ((1 << j) & generators) == 0 {
                        continue;
                    }
                    let modification = (1 << i) | (1 << j);
                    Day11::add_modifications(&mut new_states, &state, 0, modification);
                }
            }
            // Move one of each
            for i in 0..8u8 {
                for j in 0..8u8 {
                    if ((1 << i) & chips) == 0 || ((1 << j) & generators) == 0 {
                        continue;
                    }
                    Day11::add_modifications(&mut new_states, &state, 1 << i, 1 << j);
                }
            }
            for new_state in new_states {
                if !visited.contains(&new_state) {
                    visited.insert(new_state);
                    todo.push_back((new_dist, new_state));
                }
            }
        }

        unreachable!()
    }
}

impl common::Solution for Day11 {
    fn part1(&mut self, input: &mut io::Read) -> String {
        let initial = Day11::read_state(input);
        Day11::solve(initial)
    }

    fn part2(&mut self, input: &mut io::Read) -> String {
        let mut initial = Day11::read_state(input);
        // Just add the new ones as the most significant bits
        let modifier: u8 = 0b11000000;
        initial.generators[0] |= modifier;
        initial.chips[0] |= modifier;
        Day11::solve(initial)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::Solution;

    const SAMPLE: &[u8] = b"The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

    #[test]
    fn sample_part1() {
        let mut instance = Day11::new();
        assert_eq!("11", instance.part1(&mut SAMPLE));
    }
}
