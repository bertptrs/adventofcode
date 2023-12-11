use crate::common::Grid;
use crate::common::IndexSet;

fn find_doubles(occupied: IndexSet, len: usize) -> Vec<usize> {
    // TODO: this should iterate over IndexSet but that doesn't work yet and I can't be bothered.
    (0..len).filter(|&v| !occupied.contains(v)).collect()
}

fn transform(pos: usize, doubles: &[usize]) -> usize {
    let before = doubles.partition_point(|&v| v < pos);
    pos + before
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let map = Grid::new(input)?;
    let mut stars = Vec::new();
    let mut cols_occupied = IndexSet::with_capacity(map.width());
    let mut rows_occupied = IndexSet::with_capacity(map.height());

    for (y, row) in map.rows().enumerate() {
        for (x, &pixel) in row.iter().enumerate() {
            if pixel == b'#' {
                cols_occupied.insert(x);
                rows_occupied.insert(y);
                stars.push((x, y));
            }
        }
    }
    let cols_doubled = find_doubles(cols_occupied, map.width());
    let rows_doubled = find_doubles(rows_occupied, map.height());

    for star in &mut stars {
        star.0 = transform(star.0, &cols_doubled);
        star.1 = transform(star.1, &rows_doubled);
    }

    let total: usize = stars
        .iter()
        .enumerate()
        .flat_map(|(i, star)| {
            stars[i + 1..]
                .iter()
                .map(|other| star.0.abs_diff(other.0) + star.1.abs_diff(other.1))
        })
        .sum();

    Ok(total.to_string())
}

pub fn part2(_input: &[u8]) -> anyhow::Result<String> {
    anyhow::bail!("Not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/11.txt");

    #[test]
    fn sample_part1() {
        assert_eq!("374", part1(SAMPLE).unwrap());
    }

    // #[test]
    // fn sample_part2() {
    // }
}
