use anyhow::Result;

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
