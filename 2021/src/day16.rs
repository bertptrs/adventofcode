use std::io::Read;

use nom::bits::complete::tag;
use nom::bits::complete::take;
use nom::branch::alt;
use nom::combinator::map;
use nom::multi::count;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;

type Input<'a> = (&'a [u8], usize);

#[derive(Debug, Eq, PartialEq)]
enum Contents {
    Literal(Vec<u8>),
    Operator(u8, Vec<Packet>),
}

#[derive(Debug, Eq, PartialEq)]
struct Packet {
    version: u8,
    contents: Contents,
}

impl Packet {
    pub fn version_sum(&self) -> u32 {
        match &self.contents {
            Contents::Literal(_) => self.version as u32,
            Contents::Operator(_, sub_packets) => {
                self.version as u32 + sub_packets.iter().map(Packet::version_sum).sum::<u32>()
            }
        }
    }

    pub fn value(&self) -> u64 {
        match &self.contents {
            Contents::Literal(chunks) => chunks
                .iter()
                .fold(0, |acc, &chunk| (acc << 4) | (chunk as u64)),
            Contents::Operator(0, sub_packets) => sub_packets.iter().map(Packet::value).sum(),
            Contents::Operator(1, sub_packets) => sub_packets.iter().map(Packet::value).product(),
            Contents::Operator(2, sub_packets) => {
                sub_packets.iter().map(Packet::value).min().unwrap()
            }
            Contents::Operator(3, sub_packets) => {
                sub_packets.iter().map(Packet::value).max().unwrap()
            }
            Contents::Operator(5, sub_packets) => {
                (sub_packets[0].value() > sub_packets[1].value()) as u64
            }
            Contents::Operator(6, sub_packets) => {
                (sub_packets[0].value() < sub_packets[1].value()) as u64
            }
            Contents::Operator(7, sub_packets) => {
                (sub_packets[0].value() == sub_packets[1].value()) as u64
            }
            unknown => panic!("unknown packet {:?}", unknown),
        }
    }
}

fn parse_literal(mut input: Input) -> IResult<Input, (usize, Contents)> {
    let mut contents = Vec::new();

    loop {
        let (new_input, result) = take::<_, u8, usize, _>(5)(input)?;
        input = new_input;

        contents.push(result & 0xF);

        if (result & 0x10) == 0 {
            let len = 5 * contents.len();
            let contents = Contents::Literal(contents);

            return Ok((input, (len, contents)));
        }
    }
}

fn parse_operator_len(input: Input) -> IResult<Input, (usize, Vec<Packet>)> {
    const SIZE_LEN: usize = 15;

    let (mut input, to_read) = take(SIZE_LEN)(input)?;

    let mut read = 0;
    let mut packets = Vec::new();

    while read < to_read {
        let (new_input, (len, packet)) = parse_packet(input)?;
        input = new_input;
        read += len;
        packets.push(packet);
    }

    Ok((input, (to_read + SIZE_LEN, packets)))
}

fn parse_operator_count(input: Input) -> IResult<Input, (usize, Vec<Packet>)> {
    const SIZE_LEN: usize = 11;

    let (input, to_read) = take::<_, usize, _, _>(SIZE_LEN)(input)?;

    let (input, packets) = count(parse_packet, to_read)(input)?;

    let read_total = SIZE_LEN + packets.iter().map(|(len, _)| *len).sum::<usize>();

    Ok((
        input,
        (
            read_total,
            packets.into_iter().map(|(_, packet)| packet).collect(),
        ),
    ))
}

fn parse_packet(input: Input) -> IResult<Input, (usize, Packet)> {
    let parse_literal_packet = map(
        tuple((take(3usize), preceded(tag(4u8, 3usize), parse_literal))),
        |(version, (len, contents))| (len + 6, Packet { version, contents }),
    );

    let parse_operator_len_packet = map(
        tuple((
            take(3usize),
            take(3usize),
            preceded(tag(0u8, 1usize), parse_operator_len),
        )),
        |(version, operator, (len, contents))| {
            (
                len + 7,
                Packet {
                    version,
                    contents: Contents::Operator(operator, contents),
                },
            )
        },
    );

    let parse_operator_count_packet = map(
        tuple((
            take(3usize),
            take(3usize),
            preceded(tag(1u8, 1usize), parse_operator_count),
        )),
        |(version, operator, (len, contents))| {
            (
                len + 7,
                Packet {
                    version,
                    contents: Contents::Operator(operator, contents),
                },
            )
        },
    );

    alt((
        parse_literal_packet,
        parse_operator_len_packet,
        parse_operator_count_packet,
    ))(input)
}

fn convert_hex(hex: &[u8]) -> Vec<u8> {
    hex.chunks_exact(2)
        .map(|chunk| u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16).unwrap())
        .collect()
}

pub fn part1(input: &mut dyn Read) -> String {
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer).unwrap();

    let binary_data = convert_hex(&buffer);
    let (_, (_, packet)) = parse_packet((&binary_data, 0)).unwrap();

    packet.version_sum().to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer).unwrap();

    let binary_data = convert_hex(&buffer);
    let (_, (_, packet)) = parse_packet((&binary_data, 0)).unwrap();

    packet.value().to_string()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[&[u8]] = &[
        &*b"8A004A801A8002F478",
        &*b"620080001611562C8802118E34",
        &*b"C0015000016115A2E0802F182340",
        &*b"A0016C880162017C3686B18A3D4780",
    ];

    #[test]
    fn sample_part1() {
        let answers = [16, 12, 23, 31];

        for (&sample, answer) in SAMPLE.into_iter().zip(answers) {
            test_implementation(part1, sample, answer);
        }
    }

    #[test]
    fn test_parse_literal() {
        let (_, (len, packet)) = parse_packet((&convert_hex(b"D2FE28"), 0)).unwrap();

        assert_eq!(len, 21);
        assert_eq!(
            packet,
            Packet {
                version: 6,
                contents: Contents::Literal(vec![7, 14, 5])
            }
        );
    }

    #[test]
    fn test_parse_operator_len() {
        let (_, (len, packet)) = parse_packet((&convert_hex(b"38006F45291200"), 0)).unwrap();

        assert_eq!(len, 22 + 27);
        assert_eq!(packet.version, 1);

        assert!(matches!(packet.contents, Contents::Operator(6, _)));
    }

    #[test]
    fn test_parse_operator_count() {
        let (_, (len, packet)) = parse_packet((&convert_hex(b"EE00D40C823060"), 0)).unwrap();

        assert_eq!(len, 7 + 11 + 33);
        assert_eq!(packet.version, 7);

        assert!(matches!(packet.contents, Contents::Operator(3, _)));
    }
}
