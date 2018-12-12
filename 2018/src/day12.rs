use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use common::Solution;

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

fn print_state(state: &[(i32, bool)]) {
    for &(_, b) in state {
        if b {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!();
}

impl Day12 {
    pub fn new() -> Self {
        Default::default()
    }

    fn read_input(&mut self, input: &mut Read) -> Vec<(i32, bool)> {
        let mut state = Vec::new();
        let mut reader = BufReader::new(input);
        {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();

            for (idx, c) in line.trim().chars().skip("initial state: ".len()).enumerate() {
                state.push((idx as i32, char_bool(c)));
            }
        }

        for line in reader.lines() {
            let line = line.unwrap();

            if line.is_empty() {
                continue;
            }

            let mut index = 0;
            for c in line.chars().take(5) {
                index <<= 1;
                if char_bool(c) {
                    index |= 1;
                }
            }

            self.productions[index] = char_bool(line.chars().last().unwrap());
        }

        state
    }

    fn simulate(&self, state: &[(i32, bool)]) -> Vec<(i32, bool)> {
        let mut new_state = Vec::new();
        let mut index = 0;
        let mut last_idx = None;
        for &(idx, b) in state {
            index = (index << 1) & 0x1f;
            if b {
                index |= 1;
            }

            if self.productions[index] {
                new_state.push((idx - 2, true));
            } else if !new_state.is_empty() {
                new_state.push((idx - 2, false));
            }
            last_idx = Some(idx);
        }
        if let Some(idx) = last_idx {
            for i in 1..=4 {
                index = (index << 1) & 0x1f;

                if self.productions[index] {
                    new_state.push((idx + i - 2, true));
                } else if !new_state.is_empty() {
                    new_state.push((idx + i - 2, false));
                }
            }
        }

        let new_len = new_state.len() - new_state.iter().rev()
            .take_while(|&&(_, x)| !x).count();
        new_state.truncate(new_len);

        new_state
    }
}

impl Solution for Day12 {
    fn part1(&mut self, input: &mut Read) -> String {
        let mut state = self.read_input(input);
        print_state(&state);
        for _ in 1..=20 {
            state = self.simulate(&state);
        }

        let total: i32 = state.iter()
            .filter(|&&(_, x)| x)
            .map(|&(i, _)| i)
            .sum();

        format!("{}", total)
    }

    fn part2(&mut self, input: &mut Read) -> String {
        // Note: this is way too slow
        let mut state = self.read_input(input);
        print_state(&state);
        for _ in 1..=50000000000i64 {
            state = self.simulate(&state);
        }

        let total: i32 = state.iter()
            .filter(|&&(_, x)| x)
            .map(|&(i, _)| i)
            .sum();

        format!("{}", total)
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
