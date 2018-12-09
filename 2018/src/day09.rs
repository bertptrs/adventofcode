use std::cell::Cell;
use std::io::Read;

use intrusive_collections::{LinkedList, LinkedListLink};

use common::Solution;

struct Marble {
    link: LinkedListLink,
    value: Cell<usize>,
}

impl Marble {
    pub fn new(value: usize) -> Box<Self> {
        Box::new(Marble {
            link: LinkedListLink::new(),
            value: Cell::new(value),
        })
    }
}

intrusive_adapter!(MarbleAdapter = Box<Marble>: Marble { link: LinkedListLink });

fn winning_marbles(elves: usize, marbles: usize) -> usize {
    let mut scores = vec![0usize; elves];
    let mut state: LinkedList<_> = LinkedList::new(MarbleAdapter::new());
    state.push_front(Marble::new(0));
    let mut current = state.front_mut();

    for marble in 1..=marbles {
        if marble % 23 == 0 {
            let player = marble % elves;
            for _ in 0..7 {
                current.move_prev();
                if current.is_null() {
                    current.move_prev();
                }
            }

            let to_remove = current.get().unwrap().value.get();

            scores[player] += marble + to_remove;
            current.remove();
        } else {
            current.move_next();
            if current.is_null() {
                current.move_next();
            }
            current.insert_after(Marble::new(marble));
            current.move_next();
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
