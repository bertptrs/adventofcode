use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::iterator;
use nom::combinator::map;
use nom::combinator::value;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;

#[derive(Copy, Clone)]
enum Instruction {
    AddX(i32),
    Noop,
}

impl Instruction {
    #[inline]
    pub fn cycles(self) -> i32 {
        match self {
            Instruction::AddX(_) => 2,
            Instruction::Noop => 1,
        }
    }

    #[inline]
    pub fn execute(self, x: &mut i32, cycle: &mut i32) {
        *cycle += self.cycles();

        if let Instruction::AddX(v) = self {
            *x += v;
        }
    }
}

fn parse_instruction(input: &[u8]) -> IResult<&[u8], Instruction> {
    terminated(
        alt((
            value(Instruction::Noop, tag("noop")),
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
        let old_cycle = cycle;
        let old_x = x;

        instruction.execute(&mut x, &mut cycle);

        if old_cycle % 40 < 20 && cycle % 40 >= 20 {
            let to_report = if cycle % 40 == 20 { x } else { old_x };

            let checkpoint = cycle / 20 * 20;
            total += to_report * checkpoint;
        }

        if cycle >= 220 {
            return Ok(total.to_string());
        }
    }

    anyhow::bail!("out of instructions")
}

pub fn part2(input: &[u8]) -> Result<String> {
    let mut x = 1;
    let mut input_it = iterator(input, parse_instruction);
    let mut input_it = (&mut input_it).peekable();

    let mut output = String::with_capacity(6 * (40 + 1));

    let mut cpu_cycle = 0;

    for crt_cycle in 1..=240 {
        if let Some(instruction) = input_it.next_if(|i| cpu_cycle + i.cycles() < crt_cycle) {
            instruction.execute(&mut x, &mut cpu_cycle);
        }

        let beam_pos = (crt_cycle + 39) % 40;

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

    #[test]
    fn sample_part2() {
        let answer = "##  ##  ##  ##  ##  ##  ##  ##  ##  ##  
###   ###   ###   ###   ###   ###   ### 
####    ####    ####    ####    ####    
#####     #####     #####     #####     
######      ######      ######      ####
#######       #######       #######     \n";

        assert_eq!(part2(SAMPLE).unwrap(), answer);
    }
}
