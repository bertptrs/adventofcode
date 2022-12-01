//! Common helper utilities to all days

use anyhow::Result;
use nom::Finish;
use nom::Parser;

/// Parse input from some nom parser and return as an anyhow result
///
/// This method exists as a convenience because nom's errors cannot otherwise be easily converted to
/// an anyhow error, and I don't want to keep track of custom error implementations here.
pub fn parse_input<'a, O>(
    input: &'a [u8],
    mut parser: impl Parser<&'a [u8], O, nom::error::Error<&'a [u8]>>,
) -> Result<O> {
    match parser.parse(input).finish() {
        Ok((_, value)) => Ok(value),
        Err(err) => anyhow::bail!("Failed to parse at: {err:?}"),
    }
}
