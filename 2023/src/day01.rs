use anyhow::Result;
use regex::bytes::Regex;

pub fn part1(input: &[u8]) -> Result<String> {
    let mut it = input.iter();
    let mut sum = 0;

    loop {
        let mut first = None;
        let mut last = 0;

        for &c in &mut it {
            match c {
                d @ b'0'..=b'9' => {
                    let digit = u32::from(d - b'0');
                    first.get_or_insert(digit);
                    last = digit;
                }
                b'\n' => break,
                _ => continue,
            }
        }

        if let Some(first) = first {
            sum += 10 * first + last;
        } else {
            break;
        }
    }

    Ok(sum.to_string())
}

fn parse_string_digit(digit: &[u8]) -> Result<u32> {
    Ok(match digit {
        b"one" => 1,
        b"two" => 2,
        b"three" => 3,
        b"four" => 4,
        b"five" => 5,
        b"six" => 6,
        b"seven" => 7,
        b"eight" => 8,
        b"nine" => 9,
        &[d] => u32::from(d - b'0'),
        other => anyhow::bail!("invalid digit: {}", String::from_utf8_lossy(other)),
    })
}

// 53255: too low
pub fn part2(input: &[u8]) -> Result<String> {
    let parser = Regex::new(r"[1-9]|one|two|three|four|five|six|seven|eight|nine")?;
    let mut sum = 0;

    for line in input.split(|&c| c == b'\n') {
        let mut first = None;
        let mut last = &b""[..];

        let mut start = 0;

        // Cannot use find_iter because it doesn't find overlapping matches.
        while let Some(needle) = parser.find_at(line, start) {
            start = needle.start() + 1;
            let digit = needle.as_bytes();
            first.get_or_insert(digit);
            last = digit;
        }

        if let Some(first) = first {
            sum += 10 * parse_string_digit(first)? + parse_string_digit(last)?;
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
