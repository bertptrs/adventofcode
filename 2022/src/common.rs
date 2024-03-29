//! Common helper utilities to all days

use std::cmp::Ordering;
use std::ops::Add;
use std::ops::Div;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Sub;

use anyhow::Result;
use nom::combinator::map;
use nom::error::ErrorKind;
use nom::error::ParseError;
use nom::Finish;
use nom::IResult;
use nom::InputLength;
use nom::Parser;

/// Parse input from some nom parser and return as an anyhow result
///
/// This method exists as a convenience because nom's errors cannot otherwise be easily converted to
/// an anyhow error, and I don't want to keep track of custom error implementations here.
pub fn parse_input<'a, O>(
    input: &'a [u8],
    mut parser: impl Parser<&'a [u8], O, nom::error::Error<&'a [u8]>>,
) -> Result<O> {
    let (unparsed, output) = parser.parse(input).finish().map_err(|e| {
        anyhow::anyhow!(
            "Parser error {:?} to parse at {}",
            e.code,
            String::from_utf8_lossy(e.input)
        )
    })?;

    if !unparsed.is_empty() {
        Err(anyhow::anyhow!(
            "Not all input consumed: {}",
            String::from_utf8_lossy(unparsed)
        ))
    } else {
        Ok(output)
    }
}

/// Applies a parser iteratively and reduces the results using the given function. Fails if the
/// embedded parser doesn't return at least one result.
///
/// # Arguments
/// - `f`: the function to apply
/// - `g`: the function that combines the result o `f` with previous results
///
/// This implementation is based on [`nom::multi::fold_many1`] with minor differences. If
/// successful, this should probably be upstreamed.
pub fn reduce_many1<I, O, E, F>(
    mut f: F,
    mut g: impl FnMut(O, O) -> O,
) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: Clone + InputLength,
    E: ParseError<I>,
    F: Parser<I, O, E>,
{
    // Cannot delegate to fold_many0 because that would make the function FnOnce rather than FnMut,
    // since it would transfer ownership of the embedded parser to fold_many0.
    move |i: I| {
        let _i = i.clone();
        match f.parse(_i) {
            Err(nom::Err::Error(_)) => {
                Err(nom::Err::Error(E::from_error_kind(i, ErrorKind::Many1)))
            }
            Err(e) => Err(e),
            Ok((i1, mut acc)) => {
                let mut input = i1;

                loop {
                    let _input = input.clone();
                    let len = input.input_len();
                    match f.parse(_input) {
                        Err(nom::Err::Error(_)) => {
                            break;
                        }
                        Err(e) => return Err(e),
                        Ok((i, o)) => {
                            // infinite loop check: the parser must always consume
                            if i.input_len() == len {
                                return Err(nom::Err::Failure(E::from_error_kind(
                                    i,
                                    ErrorKind::Many1,
                                )));
                            }

                            acc = g(acc, o);
                            input = i;
                        }
                    }
                }

                Ok((input, acc))
            }
        }
    }
}

/// Add an index to repeated successful invocations of the embedded parser.
pub fn enumerate<I, O, E>(f: impl Parser<I, O, E>) -> impl FnMut(I) -> IResult<I, (usize, O), E> {
    let mut index = 0usize;

    map(f, move |v| {
        let res = (index, v);
        index += 1;
        res
    })
}

/// Return the minimum and maximum of two unordered variables
pub fn minmax<T>(a: T, b: T) -> (T, T)
where
    T: PartialOrd,
{
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

/// Some magic to get two mutable references into the same slice
pub fn get_both<T>(slice: &mut [T], first: usize, second: usize) -> (&mut T, &mut T) {
    match first.cmp(&second) {
        Ordering::Greater => {
            let (begin, end) = slice.split_at_mut(first);
            (&mut end[0], &mut begin[second])
        }
        Ordering::Less => {
            let (begin, end) = slice.split_at_mut(second);
            (&mut begin[first], &mut end[0])
        }
        Ordering::Equal => panic!("Tried to get the same index twice {first}"),
    }
}

#[derive(Debug, Default)]
pub struct IndexSet(Vec<u32>);

impl IndexSet {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(
            capacity / std::mem::size_of::<u32>() / 8,
        ))
    }

    fn ensure_item(&mut self, item: usize) -> &mut u32 {
        if self.0.len() <= item {
            self.0.resize(item + 1, 0);
        }

        &mut self.0[item]
    }

    #[inline]
    fn index(index: usize) -> (usize, u8) {
        const PER_ENTRY: usize = 8 * std::mem::size_of::<u32>();

        (index / PER_ENTRY, (index % PER_ENTRY) as u8)
    }

    pub fn insert(&mut self, index: usize) -> bool {
        let (entry, pos) = Self::index(index);

        let item = self.ensure_item(entry);

        if *item & (1 << pos) != 0 {
            false
        } else {
            *item |= 1 << pos;
            true
        }
    }

    pub fn contains(&self, index: usize) -> bool {
        let (entry, pos) = Self::index(index);

        self.0
            .get(entry)
            .map_or(false, |&entry| (entry & (1 << pos) != 0))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vec2(pub [i32; 2]);

impl Vec2 {
    pub fn l1(self) -> i32 {
        self.0.into_iter().map(i32::abs).sum()
    }
}

impl Add<Self> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self([self[0] + rhs[0], self[1] + rhs[1]])
    }
}

impl Sub<Self> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self([self[0] - rhs[0], self[1] - rhs[1]])
    }
}

impl Div<i32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self(self.0.map(|v| v / rhs))
    }
}

impl Index<usize> for Vec2 {
    type Output = i32;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
