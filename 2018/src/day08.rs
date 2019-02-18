use std::io::Read;

use common::read_single_input;
use common::Solution;

#[derive(Default)]
pub struct Day08 {}

fn total1(items: &[usize]) -> (usize, usize) {
    let children = items[0];
    let meta_entries = items[1];

    let mut total = 0;
    let mut start = 2;

    for _ in 0..children {
        let (ct, cl) = total1(&items[start..items.len()]);
        start += cl;
        total += ct;
    }

    total += items[start..(start + meta_entries)].iter().sum::<usize>();

    (total, start + meta_entries)
}

fn total2(items: &[usize]) -> (usize, usize) {
    let children = items[0];
    let meta_entries = items[1];

    if children == 0 {
        (items[2..(2 + meta_entries)].iter().sum(), meta_entries + 2)
    } else {
        let mut values = Vec::with_capacity(children);

        let mut start = 2;

        for _ in 0..children {
            let (ct, cl) = total2(&items[start..items.len()]);
            start += cl;
            values.push(ct);
        }

        let total = items[start..(start + meta_entries)]
            .iter()
            .filter_map(|&x| values.get(x - 1))
            .sum();

        (total, start + meta_entries)
    }
}

impl Day08 {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Solution for Day08 {
    fn part1(&mut self, input: &mut Read) -> String {
        let data: String = read_single_input(input);

        let data: Vec<usize> = data.trim().split(' ').map(|x| x.parse().unwrap()).collect();
        let (result, _) = total1(&data);

        result.to_string()
    }

    fn part2(&mut self, input: &mut Read) -> String {
        let data: String = read_single_input(input);

        let data: Vec<usize> = data.trim().split(' ').map(|x| x.parse().unwrap()).collect();
        let (result, _) = total2(&data);

        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use common::Solution;
    use day08::Day08;

    const SAMPLE_INPUT: &[u8] = include_bytes!("samples/08.txt");

    #[test]
    fn sample_part1() {
        let mut instance = Day08::new();
        assert_eq!("138", instance.part1(&mut SAMPLE_INPUT));
    }

    #[test]
    fn sample_part2() {
        let mut instance = Day08::new();
        assert_eq!("66", instance.part2(&mut SAMPLE_INPUT));
    }
}
