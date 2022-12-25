use anyhow::Result;

fn parse_num(num: &[u8]) -> Result<i64> {
    let mut total = 0;
    let mut factor = 1;

    for &b in num.iter().rev() {
        match b {
            b'0' => (),
            b'1' => total += factor,
            b'2' => total += 2 * factor,
            b'-' => total -= factor,
            b'=' => total -= 2 * factor,
            other => anyhow::bail!("Invalid digit {other}"),
        }

        factor *= 5;
    }

    Ok(total)
}

fn encode(mut num: i64) -> String {
    let mut buffer = Vec::new();

    while num > 0 {
        match num % 5 {
            0 => buffer.push(b'0'),
            1 => buffer.push(b'1'),
            2 => buffer.push(b'2'),
            3 => {
                buffer.push(b'=');
                num += 2
            }
            4 => {
                buffer.push(b'-');
                num += 1;
            }
            _ => unreachable!("math"),
        }

        num /= 5;
    }

    // We've built the string right to left, to print we must reverse
    buffer.reverse();

    // Safe unwrap as we've only pushed valid ascii characters
    String::from_utf8(buffer).unwrap()
}

pub fn part1(input: &[u8]) -> Result<String> {
    let total = input
        .split(|&b| b == b'\n')
        .map(parse_num)
        .try_fold(0, |acc, val| val.map(|val| val + acc))?;

    Ok(encode(total))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("./samples/25.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "2=-1=0");
    }
}
