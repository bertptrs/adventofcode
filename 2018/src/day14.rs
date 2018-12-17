use std::io::Read;

use common::read_single_input;
use common::Solution;

struct ResultStream {
    elves: [usize; 2],
    state: Vec<u8>,
    next: usize,
}

impl ResultStream {
    pub fn new() -> Self {
        ResultStream {
            elves: [0, 1],
            state: vec![3, 7],
            next: 0,
        }
    }
}

impl Iterator for ResultStream {
    type Item = u8;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let state = &mut self.state;
        if self.next == state.len() {
            let elves = &mut self.elves;
            let n = state[elves[0]] + state[elves[1]];
            if n >= 10 {
                state.push(1);
            }
            state.push(n % 10);

            elves[0] = (elves[0] + 1 + state[elves[0]] as usize) % state.len();
            elves[1] = (elves[1] + 1 + state[elves[1]] as usize) % state.len();
        };
        let n = Some(state[self.next]);
        self.next += 1;
        n
    }
}

fn skill_after(n: usize) -> u64 {
    let state = ResultStream::new();
    let mut skill = 0;
    for d in state.skip(n).take(10) {
        skill *= 10;
        skill += u64::from(d);
    }

    skill
}

fn find_first(n: usize, len: usize) -> usize {
    let mod_base = 10usize.pow(len as u32);
    let mut current = 0;

    for (i, b) in ResultStream::new().enumerate() {
        current = (current * 10) % mod_base;
        current += b as usize;
        if current == n {
            return i - len + 1;
        }
    }
    unreachable!();
}


#[derive(Default)]
pub struct Day14 {}

impl Day14 {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Solution for Day14 {
    fn part1(&mut self, input: &mut Read) -> String {
        let input = read_single_input(input);
        format!("{:010}", skill_after(input))
    }

    fn part2(&mut self, input: &mut Read) -> String {
        let mut buf = String::new();
        input.read_to_string(&mut buf).unwrap();

        let input = buf.trim().parse().unwrap();
        format!("{}", find_first(input, buf.trim().len()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_after() {
        assert_eq!(5158916779, skill_after(9));
        assert_eq!(124515891, skill_after(5));
        assert_eq!(9251071085, skill_after(18));
        assert_eq!(5941429882, skill_after(2018));
    }

    #[test]
    fn test_find_first() {
        assert_eq!(9, find_first(51589, 5));
        assert_eq!(5, find_first(1245, 5));
        assert_eq!(18, find_first(92510, 5));
        assert_eq!(2018, find_first(59414, 5));
    }
}
