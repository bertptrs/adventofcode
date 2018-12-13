use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::iter::FromIterator;

use common::Solution;

type State = Vec<(i64, bool)>;

#[derive(Default, Debug)]
pub struct Day12 {
    productions: [bool; 32],
}

fn char_bool(c: char) -> bool {
    match c {
        '#' => true,
        '.' => false,
        _ => panic!("Invalid input {}", c)
    }
}

fn print_state(state: &State) -> String {
    state.iter().map(|(_, x)| if *x { '#' } else { '.' }).collect()
}

fn state_from_string(representation: &str, offset: i64) -> State {
    representation.chars().enumerate()
        .map(|(i, c)| (i as i64 + offset, char_bool(c)))
        .collect()
}

impl Day12 {
    pub fn new() -> Self {
        Default::default()
    }

    fn read_input(&mut self, input: &mut Read) -> State {
        let state;
        let mut reader = BufReader::new(input);
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        state = state_from_string(&line["initial state:".len()..line.len()].trim(), 0);

        for line in reader.lines() {
            let line = line.unwrap();

            if line.is_empty() {
                continue;
            }

            let mut index = 0;
            for c in line.chars().take(5) {
                index = (index << 1) | char_bool(c) as usize;
            }

            self.productions[index] = char_bool(line.chars().last().unwrap());
        }

        state
    }

    fn simulate(&self, state: &State) -> State {
        let mut new_state = Vec::with_capacity(state.len() + 8);
        let mut index = 0;
        for &(idx, b) in state {
            index = self.advance_state(&mut new_state, index, idx, b)
        }
        if let Some((idx, _)) = state.last() {
            for i in 1..=4 {
                index = self.advance_state(&mut new_state, index, idx + i, false);
            }
        }

        let new_len = new_state.len() - new_state.iter().rev()
            .take_while(|&&(_, x)| !x).count();
        new_state.truncate(new_len);

        new_state
    }

    fn advance_state(&self, new_state: &mut State, mut index: usize, idx: i64, b: bool) -> usize {
        index = (index << 1) & 0x1f | b as usize;
        if self.productions[index] {
            new_state.push((idx - 2, true));
        } else if !new_state.is_empty() {
            new_state.push((idx - 2, false));
        }

        index
    }

    fn simulate_n(&self, mut state: State, n: usize) -> State {
        for _ in 0..n {
            state = self.simulate(&state);
        }
        state
    }

    fn sum(&self, state: &State) -> i64 {
        state.iter()
            .filter(|&&(_, x)| x)
            .map(|&(i, _)| i)
            .sum()
    }
}

impl Solution for Day12 {
    fn part1(&mut self, input: &mut Read) -> String {
        let mut state = self.read_input(input);
        state = self.simulate_n(state, 20);

        format!("{}", self.sum(&state))
    }

    fn part2(&mut self, input: &mut Read) -> String {
        // Note: this is way too slow
        let mut state = self.read_input(input);
        let mut seen = HashMap::new();
        let mut time = 1i64;
        const TARGET_TIME: i64 = 50000000000;

        loop {
            state = self.simulate(&state);
            let &(offset, _) = state.first().unwrap();
            let representation = print_state(&state);
            if let Some((o_offset, o_time)) = seen.get(&representation) {
                let remaining_time = TARGET_TIME - o_time;
                let cycle_length = time - o_time;
                let iterations_skipped = remaining_time / cycle_length;
                let remainder = remaining_time % cycle_length;
                let new_offset = o_offset + iterations_skipped * (offset - o_offset);

                state = state_from_string(&representation, new_offset);
                state = self.simulate_n(state, remainder as usize);

                return format!("{}", self.sum(&state));
            }
            seen.insert(representation, (offset, time));
            time += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use common::Solution;
    use day12::Day12;

    const SAMPLE_INPUT: &[u8] = include_bytes!("samples/12.txt");

    #[test]
    fn sample_part1() {
        let mut instance = Day12::new();
        assert_eq!("325", instance.part1(&mut SAMPLE_INPUT));
    }
}
