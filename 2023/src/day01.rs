use aho_corasick::AhoCorasick;
use anyhow::Result;

pub fn part1(input: &[u8]) -> Result<String> {
    let mut it = input.iter();
    let mut sum = 0;

    while let Some(&first) = it.find(|s| s.is_ascii_digit()) {
        let mut last = first;

        for &c in &mut it {
            match c {
                d @ b'0'..=b'9' => last = d,
                b'\n' => break,
                _ => continue,
            }
        }

        sum += u32::from(10 * (first - b'0') + last - b'0');
    }

    Ok(sum.to_string())
}

pub fn part2(input: &[u8]) -> Result<String> {
    let parser = AhoCorasick::new([
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ])?;

    fn convert_id(id: u32) -> Result<u32> {
        Ok(match id {
            0..=8 => id + 1,
            9..=17 => id - 8,
            _ => anyhow::bail!("unreachable"),
        })
    }

    let mut sum = 0;

    for line in input.split(|&c| c == b'\n') {
        let mut it = parser.find_overlapping_iter(line);
        if let Some(first) = it.next() {
            let first = convert_id(first.pattern().as_u32())?;
            let last = if let Some(last) = it.last() {
                convert_id(last.pattern().as_u32())?
            } else {
                first
            };

            sum += 10 * first + last;
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/01.1.txt");
    const SAMPLE2: &[u8] = include_bytes!("samples/01.2.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("142", part1(SAMPLE).unwrap());
    }

    #[test]
    fn sample_part2() {
        assert_eq!("281", part2(SAMPLE2).unwrap());
    }
}
