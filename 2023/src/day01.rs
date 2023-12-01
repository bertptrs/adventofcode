use anyhow::Result;

fn to_digit(c: u8) -> Option<u32> {
    match c {
        b'0'..=b'9' => Some(u32::from(c - b'0')),
        _ => None,
    }
}

pub fn part1(input: &[u8]) -> Result<String> {
    let mut it = input.iter();
    let mut sum = 0;

    loop {
        let mut first = None;
        let mut last = 0;

        for digit in it
            .by_ref()
            .take_while(|&&c| c != b'\n')
            .filter_map(|&c| to_digit(c))
        {
            first.get_or_insert(digit);
            last = digit;
        }

        if let Some(first) = first {
            sum += 10 * first + last;
        } else {
            break;
        }
    }

    Ok(sum.to_string())
}

pub fn part2(_input: &[u8]) -> Result<String> {
    anyhow::bail!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/01.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("142", part1(SAMPLE).unwrap());
    }
}
