fn trim(input: &[u8]) -> &[u8] {
    let whitespace = input
        .iter()
        .rev()
        .take_while(|c| c.is_ascii_whitespace())
        .count();

    &input[..(input.len() - whitespace)]
}

fn hash(input: &[u8]) -> u32 {
    input
        .iter()
        .fold(0, |cur, &c| ((cur + u32::from(c)) * 17) % 256)
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let input = trim(input);

    Ok(input
        .split(|&c| c == b',')
        .map(hash)
        .sum::<u32>()
        .to_string())
}

pub fn part2(_input: &[u8]) -> anyhow::Result<String> {
    anyhow::bail!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/15.txt");

    #[test]
    fn sample_hash() {
        assert_eq!(hash(b"HASH"), 52);
    }

    #[test]
    fn sample_part1() {
        assert_eq!("1320", part1(SAMPLE).unwrap());
    }
}
