use std::io::Read;

use num_integer::Integer;

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
fn target_num<T>(mut current: T, maximum: T, picked_up: &[T]) -> T
where
    T: Integer + Copy,
{
    loop {
        let next = if current.is_one() {
            maximum
        } else {
            current - T::one()
        };

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
        let target = target_num(numbers[0], 9, &numbers[1..4]);
        let target_pos = numbers.iter().position(|&c| c == target).unwrap();

        numbers[1..=target_pos].rotate_left(3);

        // Move to the new active position
        numbers.rotate_left(1);
    }

    let one_pos = numbers.iter().position(|&c| c == 1).unwrap();

    numbers.rotate_left(one_pos);

    stringify_state(&numbers[1..])
}

fn play_game_large(numbers: &[u8], to_play: usize, maximum: u32) -> Vec<u32> {
    let mut next = vec![0; maximum as usize + 1];

    // Initialize the specified numbers
    for pair in numbers.windows(2) {
        next[pair[0] as usize] = pair[1] as u32;
    }

    if numbers.len() < maximum as usize {
        // Add the remaining numbers to the loop
        next[*numbers.last().unwrap() as usize] = 10;

        for (i, pos) in next.iter_mut().enumerate().skip(10) {
            *pos = i as u32 + 1;
        }

        // Loop it around until the first spot
        next[maximum as usize] = numbers[0] as u32;
    } else {
        next[*numbers.last().unwrap() as usize] = numbers[0] as u32;
    }

    let mut active = numbers[0] as u32;

    for _ in 0..to_play {
        //print_state(active, &next);

        let picked = [
            next[active as usize],
            next[next[active as usize] as usize],
            next[next[next[active as usize] as usize] as usize],
        ];

        let target = target_num(active, maximum as u32, &picked);

        next[active as usize] = next[picked[2] as usize];
        next[picked[2] as usize] = next[target as usize];
        next[target as usize] = picked[0];

        active = next[active as usize];
    }

    next
}

#[derive(Default)]
pub struct Day23;

impl Solution for Day23 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let mut numbers = read_input(input);
        play_game(&mut numbers, 100)
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let numbers = read_input(input);

        let result = play_game_large(&numbers, 10_000_000, 1_000_000);

        let total = result[1] as u64 * result[result[1] as usize] as u64;
        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = b"389125467";

    #[test]
    fn sample_part1() {
        let sample = read_input(&mut SAMPLE.clone());
        assert_eq!("92658374", &play_game(&mut sample.clone(), 10));
        assert_eq!("67384529", &play_game(&mut sample.clone(), 100));
    }

    #[test]
    fn sample_part2() {
        test_implementation!(Day23, 2, SAMPLE, 149245887792u64);
    }
}
