use anyhow::Result;
use itertools::Itertools;

fn priority(item: u8) -> u32 {
    match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => 0,
    }
    .into()
}

fn seen(backpack: &[u8]) -> u64 {
    let mut seen = 0;

    for &b in backpack {
        seen |= 1 << priority(b);
    }

    seen
}

pub fn part1(input: &[u8]) -> Result<String> {
    let mut total = 0;

    for line in input.split(|&b| b == b'\n') {
        let (first, last) = line.split_at(line.len() / 2);

        let seen = seen(first);

        for &b in last {
            let prio = priority(b);

            if (seen & (1 << prio)) != 0 {
                total += prio;
                break;
            }
        }
    }

    Ok(total.to_string())
}

pub fn part2(input: &[u8]) -> Result<String> {
    let mut total = 0;

    for chunk in &input.split(|&b| b == b'\n').chunks(3) {
        let mut mask = u64::MAX;

        for backpack in chunk {
            let seen = seen(backpack);
            mask &= seen;
        }

        if mask != 0 {
            debug_assert_eq!(1, mask.count_ones());
            total += mask.trailing_zeros();
        }
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/03.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "157")
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "70")
    }
}
