use std::io::Read;

use crate::Solution;

fn read_input(input: &mut dyn Read) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(10);

    input.read_to_end(&mut buffer).unwrap();

    while let Some(&c) = buffer.last() {
        if c.is_ascii_whitespace() {
            buffer.pop();
        } else {
            break;
        }
    }

    for c in buffer.iter_mut() {
        *c -= b'0';
    }

    buffer
}

#[inline]
fn target_num(mut current: u8, picked_up: &[u8]) -> u8 {
    loop {
        let next = if current == 1 { 9 } else { current - 1 };

        if !picked_up.contains(&next) {
            return next;
        } else {
            current = next;
        }
    }
}

fn stringify_state(numbers: &[u8]) -> String {
    numbers.iter().map(|&c| (c + b'0') as char).collect()
}

fn play_game(numbers: &mut [u8], times: usize) -> String {
    assert!(numbers.len() >= 5); // Helps the compiler

    for _ in 0..times {
        let target = target_num(numbers[0], &numbers[1..4]);
        let target_pos = numbers.iter().position(|&c| c == target).unwrap();

        numbers[1..=target_pos].rotate_left(3);

        // Move to the new active position
        numbers.rotate_left(1);
    }

    let one_pos = numbers.iter().position(|&c| c == 1).unwrap();

    numbers.rotate_left(one_pos);

    stringify_state(&numbers[1..])
}

#[derive(Default)]
pub struct Day23;

impl Solution for Day23 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let mut numbers = read_input(input);
        play_game(&mut numbers, 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let sample = [3, 8, 9, 1, 2, 5, 4, 6, 7];
        assert_eq!("92658374", &play_game(&mut sample.clone(), 10));
        assert_eq!("67384529", &play_game(&mut sample.clone(), 100));
    }
}
