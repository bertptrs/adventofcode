use std::io;
use std::ops;
use std::cmp;
use std::io::prelude::*;

/// Apply Erathostenes's sieve to the supplied array
///
/// # Arguments
///
/// * `dest` - the destination slice to fill with the sieve. This is
///   assumed to be filled with "true" before being handed to this
///   method.
pub fn prime_sieve(dest: &mut[bool]) {
    if dest.len() >= 1 {
        dest[0] = false;
    }

    if dest.len() >= 2 {
        dest[1] = false;
    }

    let limit = (dest.len() as f64).sqrt() as usize;

    for i in 1..(limit + 1) {
        if !dest[i] {
            continue
        }

        for j in ((i * i)..(dest.len())).step_by(i) {
            dest[j] = false;
        }
    }
}

/// Greatest common divisor
pub fn gcd<T: ops::Rem<Output = T> + cmp::PartialOrd + std::convert::From<i32> + Copy>(a: T, b: T) -> T {
    if a < b {
        gcd(b, a)
    } else {
        if a % b == T::from(0) {
            b
        } else {
            gcd(a % b, b)
        }
    }
}

/// Least common multiple
pub fn lcm<T: ops::Rem<Output = T> + ops::Mul<Output = T> + ops::Div<Output = T> + cmp::PartialOrd + std::convert::From<i32> + Copy>(a: T, b: T) -> T {
    a * b / gcd(a, b)
}

/// Solution trait
///
/// Every day's solution should implement this function so that it can
/// be easily run from the main program.
pub trait Solution {
    /// Solve the first part of the day
    fn part1(&mut self, input: &mut io::Read) -> String;

    /// Solve the second part of the day
    fn part2(&mut self, input: &mut io::Read) -> String;
}

#[derive(Default)]
pub struct AssemBunnyCPU {
    instructions: Vec<Vec<String>>,
    pub registers: [i32; 4]
}

fn register_num(value: &str) -> Option<usize>
{
    match value {
        "a" => Some(0),
        "b" => Some(1),
        "c" => Some(2),
        "d" => Some(3),
        _ => None,
    }
}

impl AssemBunnyCPU {
    fn get_value(&self, value: &str) -> i32 {
        match register_num(value) {
            Some(num) => self.registers[num],
            None => value.parse().unwrap()
        }
    }

    pub fn read_instructions(&mut self, input: &mut io::Read) {
        let reader = io::BufReader::new(input);
        for line in reader.lines() {
            let contents = line.unwrap();
            let parts: Vec<String> = contents.split(" ").map(|part| String::from(part)).collect();

            self.instructions.push(parts);
        }
    }

    pub fn run(&mut self)-> i32 {
        let mut iptr: i32 = 0;

        while iptr < self.instructions.len() as i32 {
            let mut instruction_target = 0;
            let mut new_instruction = Vec::new();
            {
                let ref instruction = self.instructions[iptr as usize];
                match instruction[0].as_ref() {
                    "cpy" => {
                        let val = self.get_value(&instruction[1]);
                        match register_num(&instruction[2]) {
                            Some(num) => { self.registers[num] = val; },
                            None => {
                                // Invalid instruction generated.
                            },
                        }
                    },
                    "jnz" => {
                        let val = self.get_value(&instruction[1]);
                        if val != 0 {
                            let jump: i32 = self.get_value(&instruction[2]);
                            iptr += jump;
                            continue;
                        }
                    },
                    "inc" => {
                        match register_num(&instruction[1]) {
                            Some(num) => { self.registers[num] += 1 },
                            None => {
                                // Invalid instruction generated.
                            },
                        }
                    },
                    "dec" => {
                        match register_num(&instruction[1]) {
                            Some(num) => { self.registers[num] -= 1; },
                            None => {
                                // Invalid instruction generated.
                            },
                        }
                    },
                    "tgl" => {
                        instruction_target = (iptr + self.get_value(&instruction[1])) as usize;
                        if instruction_target < self.instructions.len() {
                            new_instruction = self.instructions[instruction_target].clone();
                            new_instruction[0] = String::from(match new_instruction.len() {
                                2 => match new_instruction[0].as_str() {
                                    "inc" => "dec",
                                    _ => "inc",
                                }
                                3 => match new_instruction[0].as_str() {
                                    "jnz" => "cpy",
                                    _ => "jnz",
                                },
                                _ => panic!("Cannot toggle instruction {}", instruction_target),
                            });
                        }
                    },
                    _ => panic!("Invalid instruction: {:?}", instruction),
                }
            }
            iptr += 1;

            // Check if we need to override an instruction
            if new_instruction.len() != 0 {
                self.instructions[instruction_target] = new_instruction;
            }
        }


        self.get_value("a")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_sieve() {
        let mut input = [true; 10];
        prime_sieve(&mut input);

        let output = [
            false, false,
            true, true,
            false, true,
            false, true,
            false, false
        ];

        assert_eq!(output, input);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(12, gcd(24, 36));
        assert_eq!(1, gcd(1, 7));
    }

    #[test]
    fn test_lcm() {
        assert_eq!(12, lcm(6, 4));
    }
}
