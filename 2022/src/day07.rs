use std::fmt::Debug;

use anyhow::Context;
use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use crate::common::parse_input;

type Listing<'a> = Vec<(&'a [u8], ListEntry<'a>)>;
type Slicing<'a> = &'a [(&'a [u8], ListEntry<'a>)];

#[derive(Debug)]
enum ListEntry<'a> {
    File(u32),
    Dir(Listing<'a>),
}

fn parse_dir(input: &[u8]) -> IResult<&[u8], Listing> {
    use nom::character::complete::u32;

    let (mut input, mut entries) = preceded(
        tag("$ ls\n"),
        many0(
            // Map many newline-terminated entries
            terminated(
                // of either
                alt((
                    // A size followed by a name
                    map(
                        separated_pair(u32, tag(" "), take_until("\n")),
                        |(size, name)| (name, ListEntry::File(size)),
                    ),
                    // Or the word "dir" followed by a name
                    map(preceded(tag("dir "), take_until("\n")), |name| {
                        (name, ListEntry::Dir(Vec::new()))
                    }),
                )),
                newline,
            ),
        ),
    )(input)?;

    // Assumption: directory entries are queried in the order they are listed
    //
    // This assumption appears to hold for my input so it's fine
    for (name, entry) in &mut entries {
        if let ListEntry::Dir(placeholder) = entry {
            let (new_input, contents) = delimited(
                tuple((tag("$ cd "), tag(*name), newline)),
                parse_dir,
                // Optional cd'ing out because the last directory is never exited.
                opt(tag("$ cd ..\n")),
            )(input)?;

            input = new_input;
            *placeholder = contents;
        }
    }

    Ok((input, entries))
}

fn parse_program(input: &[u8]) -> IResult<&[u8], Listing> {
    preceded(tag("$ cd /\n"), parse_dir)(input)
}

fn sum_sizes(listing: Slicing<'_>, acc: &mut impl FnMut(u32)) -> u32 {
    let mut total_size = 0;

    for (_, entry) in listing {
        match entry {
            ListEntry::File(size) => total_size += size,
            ListEntry::Dir(listing) => total_size += sum_sizes(listing, acc),
        }
    }

    acc(total_size);

    total_size
}

pub fn part1(input: &[u8]) -> Result<String> {
    let root = parse_input(input, parse_program)?;

    let mut searched_size = 0;

    sum_sizes(&root, &mut |size| {
        if size <= 100000 {
            searched_size += size
        }
    });

    Ok(searched_size.to_string())
}

pub fn part2(input: &[u8]) -> Result<String> {
    const TARGET: u32 = 30000000;
    const TOTAL: u32 = 70000000;

    let root = parse_input(input, parse_program)?;

    let mut sizes = Vec::new();

    let used = sum_sizes(&root, &mut |size| sizes.push(size));

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
