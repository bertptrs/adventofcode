use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn vec_str(vec: &[char]) -> String
{
    return vec.iter().cloned().collect();
}

fn find(password: &[char], search: &str) -> usize
{
    let c = search.chars().next().unwrap();
    for (index, b) in password.iter().enumerate() {
        if c == *b {
            return index;
        }
    }
    panic!("{} not found in {}", c, vec_str(password));
}

fn rotate_left(password: &[char], amount: usize) -> Vec<char>
{
    let mut new_passwd = Vec::with_capacity(password.len());

    new_passwd.extend_from_slice(&password[amount..]);
    new_passwd.extend_from_slice(&password[..amount]);

    return new_passwd;
}

fn reverse_range(password: &mut[char], pos_1: usize, pos_2: usize)
{
    password[pos_1..pos_2 + 1].reverse();
}

fn swap(password: &mut[char], by: &str, id_1: &str, id_2: &str)
{
    let pos_1: usize;
    let pos_2: usize;
    match by {
        "position" => {
            pos_1 = id_1.parse().unwrap();
            pos_2 = id_2.parse().unwrap();
        },
        "letter" => {
            pos_1 = find(password, id_1);
            pos_2 = find(password, id_2);
        },
        _ => panic!("Can't swap {}", by),
    }

    password.swap(pos_1, pos_2);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).expect("Could not open file");
    let reader = BufReader::new(f);

    let mut password: Vec<char> = "abcdefgh".chars().collect();
    let mut rules = Vec::new();

    for line in reader.lines() {
        let contents = line.unwrap();
        let parts: Vec<&str> = contents.split(" ").collect();
        rules.push(contents.clone());

        match parts[0] {
            "swap" => {
                swap(&mut password, &parts[1], &parts[2], &parts[5]);
            },
            "rotate" => {
                let amount = match parts[1] {
                    "left" => parts[2].parse().unwrap(),
                    "right" => password.len() - parts[2].parse::<usize>().unwrap(),
                    "based" => {
                        let pos = find(&password, parts[6]);

                        2 * password.len() - if pos >= 4 { pos + 2 } else { pos + 1 }
                    },

                    _ => panic!("Cannot rotate by {}", parts[2]),
                } % password.len();
                password = rotate_left(&password, amount);
            },

            "reverse" => {
                let pos_1: usize = parts[2].parse().unwrap();
                let pos_2: usize = parts[4].parse().unwrap();
                reverse_range(&mut password, pos_1, pos_2);
            },
            "move" => {
                let pos_1: usize = parts[2].parse().unwrap();
                let pos_2: usize = parts[5].parse().unwrap();

                let c = password.remove(pos_1);
                password.insert(pos_2, c);
            },
            _ => panic!("Don't understand {}", parts[0]),
        }
    }

    println!("Final password is {}", vec_str(&password));
    let mut password: Vec<char> = "fbgdceah".chars().collect();

    for line in rules.iter().rev() {
        let parts: Vec<&str> = line.split(" ").collect();

        match parts[0] {
            "swap" => {
                swap(&mut password, &parts[1], &parts[2], &parts[5]);
            },
            "rotate" => {
                // invert regular rotations, and
                let amount = match parts[1] {
                    "right" => parts[2].parse().unwrap(),
                    "left" => password.len() - parts[2].parse::<usize>().unwrap(),
                    "based" => {
                        let pos = find(&password, parts[6]);

                        let mut original = usize::max_value();
                        if pos % 2 == 1 {
                            // original pos < 4
                            for x in 0..4 {
                                if (2 * x + 1) % password.len() == pos {
                                    original = x + 1;
                                    break;
                                }
                            }
                        } else {
                            // original pos >= 4
                            for x in 4..password.len() {
                                if (2 * x + 2) % password.len() == pos {
                                    original = x + 2;
                                }
                            }
                        }

                        original
                    },

                    _ => panic!("Cannot rotate by {}", parts[2]),
                } % password.len();
                password = rotate_left(&password, amount);
            },

            "reverse" => {
                let pos_1: usize = parts[2].parse().unwrap();
                let pos_2: usize = parts[4].parse().unwrap();
                reverse_range(&mut password, pos_1, pos_2);
            },
            "move" => {
                let pos_1: usize = parts[2].parse().unwrap();
                let pos_2: usize = parts[5].parse().unwrap();

                let c = password.remove(pos_2);
                password.insert(pos_1, c);
            },
            _ => panic!("Don't understand {}", parts[0]),
        }
    }

    println!("Reversed password is {}", vec_str(&password));

}
