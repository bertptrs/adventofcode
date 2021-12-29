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

    let mut helper = state.to_owned();
    let mut changes = false;

    // Move the eastbound herd
    for (src, dest) in state
        .chunks_exact(width)
        .zip(helper.chunks_exact_mut(width))
    {
        for x in 0..width {
            if src[x] == b'>' && src[(x + 1) % width] == b'.' {
                dest[x] = b'.';
                dest[(x + 1) % width] = b'>';
                changes = true;
            }
        }
    }

    state.copy_from_slice(&helper);

    // Now the southbound herd. Y in the outer loop for better cache locality
    for y_offset in (0..helper.len()).step_by(width) {
        let n_offset = (y_offset + width) % helper.len();

        for x in 0..width {
            if helper[x + y_offset] == b'v' && helper[x + n_offset] == b'.' {
                state[x + y_offset] = b'.';
                state[x + n_offset] = b'v';
                changes = true;
            }
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
