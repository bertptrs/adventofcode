use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::iter::FromIterator;
use std::rc::Rc;
use std::str::FromStr;

/// Read input line by line and try to parse it into some collection.
pub fn from_lines<I, T, E>(input: &mut dyn Read) -> T
where
    I: FromStr<Err = E>,
    E: Debug,
    T: FromIterator<I>,
{
    Lines::new(input)
        .map(|line| line.parse::<I>().unwrap())
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
    BufReader::new(input)
        .lines()
        // filter_map avoids an expensive unwrap and we know our input is valid ascii
        .filter_map(|s| s.ok().map(String::into_bytes))
        .collect()
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

/// Iterator that allows for mostly alloc-less &str iteration
///
/// If an owned String is needed, use `BufRead::lines()` as this version is
/// optimized for temporary references.
pub struct Lines<T>
where
    T: Read,
{
    reader: BufReader<T>,
    buffer: Rc<String>,
}

impl<T> Lines<T>
where
    T: Read,
{
    pub fn new(input: T) -> Self {
        Self {
            reader: BufReader::new(input),
            buffer: Rc::default(),
        }
    }
}

impl<T> Iterator for Lines<T>
where
    T: Read,
{
    type Item = Rc<String>;

    fn next(&mut self) -> Option<Self::Item> {
        // Assuming the consumer has released the previous reference to the
        // string, this should not make a copy
        let buffer = Rc::make_mut(&mut self.buffer);
        buffer.clear();

        if let Ok(read) = self.reader.read_line(buffer) {
            if read > 0 {
                if buffer.ends_with('\n') {
                    buffer.pop();
                }

                return Some(Rc::clone(&self.buffer));
            }
        }

        None
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
