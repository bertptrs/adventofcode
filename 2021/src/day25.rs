use std::io::Read;

fn read_input(input: &mut dyn Read) -> (usize, Vec<u8>) {
    let mut result = Vec::new();
    input.read_to_end(&mut result).unwrap();

    let width = result.iter().position(|&c| c == b'\n').unwrap();
    result.retain(|c| !c.is_ascii_whitespace());

    (width, result)
}

fn advance(width: usize, state: &mut [u8]) -> bool {
    debug_assert_eq!(state.len() % width, 0);
    let mut changes = false;

    // Move the eastbound herd
    for src in state.chunks_exact_mut(width) {
        let swap_last = src[0] == b'.' && src[width - 1] == b'>';
        let mut x = 0;

        while x < src.len() - 1 {
            if src[x] == b'>' && src[x + 1] == b'.' {
                src.swap(x, x + 1);
                changes = true;
                x += 2;
            } else {
                x += 1;
            }
        }

        if swap_last {
            src.swap(0, width - 1);
            changes = true;
        }
    }

    // Then move the southbound herd. Need to do by column because of the first entry special case
    for x in 0..width {
        let last_index = state.len() - width + x;
        let swap_last = state[x] == b'.' && state[last_index] == b'v';

        let mut offset = x;

        while offset < state.len() - width {
            if state[offset] == b'v' && state[offset + width] == b'.' {
                state.swap(offset, offset + width);
                changes = true;
                offset += 2 * width;
            } else {
                offset += width;
            }
        }

        if swap_last {
            state.swap(x, last_index);
            changes = true;
        }
    }

    changes
}

pub fn part1(input: &mut dyn Read) -> String {
    let (width, mut state) = read_input(input);

    (1..)
        .find(|_| !advance(width, &mut state))
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_implementation;

    const SAMPLE: &[u8] = include_bytes!("samples/25.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 58);
    }
}
