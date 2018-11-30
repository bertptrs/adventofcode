use common;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Default)]
pub struct Day21 {
    password: Vec<char>,
}

impl Day21 {

    pub fn new() -> Day21 {
        Default::default()
    }


    fn swap(&mut self, by: &str, id_1: &str, id_2: &str)
    {
        let pos_1: usize;
        let pos_2: usize;
        match by {
            "position" => {
                pos_1 = id_1.parse().unwrap();
                pos_2 = id_2.parse().unwrap();
            },
            "letter" => {
                pos_1 = self.find(id_1);
                pos_2 = self.find(id_2);
            },
            _ => panic!("Can't swap {}", by),
        }

        self.password.swap(pos_1, pos_2);
    }

    fn find(&self, search: &str) -> usize {
        let c = search.chars().next().unwrap();
        self.password.iter().position(|&x| x == c).unwrap()
    }

    fn reverse_range(&mut self, pos_1: usize, pos_2: usize)
    {
        self.password[pos_1..=pos_2].reverse();
    }
}

impl common::Solution for Day21 {
    fn part1(&mut self, input: &mut Read) -> String {
        self.password = "abcdefgh".chars().collect();
        let reader = BufReader::new(input);

        for line in reader.lines() {
            let contents = line.unwrap();
            let parts: Vec<&str> = contents.split(" ").collect();

            match parts[0] {
                "swap" => {
                    self.swap(&parts[1], &parts[2], &parts[5]);
                },
                "rotate" => {
                    let amount = match parts[1] {
                        "left" => parts[2].parse().unwrap(),
                        "right" => self.password.len() - parts[2].parse::<usize>().unwrap(),
                        "based" => {
                            let pos = self.find(parts[6]);

                            2 * self.password.len() - if pos >= 4 { pos + 2 } else { pos + 1 }
                        },

                        _ => panic!("Cannot rotate by {}", parts[2]),
                    } % self.password.len();
                    self.password.rotate_left(amount);
                },

                "reverse" => {
                    let pos_1: usize = parts[2].parse().unwrap();
                    let pos_2: usize = parts[4].parse().unwrap();
                    self.reverse_range(pos_1, pos_2);
                },
                "move" => {
                    let pos_1: usize = parts[2].parse().unwrap();
                    let pos_2: usize = parts[5].parse().unwrap();

                    let c = self.password.remove(pos_1);
                    self.password.insert(pos_2, c);
                },
                _ => panic!("Don't understand {}", parts[0]),
            }
        }

        self.password.iter().collect()
    }

    fn part2(&mut self, input: &mut Read) -> String {
        let reader = BufReader::new(input);
        let rules: Vec<String> = reader.lines()
            .map(|x| x.unwrap())
            .collect();
        self.password = "fbgdceah".chars().collect();

        for line in rules.iter().rev() {
            let parts: Vec<&str> = line.split(" ").collect();

            match parts[0] {
                "swap" => {
                    self.swap(&parts[1], &parts[2], &parts[5]);
                },
                "rotate" => {
                    // invert regular rotations, and
                    let amount = match parts[1] {
                        "right" => parts[2].parse().unwrap(),
                        "left" => self.password.len() - parts[2].parse::<usize>().unwrap(),
                        "based" => {
                            let pos = self.find(parts[6]);

                            let mut original = usize::max_value();
                            if pos % 2 == 1 {
                                // original pos < 4
                                for x in 0..4 {
                                    if (2 * x + 1) % self.password.len() == pos {
                                        original = x + 1;
                                        break;
                                    }
                                }
                            } else {
                                // original pos >= 4
                                for x in 4..self.password.len() {
                                    if (2 * x + 2) % self.password.len() == pos {
                                        original = x + 2;
                                    }
                                }
                            }

                            original
                        },

                        _ => panic!("Cannot rotate by {}", parts[2]),
                    } % self.password.len();
                    self.password.rotate_left(amount);
                },

                "reverse" => {
                    let pos_1: usize = parts[2].parse().unwrap();
                    let pos_2: usize = parts[4].parse().unwrap();
                    self.reverse_range(pos_1, pos_2);
                },
                "move" => {
                    let pos_1: usize = parts[2].parse().unwrap();
                    let pos_2: usize = parts[5].parse().unwrap();

                    let c = self.password.remove(pos_2);
                    self.password.insert(pos_1, c);
                },
                _ => panic!("Don't understand {}", parts[0]),
            }
        }

        self.password.iter().collect()
    }
}
