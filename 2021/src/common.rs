use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::marker::PhantomData;
use std::str::FromStr;

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

impl<'a, I: FromStr> Iterator for LineParser<'a, I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()?.parse().ok()
    }
}
