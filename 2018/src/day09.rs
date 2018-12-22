use std::collections::VecDeque;
use std::io::Read;

use common::Solution;

fn winning_marbles(elves: usize, marbles: usize) -> usize {
    let mut scores = vec![0usize; elves];
    let mut state = VecDeque::with_capacity(marbles);
    state.push_front(0);

    for marble in 1..=marbles {
        if marble % 23 == 0 {
            let player = marble % elves;

            for _ in 0..7 {
                let c = state.pop_back().unwrap();
                state.push_front(c);
            }

            let removed = state.pop_back().unwrap();
            scores[player] += removed + marble;

            let c = state.pop_front().unwrap();
            state.push_back(c);
        } else {
            let c = state.pop_front().unwrap();
            state.push_back(c);
            state.push_back(marble);
        }
    }

    *scores.iter().max().unwrap()
}

#[derive(Default)]
pub struct Day09 {}

impl Day09 {
    pub fn new() -> Self {
        Default::default()
    }

    fn read_input(input: &mut Read) -> (usize, usize) {
        let mut data = String::new();
        input.read_to_string(&mut data).unwrap();
        let mut parts = data.split(' ');
        let elves = parts.next().unwrap().parse().unwrap();
        let marbles = parts.nth(5).unwrap().parse().unwrap();

        (elves, marbles)
    }
}

impl Solution for Day09 {
    fn part1(&mut self, input: &mut Read) -> String {
        let (elves, marbles) = Day09::read_input(input);

        winning_marbles(elves, marbles).to_string()
    }

    fn part2(&mut self, input: &mut Read) -> String {
        let (elves, marbles) = Day09::read_input(input);

        winning_marbles(elves, marbles * 100).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winning_marbles() {
        assert_eq!(32, winning_marbles(9, 25));
        assert_eq!(8317, winning_marbles(10, 1618));
        assert_eq!(146373, winning_marbles(13, 7999));
        assert_eq!(2764, winning_marbles(17, 1104));
        assert_eq!(54718, winning_marbles(21, 6111));
        assert_eq!(37305, winning_marbles(30, 5807));
    }
}
