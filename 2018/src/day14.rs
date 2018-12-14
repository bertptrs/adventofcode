use std::io::Read;

use common::Solution;
use common::read_single_input;

fn skill_after(n: usize) -> u64 {
    let mut state = vec![3u8, 7];
    let mut elves = [0, 1];

    while state.len() < n + 10 {
        let result = state[elves[0]] + state[elves[1]];
        if result >= 10 {
            state.push(result / 10);
        }
        state.push(result % 10);

        elves[0] = (elves[0] + state[elves[0]] as usize + 1) % state.len();
        elves[1] = (elves[1] + state[elves[1]] as usize + 1) % state.len();
    }

    let mut skill = 0;
    for d in state.into_iter().skip(n).take(10) {
        skill *= 10;
        skill += d as u64;
    }

    skill
}

fn update_current(mut current: usize, by: usize, base: usize) -> usize {
    current *= 10;
    current %= base;
    current += by;
    current
}

fn find_first(n: usize, len: usize) -> usize {
    let mut state = vec![3u8, 7];
    let mut elves = [0, 1];
    let mut current = 37;
    let mod_base = 10usize.pow(len as u32);

    loop {
        let result = state[elves[0]] + state[elves[1]];
        if result >= 10 {
            current = update_current(current, result as usize / 10, mod_base);
            if current == n {
                return state.len() - len + 1;
            }
            state.push(result / 10);
        }
        current = update_current(current, result as usize % 10, mod_base);
        if current == n {
            return state.len() - len + 1;
        }
        state.push(result % 10);

        elves[0] = (elves[0] + state[elves[0]] as usize + 1) % state.len();
        elves[1] = (elves[1] + state[elves[1]] as usize + 1) % state.len();
    }
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
