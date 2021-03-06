use std::collections::HashMap;
use std::io;
use std::io::BufRead;

use chrono::offset::TimeZone;
use chrono::offset::Utc;
use chrono::DateTime;
use chrono::Timelike;
use regex::Regex;

use common;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
enum EventType {
    WAKE,
    SLEEP,
    SHIFT(usize),
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
struct Event {
    time: DateTime<Utc>,
    event: EventType,
}

#[derive(Default)]
pub struct Day04 {
    events: Vec<Event>,
}

impl Day04 {
    pub fn new() -> Day04 {
        Default::default()
    }

    fn read_events(&mut self, input: &mut dyn io::Read) {
        self.events.clear();
        let reader = io::BufReader::new(input);

        let scanner = Regex::new(r"^\[([^]]+)] (Guard #(\d+)|falls asleep|wakes up)").unwrap();

        for line in reader.lines() {
            let line = line.unwrap();
            let captures = scanner.captures(&line).unwrap();
            let timestamp = Utc
                .datetime_from_str(&captures[1], "%Y-%m-%d %H:%M")
                .unwrap();

            let event = match &captures[2] {
                "falls asleep" => EventType::SLEEP,
                "wakes up" => EventType::WAKE,
                _ => EventType::SHIFT(captures[3].parse().unwrap()),
            };

            self.events.push(Event {
                time: timestamp,
                event,
            });
        }

        self.events.sort_unstable();
    }

    fn get_sleeps(&self) -> HashMap<usize, [u32; 60]> {
        let mut sleeps = HashMap::new();
        let mut guard: Option<usize> = None;
        let mut sleep_start: Option<DateTime<Utc>> = None;

        for event in &self.events {
            match &event.event {
                EventType::SHIFT(val) => {
                    guard = Some(*val);
                    sleep_start = None;
                }
                EventType::SLEEP => {
                    sleep_start = Some(event.time);
                }
                EventType::WAKE => {
                    let minutes = sleeps.entry(guard.unwrap()).or_insert([0u32; 60]);
                    for m in sleep_start.unwrap().minute()..event.time.minute() {
                        minutes[m as usize] += 1;
                    }
                }
            }
        }

        sleeps
    }

    fn format_results(
        sleepers: &HashMap<usize, [u32; 60]>,
        scores: &HashMap<usize, u32>,
    ) -> String {
        let (best_sleeper, _) = scores.iter().max_by(|&(_, a), &(_, b)| a.cmp(b)).unwrap();

        let best_minute = sleepers[best_sleeper]
            .iter()
            .enumerate()
            .max_by(|&(_, a), &(_, b)| a.cmp(b))
            .unwrap()
            .0;

        (best_sleeper * (best_minute as usize)).to_string()
    }
}

impl common::Solution for Day04 {
    fn part1(&mut self, input: &mut dyn io::Read) -> String {
        self.read_events(input);
        let sleepers = self.get_sleeps();
        let scores: HashMap<usize, u32> =
            sleepers.iter().map(|(k, v)| (*k, v.iter().sum())).collect();

        Day04::format_results(&sleepers, &scores)
    }

    fn part2(&mut self, input: &mut dyn io::Read) -> String {
        self.read_events(input);
        let sleepers = self.get_sleeps();
        let scores: HashMap<usize, u32> = sleepers
            .iter()
            .map(|(k, v)| (*k, *v.iter().max().unwrap()))
            .collect();

        Day04::format_results(&sleepers, &scores)
    }
}

#[cfg(test)]
mod tests {
    use common::Solution;
    use day04::Day04;

    const SAMPLE_INPUT: &[u8] = include_bytes!("samples/04.txt");

    #[test]
    fn sample_part1() {
        let mut instance = Day04::new();
        assert_eq!("240", instance.part1(&mut SAMPLE_INPUT));
    }

    #[test]
    fn sample_part2() {
        let mut instance = Day04::new();
        assert_eq!("4455", instance.part2(&mut SAMPLE_INPUT));
    }
}
