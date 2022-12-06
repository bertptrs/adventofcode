use anyhow::Result;

fn find_first(input: &[u8], unique: usize) -> Result<usize> {
    let mut seen = [false; 256];

    let mut tail_it = input.iter();

    let mut first = 0;

    // Loop invariant: input[first..last] contains only unique characters
    for (last, &c) in input.iter().enumerate() {
        if seen[c as usize] {
            first += (&mut tail_it)
                .take_while(|&&b| b != c)
                .map(|&b| seen[b as usize] = false)
                .count()
                + 1; // +1 because take_while doesn't return the first element that didn't satisfy the condition, while we do need to count it
        } else {
            // New unique character found: input[first..=last] contains unique characters
            if last - first + 1 == unique {
                return Ok(last + 1);
            }

            seen[c as usize] = true;
        }
    }

    anyhow::bail!("Did not find unique sequence of length {unique}");
}

pub fn part1(input: &[u8]) -> Result<String> {
    Ok(find_first(input, 4)?.to_string())
}

pub fn part2(input: &[u8]) -> Result<String> {
    Ok(find_first(input, 14)?.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLES: &[&[u8]] = &[
        b"mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        b"bvwbjplbgvbhsrlpgdmjqwftvncz",
        b"nppdvjthqldpwncqszvftbrmjlhg",
        b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    #[test]
    fn sample_part1() {
        const CORRECT: &[usize] = &[7, 5, 6, 10, 11];

        for (&sample, &correct) in SAMPLES.iter().zip(CORRECT) {
            assert_eq!(find_first(sample, 4).unwrap(), correct);
        }
    }

    #[test]
    fn sample_part2() {
        const CORRECT: &[usize] = &[19, 23, 23, 29, 26];

        for (&sample, &correct) in SAMPLES.iter().zip(CORRECT) {
            assert_eq!(find_first(sample, 14).unwrap(), correct);
        }
    }
}
