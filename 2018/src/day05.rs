use std::io::Read;

use common;

#[derive(Default)]
pub struct Day05 {}

impl Day05 {
    pub fn new() -> Day05 {
        Default::default()
    }

    fn reduce(mut data: Vec<u8>) -> usize {
        let mut dptr = 0;
        for iptr in 0..data.len() {
            if dptr > 0 && (data[iptr] ^ data[dptr - 1]) == 32 {
                dptr -= 1;
            } else {
                data[dptr] = data[iptr];
                dptr += 1;
            }
        }

        dptr
    }
}

impl common::Solution for Day05 {
    fn part1(&mut self, input: &mut Read) -> String {
        let mut data = Vec::new();
        input.read_to_end(&mut data).expect("Can't read input!");
        common::trim_back(&mut data);

        format!("{}", Day05::reduce(data))
    }

    fn part2(&mut self, input: &mut Read) -> String {
        let mut data = Vec::new();
        input.read_to_end(&mut data).expect("Can't read input!");
        common::trim_back(&mut data);

        let min_len = (b'a'..=b'z').map(|option| data.iter().filter(|&x| !x.eq_ignore_ascii_case(&option)).map(|&x| x).collect())
            .map(|x| Day05::reduce(x))
            .min().unwrap();

        format!("{}", min_len)
    }
}

#[cfg(test)]
mod tests {
    use common::Solution;
    use day05::Day05;

    const SAMPLE_INPUT: &[u8] = include_bytes!("samples/05");

    #[test]
    fn sample_part1() {
        let mut instance = Day05::new();
        assert_eq!("10", instance.part1(&mut SAMPLE_INPUT));
    }

    #[test]
    fn sample_part2() {
        let mut instance = Day05::new();
        assert_eq!("4", instance.part2(&mut SAMPLE_INPUT));
    }
}
