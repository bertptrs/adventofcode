use std::collections::VecDeque;
use std::io::Read;

use crate::common::Lines;
use crate::Solution;

type Deck = VecDeque<u32>;

fn read_input(input: &mut dyn Read) -> (Deck, Deck) {
    let mut lines = Lines::new(input).skip(1);

    let mut player1 = VecDeque::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        player1.push_back(line.parse().unwrap());
    }

    let player2 = lines.skip(1).map(|l| l.parse().unwrap()).collect();

    (player1, player2)
}

fn play_recursive_game(mut p1: Deck, mut p2: Deck) -> (bool, Deck) {
    let mut c1 = p1.clone();
    let mut c2 = p2.clone();
    let mut iter = 0u32;

    while !p1.is_empty() && !p2.is_empty() {
        if iter.count_ones() == 1 {
            // Power of two
            c1 = p1.clone();
            c2 = p2.clone();
        }
        let v1 = p1.pop_front().unwrap();
        let v2 = p2.pop_front().unwrap();

        let p1_wins = if v1 as usize <= p1.len() && v2 as usize <= p2.len() {
            let p1_copy = p1.iter().take(v1 as usize).copied().collect();
            let p2_copy = p2.iter().take(v2 as usize).copied().collect();

            play_recursive_game(p1_copy, p2_copy).0
        } else {
            v1 > v2
        };

        if p1_wins {
            p1.push_back(v1);
            p1.push_back(v2);
        } else {
            p2.push_back(v2);
            p2.push_back(v1);
        }

        if p1 == c1 && p2 == c2 {
            return (true, p1);
        }

        iter += 1;
    }

    if p1.is_empty() {
        (false, p2)
    } else {
        (true, p1)
    }
}

fn score(winner: Deck) -> u32 {
    winner
        .into_iter()
        .rev()
        .enumerate()
        .map(|(pos, val)| (pos as u32 + 1) * (val as u32))
        .sum()
}

#[derive(Default)]
pub struct Day22;

impl Solution for Day22 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let (mut p1, mut p2) = read_input(input);

        while !p1.is_empty() && !p2.is_empty() {
            let v1 = p1.pop_front().unwrap();
            let v2 = p2.pop_front().unwrap();

            if v1 > v2 {
                p1.push_back(v1);
                p1.push_back(v2);
            } else {
                p2.push_back(v2);
                p2.push_back(v1);
            }
        }

        let winner = if p1.is_empty() { p2 } else { p1 };

        score(winner).to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let (p1, p2) = read_input(input);

        score(play_recursive_game(p1, p2).1).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../samples/22.txt");

    #[test]
    fn sample_part1() {
        test_implementation(Day22, 1, SAMPLE, 306);
    }

    #[test]
    fn sample_part2() {
        test_implementation(Day22, 2, SAMPLE, 291);
    }
}
