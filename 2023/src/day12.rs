use std::mem;

use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;

use crate::common::parse_input;

fn number_ways(line: &[u8], groups: &[u8]) -> u64 {
    let Some(&max_group) = groups.iter().max() else {
        return 0;
    };

    let mut next = vec![vec![0; max_group as usize + 1]; groups.len() + 1];
    let mut cur = next.clone();
    cur[0][0] = 1;

    for &c in line {
        for entry in &mut next {
            entry.fill(0);
        }

        for group_pos in 0..=groups.len() {
            let group = *groups.get(group_pos).unwrap_or(&0);
            for cur_group in 0..=max_group {
                let ways = cur[group_pos][cur_group as usize];
                if ways == 0 {
                    continue;
                }

                // Either defective or maybe defective
                if c != b'.' && cur_group < group {
                    next[group_pos][cur_group as usize + 1] += ways;
                }

                if c != b'#' {
                    if cur_group == 0 {
                        next[group_pos][0] += ways;
                    } else if group == cur_group {
                        next[group_pos + 1][0] += ways;
                    }
                }
            }
        }

        mem::swap(&mut cur, &mut next);
    }

    cur[groups.len()][0] + cur[groups.len() - 1][groups[groups.len() - 1] as usize]
}

fn parse_lines(i: &[u8]) -> IResult<&[u8], Vec<(&[u8], Vec<u8>)>> {
    many1(terminated(
        separated_pair(
            take_until(" "),
            tag(" "),
            separated_list1(tag(","), nom::character::complete::u8),
        ),
        tag("\n"),
    ))(i)
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let lines = parse_input(input, parse_lines)?;

    let total: u64 = lines
        .iter()
        .map(|(line, groups)| number_ways(line, groups))
        .sum();

    Ok(total.to_string())
}

pub fn part2(input: &[u8]) -> anyhow::Result<String> {
    let lines = parse_input(input, parse_lines)?;

    let total: u64 = lines
        .iter()
        .map(|(line, groups)| {
            let line: Vec<_> = [*line; 5].join(&b"?"[..]);
            let groups = groups.repeat(5);
            number_ways(&line, &groups)
        })
        .sum();

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/12.txt");

    #[test]
    fn test_number_ways() {
        assert_eq!(1, number_ways(b"???.###", &[1, 1, 3]));
        assert_eq!(4, number_ways(b".??..??...?##.", &[1, 1, 3]));
        assert_eq!(1, number_ways(b"?#?#?#?#?#?#?#?", &[1, 3, 1, 6]));
        assert_eq!(1, number_ways(b"????.#...#...", &[4, 1, 1]));
        assert_eq!(4, number_ways(b"????.######..#####.", &[1, 6, 5]));
        assert_eq!(10, number_ways(b"?###????????", &[3, 2, 1]));
    }

    #[test]
    fn sample_part1() {
        assert_eq!("21", part1(SAMPLE).unwrap());
    }

    #[test]
    fn sample_part2() {
        assert_eq!("525152", part2(SAMPLE).unwrap());
    }
}
