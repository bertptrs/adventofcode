use std::io::Read;

fn fish_growth(fish: &[u8], days: usize) -> usize {
    let mut fish_per_day = [0usize; 9];

    for &life in fish {
        fish_per_day[life as usize] += 1;
    }

    for day in 0..days {
        let index = day % fish_per_day.len();
        let offspring_today = fish_per_day[index];

        // The parents can be parents in 6 days
        fish_per_day[(index + 7) % fish_per_day.len()] += offspring_today;
        // The offspring from today will be ready the next time they come around
    }

    fish_per_day.into_iter().sum()
}

fn part_common(input: &mut dyn Read, days: usize) -> String {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let fish: Vec<u8> = buffer
        .trim_end()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    fish_growth(&fish, days).to_string()
}

pub fn part1(input: &mut dyn Read) -> String {
    part_common(input, 80)
}

pub fn part2(input: &mut dyn Read) -> String {
    part_common(input, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: [u8; 5] = [3, 4, 3, 1, 2];

    #[test]
    fn sample_part1() {
        assert_eq!(fish_growth(&SAMPLE, 1), 5);
        assert_eq!(fish_growth(&SAMPLE, 2), 6);
        assert_eq!(fish_growth(&SAMPLE, 3), 7);
        assert_eq!(fish_growth(&SAMPLE, 4), 9);
        assert_eq!(fish_growth(&SAMPLE, 18), 26);
        assert_eq!(fish_growth(&SAMPLE, 80), 5934);
    }

    #[test]
    fn sample_part2() {
        assert_eq!(fish_growth(&SAMPLE, 256), 26984457539);
    }
}
