//! Common helper utilities to all days

use anyhow::Result;
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
