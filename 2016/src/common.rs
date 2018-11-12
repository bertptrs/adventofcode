use std::io;
use std::ops;
use std::cmp;

/// Apply Erathostenes's sieve to the supplied array
///
/// # Arguments
///
/// * `dest` - the destination slice to fill with the sieve. This is
///   assumed to be filled with "true" before being handed to this
///   method.
pub fn prime_sieve(dest: &mut[bool]) {
    if dest.len() >= 1 {
        dest[0] = false;
    }

    if dest.len() >= 2 {
        dest[1] = false;
    }

    let limit = (dest.len() as f64).sqrt() as usize;

    for i in 1..(limit + 1) {
        if !dest[i] {
            continue
        }

        for j in ((i * i)..(dest.len())).step_by(i) {
            dest[j] = false;
        }
    }
}

/// Greatest common divisor
pub fn gcd<T: ops::Rem<Output = T> + cmp::PartialOrd + std::convert::From<i32> + Copy>(a: T, b: T) -> T {
    if a < b {
        gcd(b, a)
    } else {
        if a % b == T::from(0) {
            b
        } else {
            gcd(a % b, b)
        }
    }
}

/// Least common multiple
pub fn lcm<T: ops::Rem<Output = T> + ops::Mul<Output = T> + ops::Div<Output = T> + cmp::PartialOrd + std::convert::From<i32> + Copy>(a: T, b: T) -> T {
    a * b / gcd(a, b)
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
    fn test_gcd() {
        assert_eq!(12, gcd(24, 36));
        assert_eq!(1, gcd(1, 7));
    }

    #[test]
    fn test_lcm() {
        assert_eq!(12, lcm(6, 4));
    }
}
