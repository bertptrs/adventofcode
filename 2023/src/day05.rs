use anyhow::Context;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use crate::common::parse_input;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Mapping {
    source_start: u64,
    dest_start: u64,
    len: u64,
}

fn parse_mapping(i: &[u8]) -> IResult<&[u8], Vec<Mapping>> {
    use nom::character::complete::u64;

    let mut mapping = many1(map(
        tuple((
            terminated(u64, tag(" ")),
            terminated(u64, tag(" ")),
            terminated(u64, newline),
        )),
        |(dest_start, source_start, len)| Mapping {
            source_start,
            dest_start,
            len,
        },
    ))(i)?;

    // Sort mappings for O(log n) lookup of appropriate mapping later
    mapping.1.sort_unstable();
    Ok(mapping)
}

struct Almanac {
    seeds: Vec<u64>,
    // There's a lot of mappings but their names don't matter, they all work the same.
    mappings: [Vec<Mapping>; 7],
}

fn parse_almanac(i: &[u8]) -> IResult<&[u8], Almanac> {
    let parse_seeds = delimited(
        tag("seeds:"),
        many1(preceded(tag(" "), nom::character::complete::u64)),
        newline,
    );

    let mapping_parser = |header| preceded(tag(header), parse_mapping);

    map(
        tuple((
            parse_seeds,
            mapping_parser("\nseed-to-soil map:\n"),
            mapping_parser("\nsoil-to-fertilizer map:\n"),
            mapping_parser("\nfertilizer-to-water map:\n"),
            mapping_parser("\nwater-to-light map:\n"),
            mapping_parser("\nlight-to-temperature map:\n"),
            mapping_parser("\ntemperature-to-humidity map:\n"),
            mapping_parser("\nhumidity-to-location map:\n"),
        )),
        |(seeds, soil, fertilizer, water, light, temperature, humidity, location)| Almanac {
            seeds,
            mappings: [
                soil,
                fertilizer,
                water,
                light,
                temperature,
                humidity,
                location,
            ],
        },
    )(i)
}

fn follow_mapping(node: u64, mappings: &[Mapping]) -> u64 {
    let point = mappings.partition_point(|mapping| mapping.source_start <= node);
    if point == 0 {
        // There are no mappings that are smaller than the node, so it maps to itself
        node
    } else {
        // `mapping`` is the last mapping that starts smaller or equal to our node, so it is the one
        // that might contain it.
        let mapping = &mappings[point - 1];

        // Check if the node is in range of this mapping
        if node - mapping.source_start < mapping.len {
            // It is, note the order of operations to avoid underflow
            node + mapping.dest_start - mapping.source_start
        } else {
            // It's not, return itself
            node
        }
    }
}

fn follow_all_mappings(mut node: u64, mappings: &[Vec<Mapping>]) -> u64 {
    for mappings in mappings {
        node = follow_mapping(node, mappings)
    }
    node
}

pub fn part1(input: &[u8]) -> anyhow::Result<String> {
    let almanac = parse_input(input, parse_almanac)?;

    let min = almanac
        .seeds
        .iter()
        .map(|node| follow_all_mappings(*node, &almanac.mappings))
        .min()
        .context("Unreachable, no seeds but parser ensures seeds")?;

    Ok(min.to_string())
}

pub fn part2(input: &[u8]) -> anyhow::Result<String> {
    let almanac = parse_input(input, parse_almanac)?;

    let min = almanac
        .seeds
        .chunks_exact(2)
        .flat_map(|c| c[0]..c[0] + c[1])
        .map(|node| follow_all_mappings(node, &almanac.mappings))
        .min()
        .context("Unreachable, no seeds but parser ensures seeds")?;

    Ok(min.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/05.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "35");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), "46");
    }
}
