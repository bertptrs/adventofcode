use std::collections::HashMap;
use std::io::Read;

use crate::common::LineIter;

#[derive(Debug)]
struct BingoCard {
    ticks: [[bool; 5]; 5],
    mapping: HashMap<u8, (u8, u8)>,
}

impl BingoCard {
    pub fn cross(&mut self, num: u8) -> bool {
        if let Some(&(x, y)) = self.mapping.get(&num) {
            self.ticks[y as usize][x as usize] = true;
            true
        } else {
            false
        }
    }

    pub fn has_won(&self) -> bool {
        // Check horizontal lines
        if self.ticks.iter().any(|s| s.iter().all(|&b| b)) {
            return true;
        }

        // Check vertical lines
        (0..5).any(|x| (0..5).all(|y| self.ticks[y][x]))

        // Diagonals do not count
    }

    pub fn remaining(&self) -> u32 {
        self.mapping
            .iter()
            .filter_map(|(&num, &(x, y))| {
                if self.ticks[y as usize][x as usize] {
                    None
                } else {
                    Some(num as u32)
                }
            })
            .sum()
    }
}

fn read_input(input: &mut dyn Read) -> (Vec<u8>, Vec<BingoCard>) {
    let mut reader = LineIter::new(input);

    let numbers: Vec<u8> = reader
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let mut bingo_cards = Vec::new();

    while reader.next().is_some() {
        let mut mapping = HashMap::new();

        for y in 0..5 {
            reader
                .next()
                .unwrap()
                .split(' ')
                .filter_map(|s| if s.is_empty() { None } else { s.parse().ok() })
                .enumerate()
                .for_each(|(x, num)| {
                    mapping.insert(num, (x as u8, y));
                });
        }

        let card = BingoCard {
            mapping,
            ticks: Default::default(),
        };

        bingo_cards.push(card);
    }

    (numbers, bingo_cards)
}

pub fn part1(input: &mut dyn Read) -> String {
    let (numbers, mut bingo_cards) = read_input(input);

    for number in numbers {
        for card in &mut bingo_cards {
            if card.cross(number) && card.has_won() {
                return (number as u32 * card.remaining()).to_string();
            }
        }
    }

    panic!("None of the cards won")
}

pub fn part2(input: &mut dyn Read) -> String {
    let (numbers, mut bingo_cards) = read_input(input);

    for number in numbers {
        bingo_cards.iter_mut().for_each(|card| {
            card.cross(number);
        });

        if bingo_cards.len() == 1 && bingo_cards[0].cross(number) && bingo_cards[0].has_won() {
            return (number as u32 * bingo_cards[0].remaining()).to_string();
        } else {
            bingo_cards.retain(|card| !card.has_won());
        }
    }

    panic!("Not all cards won!")
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/04.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 4512)
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 1924)
    }
}
