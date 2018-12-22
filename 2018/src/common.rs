use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::io;
use std::io::Read;
use std::ops::Add;
use std::ops::Sub;
use std::str::FromStr;

/// Utility trait for things representing coordinates.
///
/// This is implemented by default for any simple pair-tuple
pub trait Point {
    type CoordType;

    /// Compute the manhattan distance between this and another.
    ///
    /// The distance will always be >= 0.
    fn manhattan(self, other: Self) -> Self::CoordType;
}

impl<T> Point for (T, T)
    where T: Add<Output=T> + Sub<Output=T> + Copy + Ord
{
    type CoordType = T;

    fn manhattan(self, other: Self) -> T {
        let (xa, ya) = self;
        let (xb, yb) = other;
        xa.max(xb) + ya.max(yb) - xa.min(xb) - ya.min(yb)
    }
}


/// Apply Erathostenes's sieve to the supplied array
///
/// # Arguments
///
/// * `dest` - the destination slice to fill with the sieve. This is
///   assumed to be filled with "true" before being handed to this
///   method.
pub fn prime_sieve(dest: &mut [bool]) {
    if !dest.is_empty() {
        dest[0] = false;
    }

    if dest.len() >= 2 {
        dest[1] = false;
    }

    let limit = (dest.len() as f64).sqrt() as usize;

    for i in 1..=limit {
        if !dest[i] {
            continue;
        }

        for j in ((i * i)..(dest.len())).step_by(i) {
            dest[j] = false;
        }
    }
}

/// Trim ascii whitespace from a byte vector.
///
/// This method does no allocations, guaranteed.
pub fn trim_back(input: &mut Vec<u8>) {
    let mut to_truncate = 0;
    for b in input.iter().rev() {
        if b.is_ascii_whitespace() {
            to_truncate += 1;
        } else {
            break;
        }
    }

    if to_truncate > 0 {
        let new_len = input.len() - to_truncate;
        input.truncate(new_len);
    }
}

/// Read the entire input as one value.
///
/// This function loads the input into a string and then attempts to parse it.
pub fn read_single_input<T>(input: &mut Read) -> T
    where T: FromStr,
          <T as FromStr>::Err: Debug
{
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();

    buf.trim().parse().unwrap()
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
    where T: Iterator,
          T::Item: Eq + Hash {
    type Type = T::Item;

    fn grouping_count(&mut self) -> HashMap<Self::Type, usize>
    {
        let mut counts = HashMap::new();

        for element in self {
            *counts.entry(element).or_insert(0) += 1;
        }

        counts
    }
}

/// Solution trait
///
/// Every day's solution should implement this function so that it can
/// be easily run from the main program.
pub trait Solution {
    /// Solve the first part of the day
    fn part1(&mut self, input: &mut io::Read) -> String;

    /// Solve the second part of the day
    fn part2(&mut self, input: &mut io::Read) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_sieve() {
        let mut input = [true; 10];
        prime_sieve(&mut input);

        let output = [
            false, false,
            true, true,
            false, true,
            false, true,
            false, false
        ];

        assert_eq!(output, input);
    }

    #[test]
    fn test_grouping_count() {
        let result = [1, 1, 2, 2, 3, 1].iter().grouping_count();
        assert_eq!(3, result[&1]);
        assert_eq!(2, result[&2]);
        assert_eq!(1, result[&3]);
    }
}
