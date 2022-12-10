use anyhow::Context;
use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::iterator;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;

#[derive(Copy, Clone, Debug)]
enum Instruction {
    AddX(i32),
    Noop,
}

impl Instruction {
    pub fn cycles(self) -> usize {
        match self {
            Instruction::AddX(_) => 2,
            Instruction::Noop => 1,
        }
    }
}

fn parse_instruction(input: &[u8]) -> IResult<&[u8], Instruction> {
    terminated(
        alt((
            map(tag("noop"), |_| Instruction::Noop),
            map(preceded(tag("addx "), nom::character::complete::i32), |v| {
                Instruction::AddX(v)
            }),
        )),
        newline,
    )(input)
}

pub fn part1(input: &[u8]) -> Result<String> {
    let mut x = 1;
    // Count from one like a scrub
    let mut cycle = 1;

    let mut input_it = iterator(input, parse_instruction);

    let mut total = 0;

    for instruction in &mut input_it {
        let cycles = instruction.cycles();

        let old_x = x;

        match instruction {
            Instruction::AddX(val) => x += val,
            Instruction::Noop => (),
        }

        if cycle % 40 < 20 && (cycle + cycles) % 40 >= 20 {
            let to_report = if (cycle + cycles) % 40 == 20 {
                x
            } else {
                old_x
            };

            let checkpoint = (cycle + cycles) / 20 * 20;
            total += to_report * (checkpoint as i32);
        }

        cycle += cycles;

        if cycle >= 220 {
            return Ok(total.to_string());
        }
    }

    anyhow::bail!("out of instructions")
}

pub fn part2(input: &[u8]) -> Result<String> {
    let mut x = 1;
    let mut input_it = iterator(input, parse_instruction);

    let mut output = String::with_capacity(226);

    let mut cpu_cycle = 0;

    let mut next_instruction = (&mut input_it).next().context("No instructions?")?;

    for crt_cycle in 1..=240 {
        while cpu_cycle + next_instruction.cycles() < crt_cycle {
            match next_instruction {
                Instruction::AddX(v) => x += v,
                Instruction::Noop => (),
            }

            cpu_cycle += next_instruction.cycles();
            next_instruction = (&mut input_it).next().unwrap_or(next_instruction);
        }

        let beam_pos = ((crt_cycle + 39) % 40) as i32;

        if (beam_pos - x).abs() <= 1 {
            output.push('#');
        } else {
            output.push(' ');
        }

        if crt_cycle % 40 == 0 {
            output.push('\n');
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/10.txt");

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "13140");
    }
}
