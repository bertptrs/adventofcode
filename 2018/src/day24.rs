use std::cmp::Ordering;
use std::cmp::Reverse;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use itertools::Itertools;
use regex::Regex;

use common::Solution;

#[derive(Default, Debug, Eq, PartialEq, Clone)]
struct Group {
    power: u32,
    count: u32,
    hp: u32,
    initiative: i32,
    damage: String,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
    faction: char,
}

impl Group {
    pub fn is_alive(&self) -> bool {
        self.count > 0
    }

    pub fn damage_to(&self, other: &Group) -> u32 {
        let damage = self.effective_power();

        if other.weaknesses.contains(&self.damage) {
            damage * 2
        } else if other.immunities.contains(&self.damage) {
            0
        } else {
            damage
        }
    }

    pub fn effective_power(&self) -> u32 {
        self.power * self.count
    }
}

impl Ord for Group {
    fn cmp(&self, other: &Self) -> Ordering {
        self.effective_power().cmp(&other.effective_power())
            .then(other.initiative.cmp(&self.initiative))
    }
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default)]
pub struct Day24 {
    units: Vec<Group>,
}

impl Day24 {
    pub fn new() -> Self {
        Default::default()
    }

    fn read_input(&mut self, input: &mut Read) {
        let matcher = Regex::new(r"(\d+) units each with (\d+) hit points (\(([^)]+)\) )?with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();
        let weakness_matcher = Regex::new(r"(weak|immune) to ([^;)]+)").unwrap();
        let reader = BufReader::new(input);
        let mut fac = 'X';

        for line in reader.lines() {
            let line = line.unwrap();
            match line.as_str() {
                "Immune System:" => { fac = 'D' }
                "" => {}
                "Infection:" => { fac = 'I' }
                line => {
                    let caps = matcher.captures(line).unwrap_or_else(|| {
                        panic!("{}", line);
                    });

                    let mut group = Group {
                        count: caps[1].parse().unwrap(),
                        hp: caps[2].parse().unwrap(),
                        power: caps[5].parse().unwrap(),
                        damage: caps[6].to_string(),
                        initiative: caps[7].parse().unwrap(),
                        faction: fac,
                        ..Default::default()
                    };

                    if let Some(caps) = caps.get(4) {
                        for modified in weakness_matcher.captures_iter(caps.as_str()) {
                            let target = match &modified[1] {
                                "weak" => &mut group.weaknesses,
                                "immune" => &mut group.immunities,
                                _ => panic!(),
                            };

                            for t in modified[2].split(", ") {
                                target.push(t.to_string());
                            }
                        }
                    }
                    self.units.push(group)
                }
            }
        }
    }

    fn simulate(&mut self) {
        let mut order: Vec<usize> = (0..self.units.len()).collect();
        order.sort_unstable_by_key(|&x| &self.units[x]);
        order.reverse();

        // select targets
        let mut targets: Vec<Option<usize>> = vec![None; self.units.len()];
        let mut is_targeted = vec![false; self.units.len()];

        for &i in &order {
            let unit = &self.units[i];
            if !unit.is_alive() {
                continue;
            }
            let damage = self.units.iter().map(|x| unit.damage_to(x)).collect_vec();
            let target = (0..self.units.len())
                .filter(|&x| !is_targeted[x] && self.units[x].faction != unit.faction)
                .max_by(|&x, &y| {
                    damage[x].cmp(&damage[y])
                        .then(self.units[x].effective_power().cmp(&self.units[y].effective_power()))
                        .then(self.units[x].initiative.cmp(&self.units[y].initiative))
                });

            if let Some(target) = target {
                println!("{} will attack {}, dealing {}", i, target, damage[target]);
                targets[i] = Some(target);
                is_targeted[target] = true;
            }
        }

        order.sort_unstable_by_key(|&x| Reverse(self.units[x].initiative));
        println!("{:?}", order);

        for attacker in order {
            if !self.units[attacker].is_alive() {
                continue;
            }

            if let Some(id) = targets[attacker] {
                let damage = self.units[attacker].damage_to(&self.units[id]);
                let defender = &mut self.units[id];

                let losses = damage / defender.hp;
                println!("{} does {} damage to {}. Killed {}", attacker, damage, id, losses);
                defender.count = defender.count.saturating_sub(losses);
            }
        }
    }

    fn both_alive(&self) -> bool {
        let mut seen = None;

        for unit in self.units.iter().filter(|g| g.is_alive()) {
            if let Some(seen) = seen {
                if seen != unit.faction {
                    return true;
                }
                continue;
            }

            seen = Some(unit.faction);
        }

        return false;
    }
}

impl Solution for Day24 {
    fn part1(&mut self, input: &mut Read) -> String {
        self.read_input(input);
        while self.both_alive() {
            self.simulate();
        }

        let result: u32 = self.units.iter().map(|x| x.count).sum();
        result.to_string()
    }

    fn part2(&mut self, _input: &mut Read) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use common::Solution;
    use day24::Day24;

    const SAMPLE_INPUT: &[u8] = include_bytes!("samples/24.txt");

    #[test]
    fn sample_part1() {
        let mut instance = Day24::new();
        assert_eq!("5216", instance.part1(&mut SAMPLE_INPUT))
    }
}
