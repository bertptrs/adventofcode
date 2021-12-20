use std::collections::HashSet;
use std::io::Read;
use std::mem::swap;

type Translation = [bool; 512];
type Point = (i32, i32);
type Field = HashSet<Point>;

fn read_input(input: &mut dyn Read) -> (Translation, Field) {
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer).unwrap();

    let mut translation = [false; 512];

    let mut it = buffer.split(|&b| b == b'\n');

    translation
        .iter_mut()
        .zip(it.next().unwrap())
        .for_each(|(t, &c)| *t = c == b'#');

    let mut field = Field::default();

    for (y, line) in it.skip(1).enumerate() {
        for (x, _) in line.iter().enumerate().filter(|(_, &c)| c == b'#') {
            field.insert((x as i32, y as i32));
        }
    }

    (translation, field)
}

fn find_dimensions(field: &Field) -> ((i32, i32), (i32, i32)) {
    field
        .iter()
        .fold(((0, 0), (0, 0)), |((xmin, xmax), (ymin, ymax)), &(x, y)| {
            ((xmin.min(x), xmax.max(x)), (ymin.min(y), ymax.max(y)))
        })
}

fn advance(translation: &Translation, field: &Field, new_field: &mut Field, infinity: &mut bool) {
    const INDEX_MASK: usize = (1 << 9) - 1;
    new_field.clear();

    let ((xmin, xmax), (ymin, ymax)) = find_dimensions(field);

    for x in (xmin - 1)..=(xmax + 1) {
        let mut index = if *infinity { INDEX_MASK } else { 0 };

        for y in (ymin - 1)..=(ymax + 1) {
            for dx in -1..=1 {
                index <<= 1;

                let nx = x + dx;
                let ny = y + 1;

                if nx < xmin || nx > xmax || ny < ymin || ny > ymax {
                    index |= *infinity as usize;
                } else if field.contains(&(nx, ny)) {
                    index |= 1;
                }
            }

            index &= INDEX_MASK;

            if translation[index] {
                new_field.insert((x, y));
            }
        }
    }

    *infinity = translation[if *infinity { 511 } else { 0 }]
}

fn parts_common(input: &mut dyn Read, count: usize) -> String {
    let (translation, mut field) = read_input(input);
    let mut new_field = Field::new();
    let mut infinity = false;

    for _ in 0..count {
        advance(&translation, &field, &mut new_field, &mut infinity);
        swap(&mut field, &mut new_field);
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
