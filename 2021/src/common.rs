use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::marker::PhantomData;
use std::str::FromStr;

use nom::Finish;
use nom::IResult;

pub struct LineIter<'a> {
    reader: BufReader<&'a mut dyn Read>,
    buffer: String,
}

impl<'a> LineIter<'a> {
    pub fn new(input: &'a mut dyn Read) -> Self {
        Self {
            reader: BufReader::new(input),
            buffer: String::new(),
        }
    }

    /// Get the next line, or None
    ///
    /// This is deliberately not an [Iterator] impl as those cannot hand out references to self.
    pub fn next(&mut self) -> Option<&str> {
        self.buffer.clear();

        if matches!(self.reader.read_line(&mut self.buffer), Ok(n) if n > 0) {
            Some(self.buffer.trim_end_matches('\n'))
        } else {
            None
        }
    }
}

/// Line-based iterator/parser
///
/// For each line of the input, attempt to parse it as the requested type. Iteration is stopped on
/// the first IO error or parse error, silently. Leading and trailing whitespace is stripped before
/// attempting to parse.
pub struct LineParser<'a, I>
where
    I: FromStr,
{
    iter: LineIter<'a>,
    _data: PhantomData<I>,
}

impl<'a, I: FromStr> LineParser<'a, I> {
    pub fn new(input: &'a mut dyn Read) -> Self {
        Self {
            iter: LineIter::new(input),
            _data: PhantomData,
        }
    }
}

impl<I: FromStr> Iterator for LineParser<'_, I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()?.parse().ok()
    }
}

impl<'a, I: FromStr> From<LineIter<'a>> for LineParser<'a, I> {
    fn from(iter: LineIter<'a>) -> Self {
        Self {
            iter,
            _data: PhantomData,
        }
    }
}

/// Return two arguments in their natural PartialOrd order
pub fn ordered<O: PartialOrd>(a: O, b: O) -> (O, O) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

pub fn read_input<I, P, O>(mut input: I, parser: P) -> O
where
    I: Read,
    P: for<'a> FnOnce(&'a [u8]) -> IResult<&'a [u8], O>,
{
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer).unwrap();

    match parser(&buffer).finish() {
        Ok((_, output)) => output,
        Err(err) => {
            panic!(
                "Failed to parse input with error {:?} at \"{}\"",
                err.code,
                String::from_utf8_lossy(err.input)
            );
        }
    }
}

pub struct BitSet {
    buffer: Vec<u32>,
}

impl BitSet {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let buffer = Vec::with_capacity(capacity);

        Self { buffer }
    }

    pub fn insert(&mut self, value: usize) -> bool {
        let chunk = value / 32;
        let bit = 1 << (31 - (value % 32));

        if self.buffer.len() <= chunk + 1 {
            self.buffer.resize(chunk + 1, 0);
        }

        let not_present = self.buffer[chunk] & bit;

        self.buffer[chunk] |= bit;

        not_present == 0
    }

    pub fn len(&self) -> usize {
        self.buffer.iter().map(|c| c.count_ones() as usize).sum()
    }
}
