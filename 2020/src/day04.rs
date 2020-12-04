use std::io::Read;

use crate::common::read_single_input;
use crate::Solution;

#[derive(Default)]
pub struct Day04;

fn is_valid(entry: &str) -> bool {
    let required_fields = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];

    required_fields.iter().all(|&f| entry.contains(f))
}

fn validate_year(value: &str, min: i32, max: i32) -> bool {
    if value.len() != 4 {
        return false;
    }

    if let Ok(value) = value.parse::<i32>() {
        (min..=max).contains(&value)
    } else {
        false
    }
}

fn validate_length(value: &str) -> bool {
    if let Ok(num) = value[..(value.len() - 2)].parse::<i32>() {
        if value.ends_with("cm") {
            return (150..=193).contains(&num);
        } else if value.ends_with("in") {
            return (59..=76).contains(&num);
        }
    }

    false
}

fn validate_hcl(value: &str) -> bool {
    if !value.starts_with('#') || value.len() != 7 {
        return false;
    }

    value[1..].chars().all(|c| c.is_ascii_hexdigit())
}

fn is_valid2(entry: &str) -> bool {
    if !is_valid(entry) {
        return false;
    }

    let valid_ecl = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    for key_value in entry.trim().split(|s: char| s.is_whitespace()) {
        let (key, value) = key_value.split_at(key_value.find(':').unwrap());
        let value = &value[1..];

        let valid = match key {
            "byr" => validate_year(value, 1920, 2002),
            "iyr" => validate_year(value, 2010, 2020),
            "eyr" => validate_year(value, 2020, 2030),
            "hgt" => validate_length(value),
            "hcl" => validate_hcl(value),
            "ecl" => valid_ecl.contains(&value),
            "pid" => value.len() == 9 && value.chars().all(|c| c.is_ascii_digit()),
            "cid" => true,
            _ => panic!("No validation rule for {}", key),
        };

        if !valid {
            return false;
        }
    }

    true
}

impl Solution for Day04 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let buffer: String = read_single_input(input);

        buffer
            .split("\n\n")
            .filter(|&s| is_valid(s))
            .count()
            .to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let buffer: String = read_single_input(input);

        buffer
            .split("\n\n")
            .filter(|&s| is_valid2(s))
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        assert!(is_valid(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm"
        ));
        assert!(!is_valid(
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929"
        ));
        assert!(is_valid(
            "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm"
        ));
        assert!(!is_valid(
            "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in"
        ));
    }
}
