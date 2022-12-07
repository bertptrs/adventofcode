use anyhow::Context;
use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::fold_many0;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use crate::common::parse_input;

fn parse_dir<'a>(
    input: &'a [u8],
    dirs: &mut Vec<u32>,
    dir_stack: &mut Vec<&'a [u8]>,
) -> IResult<&'a [u8], u32> {
    use nom::character::complete::u32;

    enum Entry<'a> {
        File(u32),
        Dir(&'a [u8]),
    }
    let initial_len = dir_stack.len();

    let (mut input, mut size) = preceded(
        tag("$ ls\n"),
        fold_many0(
            // Map many newline-terminated entries
            terminated(
                // of either
                alt((
                    // A size followed by a name
                    map(terminated(u32, take_until("\n")), Entry::File),
                    // Or the word "dir" followed by a name
                    map(preceded(tag("dir "), take_until("\n")), Entry::Dir),
                )),
                newline,
            ),
            || 0u32,
            |files_sum, entry| match entry {
                Entry::File(size) => files_sum + size,
                Entry::Dir(name) => {
                    dir_stack.push(name);
                    files_sum
                }
            },
        ),
    )(input)?;

    for i in initial_len..dir_stack.len() {
        let (new_input, content_size) = delimited(
            tuple((tag("$ cd "), tag(dir_stack[i]), newline)),
            |input| parse_dir(input, dirs, dir_stack),
            // Optional cd'ing out because the last directory is never exited.
            opt(tag("$ cd ..\n")),
        )(input)?;

        input = new_input;
        size += content_size;
    }

    dirs.push(size);
    dir_stack.truncate(initial_len);

    Ok((input, size))
}

fn parse_program(input: &[u8]) -> IResult<&[u8], (u32, Vec<u32>)> {
    let mut dirs = Vec::new();
    let mut dirstack = Vec::new();
    let (input, size) = preceded(tag("$ cd /\n"), |input| {
        parse_dir(input, &mut dirs, &mut dirstack)
    })(input)?;

    Ok((input, (size, dirs)))
}

pub fn part1(input: &[u8]) -> Result<String> {
    let (_, sizes) = parse_input(input, parse_program)?;

    let searched_size: u32 = sizes.into_iter().filter(|&size| size <= 100000).sum();

    Ok(searched_size.to_string())
}

pub fn part2(input: &[u8]) -> Result<String> {
    const TARGET: u32 = 30000000;
    const TOTAL: u32 = 70000000;

    let (used, sizes) = parse_input(input, parse_program)?;

    let required = TARGET - (TOTAL - used);

    let min = sizes
        .into_iter()
        .filter(|&size| size >= required)
        .min()
        .context("Did not find dir large enough to delete")?;

    Ok(min.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/07.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "95437");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "24933642");
    }
}
