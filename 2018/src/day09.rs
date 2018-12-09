use common::Solution;
use std::io::Read;

fn winning_marbles(elves: usize, marbles: usize) -> usize {
    let mut scores = vec![0usize; elves];
    let mut state = Vec::new();
    state.push(0);
    let mut current = 0;

    for marble in 1..=marbles {
        if marble % 23 == 0 {
            let player = marble % elves;
            for _ in 0..7 {}
            let to_remove = (current + state.len() - 7) % state.len();

            scores[player] += marble + state[to_remove];

            state.remove(to_remove);
            current = to_remove;
        } else {
            let after = (current + 1) % state.len();
            if after == state.len() - 1 {
                state.push(marble);
            } else {
                state.insert(after + 1, marble);
            }
            current = after + 1;
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
        let mut parts = data.split(" ");
        let elves = parts.next().unwrap().parse().unwrap();
        let marbles = parts.skip(5).next().unwrap().parse().unwrap();

        (elves, marbles)
    }
}

impl Solution for Day09 {
    fn part1(&mut self, input: &mut Read) -> String {
        let (elves, marbles) = Day09::read_input(input);

        format!("{}", winning_marbles(elves, marbles))
    }

    fn part2(&mut self, input: &mut Read) -> String {
        let (elves, marbles) = Day09::read_input(input);

        format!("{}", winning_marbles(elves, marbles * 100))
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
