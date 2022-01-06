use std::fmt::Display;
use std::io::Read;
use std::ops::Index;

use crate::common::BitSet;

type Translation = [bool; 512];

struct Field {
    width: usize,
    height: usize,
    infinity: bool,
    finite: BitSet,
}

impl Field {
    pub fn from_input<'a>(input: impl Iterator<Item = &'a [u8]>) -> Self {
        let mut input = input.peekable();

        let width = input.peek().unwrap().len();

        let mut finite = BitSet::new();

        let len = input
            .flatten()
            .enumerate()
            .map(|(index, &c)| {
                if c == b'#' {
                    finite.insert(index);
                }
            })
            .count();

        debug_assert_eq!(len % width, 0);
        let height = len / width;

        Self {
            width,
            height,
            finite,
            infinity: false,
        }
    }

    pub fn advance(&mut self, translation: &[bool; 512]) {
        const INDEX_MASK: usize = (1 << 9) - 1;

        let new_width = self.width + 2;
        let new_height = self.height + 2;

        let mut new_finite = BitSet::with_capacity(new_width * new_height);

        // Now we can just do a normal loop
        for y in 0..new_height {
            for x in 0..new_width {
                let mut mask = if self.infinity { INDEX_MASK } else { 0 };

                for y in y.saturating_sub(2)..=y {
                    if x < 2 {
                        for _ in 0..(2 - x) {
                            mask = self.infinity as usize | (mask << 1);
                        }
                    }

                    for x in x.saturating_sub(2)..=x {
                        mask = (mask << 1) | (self[(x, y)] as usize);
                    }
                }

                if translation[mask & INDEX_MASK] {
                    let index = x + y * new_width;
                    new_finite.insert(index);
                }
            }
        }

        self.width += 2;
        self.height += 2;
        self.finite = new_finite;
        self.infinity = translation[if self.infinity { INDEX_MASK } else { 0 }];
    }

    pub fn len(&self) -> usize {
        assert!(!self.infinity);
        self.finite.len()
    }
}

impl Index<(usize, usize)> for Field {
    type Output = bool;

    #[inline]
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        if x >= self.width || y >= self.height {
            return &self.infinity;
        }

        let index = x + y * self.width;

        if self.finite.contains(index) {
            &true
        } else {
            &false
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if self[(x, y)] {
                    write!(f, "#")?
                } else {
                    write!(f, ".")?
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn read_input(input: &mut dyn Read) -> (Translation, Field) {
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer).unwrap();

    let mut translation = [false; 512];

    let mut it = buffer.split(|&b| b == b'\n');

    translation
        .iter_mut()
        .zip(it.next().unwrap())
        .for_each(|(t, &c)| *t = c == b'#');

    let field = Field::from_input(it.skip(1));

    (translation, field)
}

fn parts_common(input: &mut dyn Read, count: usize) -> String {
    let (translation, mut field) = read_input(input);

    for _ in 0..count {
        field.advance(&translation);
    }

    field.len().to_string()
}

pub fn part1(input: &mut dyn Read) -> String {
    parts_common(input, 2)
}

pub fn part2(input: &mut dyn Read) -> String {
    parts_common(input, 50)
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/20.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 35);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 3351);
    }
}
