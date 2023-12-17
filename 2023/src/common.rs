//! Common helper utilities to all days

use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Div;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Sub;

use anyhow::Context;
use anyhow::Result;
use nom::combinator::map;
use nom::error::ErrorKind;
use nom::error::ParseError;
use nom::Finish;
use nom::IResult;
use nom::InputLength;
use nom::Parser;

pub fn convert_nom_error(e: nom::error::Error<&[u8]>) -> anyhow::Error {
    anyhow::anyhow!(
        "Parser error {:?} to parse at {}",
        e.code,
        String::from_utf8_lossy(e.input)
    )
}

/// Parse input from some nom parser and return as an anyhow result
///
/// This method exists as a convenience because nom's errors cannot otherwise be easily converted to
/// an anyhow error, and I don't want to keep track of custom error implementations here.
pub fn parse_input<'a, O>(
    input: &'a [u8],
    mut parser: impl Parser<&'a [u8], O, nom::error::Error<&'a [u8]>>,
) -> Result<O> {
    let (unparsed, output) = parser.parse(input).finish().map_err(convert_nom_error)?;

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
#[allow(unused)]
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
#[allow(unused)]
pub fn enumerate<I, O, E>(f: impl Parser<I, O, E>) -> impl FnMut(I) -> IResult<I, (usize, O), E> {
    let mut index = 0usize;

    map(f, move |v| {
        let res = (index, v);
        index += 1;
        res
    })
}

/// Return the minimum and maximum of two unordered variables
#[allow(unused)]
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
#[allow(unused)]
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
        let old = *item;

        *item |= 1 << pos;

        old != *item
    }

    pub fn contains(&self, index: usize) -> bool {
        let (entry, pos) = Self::index(index);

        self.0
            .get(entry)
            .map_or(false, |&entry| (entry & (1 << pos) != 0))
    }
}

#[allow(unused)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vec2(pub [i32; 2]);

#[allow(unused)]
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

#[derive(PartialEq)]
pub struct Grid<T: AsRef<[u8]>> {
    width: usize,
    data: T,
}

impl<T: AsRef<[u8]>> Grid<T> {
    pub fn new(data: T) -> anyhow::Result<Self> {
        let width = 1 + data
            .as_ref()
            .iter()
            .position(|&c| c == b'\n')
            .context("Failed to find end of line in grid")?;

        anyhow::ensure!(
            data.as_ref().len() % width == 0,
            "Grid should divide equally into rows"
        );

        Ok(Self { width, data })
    }

    pub fn height(&self) -> usize {
        self.data.as_ref().len() / self.width
    }

    pub fn width(&self) -> usize {
        self.width - 1
    }

    pub fn rows(&self) -> impl Iterator<Item = &[u8]> {
        let width = self.width();
        self.data
            .as_ref()
            .chunks_exact(self.width)
            .map(move |row| &row[..width])
    }

    pub fn find(&self, c: u8) -> Option<(usize, usize)> {
        let pos = self.data.as_ref().iter().position(|&d| d == c)?;

        Some((pos % self.width, pos / self.width))
    }
}

impl Grid<Vec<u8>> {
    pub fn zeroed(width: usize, height: usize) -> Self {
        let mut data = vec![0; (width + 1) * height];

        for line_end in data[width..].iter_mut().step_by(width + 1) {
            *line_end = b'\n';
        }

        Self {
            width: width + 1,
            data,
        }
    }
}

impl<T: AsMut<[u8]> + AsRef<[u8]>> Grid<T> {
    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [u8]> {
        let width = self.width();
        self.data
            .as_mut()
            .chunks_exact_mut(self.width)
            .map(move |row| &mut row[..width])
    }
}

impl<T: AsRef<[u8]>> Index<usize> for Grid<T> {
    type Output = [u8];

    fn index(&self, y: usize) -> &Self::Output {
        let offset = y * self.width;
        &self.data.as_ref()[offset..(offset + self.width())]
    }
}

impl<T: AsRef<[u8]>> Index<(usize, usize)> for Grid<T> {
    type Output = u8;

    fn index(&self, (y, x): (usize, usize)) -> &Self::Output {
        debug_assert!(y < self.height());
        debug_assert!(x < self.width());

        let offset = self.width * y + x;
        &self.data.as_ref()[offset]
    }
}

impl<T: AsRef<[u8]>> Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.data.as_ref()))
    }
}

// Custom Clone impl so we don't allocate in `clone_from`
impl<T: AsRef<[u8]> + Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            data: self.data.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.width = source.width;
        self.data.clone_from(&source.data);
    }
}

impl<T: AsMut<[u8]> + AsRef<[u8]>> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, y: usize) -> &mut Self::Output {
        let offset = y * self.width;
        let width = self.width;
        &mut self.data.as_mut()[offset..(offset + width)]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3,
}

impl Direction {
    pub fn bit(self) -> u8 {
        1 << self as u8
    }

    pub fn is_vertical(self) -> bool {
        matches!(self, Direction::Down | Direction::Up)
    }
}
