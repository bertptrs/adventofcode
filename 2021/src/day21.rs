use std::io::Read;

use crate::common::LineIter;

fn read_input(input: &mut dyn Read) -> (i32, i32) {
    let mut reader = LineIter::new(input);

    let a = reader
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let b = reader
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse()
        .unwrap();

    (a, b)
}

#[inline]
fn simulate(die: i32, pos: i32) -> i32 {
    (pos + 3 * die + 3 - 1) % 10 + 1
}

fn find_repetition(mut pos: i32, mut die: i32) -> i32 {
    let mut advance = 0;

    for _ in 0..10 {
        pos = simulate(die, pos);
        advance += pos;
        die = (die + 6) % 10;
    }

    advance
}

pub fn part1(input: &mut dyn Read) -> String {
    const TARGET_SCORE: i32 = 1000;

    let (a, b) = read_input(input);

    let a10 = find_repetition(a, 1);
    let b10 = find_repetition(b, 4);

    let a_win = TARGET_SCORE / a10;
    let b_win = TARGET_SCORE / b10;

    let mut rolls = 3 * 20 * a_win.min(b_win);
    let mut a_score = rolls / 3 / 20 * a10;
    let mut b_score = rolls / 3 / 20 * b10;
    let mut a_pos = a;
    let mut b_pos = b;
    let mut die = 1;

    let (loser_score, rolls) = if a_win < b_win {
        while a_score < TARGET_SCORE {
            a_pos = simulate(die, a_pos);
            a_score += a_pos;
            rolls += 3;

            if a_score < TARGET_SCORE {
                b_pos = simulate(die + 3, b_pos);
                b_score += b_pos;
                rolls += 3;
            }

            die = (die + 6) % 10;
        }

        (b_score, rolls)
    } else {
        while b_score < TARGET_SCORE {
            a_pos = simulate(die, a_pos);
            a_score += a_pos;

            b_pos = simulate(die + 3, b_pos);
            b_score += b_pos;

            rolls += 6;

            die = (die + 6) % 10;
        }

        (a_score, rolls)
    };

    (loser_score * rolls).to_string()
}

const MAX_TURNS: usize = 21; // Trivial upper bound, could be lower
const ROLLS: [i32; 7] = [3, 4, 5, 6, 7, 8, 9];
const FREQS: [u64; 7] = [1, 3, 6, 7, 6, 3, 1];

fn multiverse_recursive(
    remaining: &mut [u64],
    wins: &mut [u64],
    turn: usize,
    alive: u64,
    pos: i32,
    score: i32,
) {
    for (roll, freq) in ROLLS.into_iter().zip(FREQS) {
        let new_pos = (pos + roll - 1) % 10 + 1;
        let new_score = score + new_pos;
        let new_alive = alive * freq;

        if new_score >= 21 {
            wins[turn] += new_alive;
        } else {
            remaining[turn] += new_alive;
            multiverse_recursive(remaining, wins, turn + 1, new_alive, new_pos, new_score);
        }
    }
}

fn multiverse(pos: i32) -> ([u64; MAX_TURNS], [u64; MAX_TURNS]) {
    let mut alive = [0; MAX_TURNS];
    let mut wins = [0; MAX_TURNS];

    alive[0] = 1;

    multiverse_recursive(&mut alive, &mut wins, 1, 1, pos, 0);

    (wins, alive)
}

pub fn part2(input: &mut dyn Read) -> String {
    let (a, b) = read_input(input);

    let (a_wins, a_alive) = multiverse(a);
    let (b_wins, b_alive) = multiverse(b);

    let a_winner: u64 = a_wins[1..].iter().zip(b_alive).map(|(&a, b)| a * b).sum();
    let b_winner: u64 = b_wins.into_iter().zip(a_alive).map(|(a, b)| a * b).sum();

    a_winner.max(b_winner).to_string()
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("samples/21.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 739785);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 444356092776315u64);
    }
}
