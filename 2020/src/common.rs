use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::iter::FromIterator;
use std::str::FromStr;

/// Read input line by line and try to parse it into some collection.
pub fn from_lines<I, T, E>(input: &mut dyn Read) -> T
where
    I: FromStr<Err = E>,
    E: Debug,
    T: FromIterator<I>,
{
    let reader = BufReader::new(input);

    reader
        .lines()
        .map(|line| line.unwrap().parse::<I>().unwrap())
        .collect()
}

/// Parse the entire input into a single variable
pub fn read_single_input<T>(input: &mut dyn Read) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();

    buf.trim().parse().unwrap()
}

pub fn read_char_grid(input: &mut dyn Read) -> Vec<Vec<u8>> {
    let mut reader = BufReader::new(input);
    let mut buffer = Vec::new();

    let mut grid = Vec::new();

    while let Ok(read) = reader.read_until(b'\n', &mut buffer) {
        if read == 0 {
            break;
        }

        let line: &[u8] = if let Some(&b'\n') = buffer.last() {
            &buffer[..(buffer.len() - 1)]
        } else {
            &buffer[..]
        };

        grid.push(line.to_owned());
        buffer.clear();
    }

    grid
}

/// An interface to count elements in particular categories.
pub trait GroupingCount {
    /// The type of the categories under inspection
    type Type;

    /// Count the occurrence of all possible values.
    ///
    /// This method will return a map from a value to its occurrence rate.
    fn grouping_count(&mut self) -> HashMap<Self::Type, usize>;
}

impl<T> GroupingCount for T
where
    T: Iterator,
    T::Item: Eq + Hash,
{
    type Type = T::Item;

    fn grouping_count(&mut self) -> HashMap<Self::Type, usize> {
        let mut counts = HashMap::new();

        for element in self {
            *counts.entry(element).or_insert(0) += 1;
        }

        counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grouping_count() {
        let result = [1, 1, 2, 2, 3, 1].iter().grouping_count();
        assert_eq!(3, result[&1]);
        assert_eq!(2, result[&2]);
        assert_eq!(1, result[&3]);
    }
}
