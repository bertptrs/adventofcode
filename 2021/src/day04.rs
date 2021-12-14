use std::io::Read;

use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::Finish;
use nom::IResult;

#[derive(Debug)]
struct BingoCard([(bool, u8); 25]);

impl BingoCard {
    pub fn cross(&mut self, num: u8) -> Option<usize> {
        self.0
            .iter_mut()
            .enumerate()
            .find_map(|(pos, (ticked, x))| {
                if *x == num {
                    *ticked = true;
                    Some(pos)
                } else {
                    None
                }
            })
    }

    pub fn has_won(&self, crossed: usize) -> bool {
        // Check horizontal lines
        if self
            .0
            .chunks_exact(5)
            .nth(crossed / 5)
            .unwrap()
            .iter()
            .all(|&b| b.0)
        {
            return true;
        }

        // Check vertical lines
        self.0.iter().skip(crossed % 5).step_by(5).all(|b| b.0)

        // Diagonals do not count
    }

    pub fn remaining(&self) -> u32 {
        self.0
            .iter()
            .filter_map(|&(ticked, num)| if !ticked { Some(num as u32) } else { None })
            .sum()
    }
}

fn parse_numbers(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    use nom::character::complete::u8;
    separated_list1(tag(","), u8)(input)
}

fn parse_bingo(mut input: &[u8]) -> IResult<&[u8], BingoCard> {
    use nom::character::complete::u8;

    let mut card = [0; 25];

    let mut parse_num = preceded(multispace1, u8);

    // fill doesn't work with preceded
    for i in 0..25 {
        let result = parse_num(input)?;
        card[i] = result.1;
        input = result.0;
    }

    Ok((input, BingoCard(card.map(|x| (false, x)))))
}

fn parse_input(input: &[u8]) -> IResult<&[u8], (Vec<u8>, Vec<BingoCard>)> {
    tuple((parse_numbers, many1(parse_bingo)))(input)
}

fn read_input(input: &mut dyn Read) -> (Vec<u8>, Vec<BingoCard>) {
    let mut buffer = Vec::new();

    input.read_to_end(&mut buffer).unwrap();

    parse_input(&buffer).finish().unwrap().1
}

pub fn part1(input: &mut dyn Read) -> String {
    let (numbers, mut bingo_cards) = read_input(input);

    for number in numbers {
        for card in &mut bingo_cards {
            if matches!(card.cross(number), Some(pos) if card.has_won(pos)) {
                return (number as u32 * card.remaining()).to_string();
            }
        }
    }

    panic!("None of the cards won")
}

pub fn part2(input: &mut dyn Read) -> String {
    let (numbers, mut bingo_cards) = read_input(input);
    let mut bingo_won = vec![false; bingo_cards.len()];
    let mut num_won = 0;
    let to_win = bingo_cards.len();

    for num in numbers {
        for (won, card) in bingo_won.iter_mut().zip(bingo_cards.iter_mut()) {
            if *won {
                continue;
            }

            if matches!(card.cross(num), Some(pos) if card.has_won(pos)) {
                *won = true;
                num_won += 1;

                if num_won == to_win {
                    return (num as u32 * card.remaining()).to_string();
                }
            }
        }
    }

    panic!("Not all cards won!")
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/04.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 4512)
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 1924)
    }
}
