use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::marker::PhantomData;
use std::str::FromStr;

/// Line-based iterator/parser
///
/// For each line of the input, attempt to parse it as the requested type. Iteration is stopped on
/// the first IO error or parse error, silently. Leading and trailing whitespace is stripped before
/// attempting to parse.
pub struct LineParser<'a, I>
where
    I: FromStr,
{
    reader: BufReader<&'a mut dyn Read>,
    buffer: String,
    _data: PhantomData<I>,
}

impl<'a, I: FromStr> LineParser<'a, I> {
    pub fn new(input: &'a mut dyn Read) -> Self {
        Self {
            reader: BufReader::new(input),
            buffer: String::new(),
            _data: PhantomData,
        }
    }

    fn next_line(&mut self) -> Option<&str> {
        self.buffer.clear();

        self.reader.read_line(&mut self.buffer).ok()?;

        Some(self.buffer.trim())
    }
}

impl<'a, I: FromStr> Iterator for LineParser<'a, I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_line()?.parse().ok()
    }
}
