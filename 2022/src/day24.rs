use std::collections::VecDeque;
use std::mem;

use ahash::AHashSet;
use anyhow::Context;
use anyhow::Result;
use strength_reduce::StrengthReducedUsize;

fn gcd(mut a: usize, mut b: usize) -> usize {
    while a % b != 0 {
        b = a % b;
        mem::swap(&mut a, &mut b);
    }

    b
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[derive(Debug)]
struct Storm {
    // Dimensions of the entire area. Includes the wall
    width: usize,
    height: usize,

    // Periods of repetition. Basically the dimensions without the wall
    width_period: StrengthReducedUsize,
    height_period: StrengthReducedUsize,
    combined_period: StrengthReducedUsize,

    // Flying blizzards by direction and starting point
    left_right: AHashSet<(usize, usize)>,
    right_left: AHashSet<(usize, usize)>,
    top_bottom: AHashSet<(usize, usize)>,
    bottom_top: AHashSet<(usize, usize)>,
}

impl Storm {
    /// Whether you can stand in the given position at the given time
    fn can_stand(&self, time: usize, (x, y): (usize, usize)) -> bool {
        !self
            .right_left
            .contains(&((x + time) % self.width_period, y))
            && !self
                .bottom_top
                .contains(&(x, (y + time) % self.height_period))
            && !self.left_right.contains(&(
                (self.width_period.get() - time % self.width_period + x) % self.width_period,
                y,
            ))
            && !self.top_bottom.contains(&(
                x,
                (self.height_period.get() - time % self.height_period + y) % self.height_period,
            ))
    }
}

impl TryFrom<&'_ [u8]> for Storm {
    type Error = anyhow::Error;

    fn try_from(value: &'_ [u8]) -> Result<Self, Self::Error> {
        let width = value
            .iter()
            .position(|&b| b == b'\n')
            .context("Could not find end of line")?;
        let height = value.len() / (width + 1);

        let width_period = StrengthReducedUsize::new(width - 2);
        let height_period = StrengthReducedUsize::new(height - 2);
        let combined_period = StrengthReducedUsize::new(lcm(width - 2, height - 2));

        let mut left_right = AHashSet::new();
        let mut right_left = AHashSet::new();
        let mut top_bottom = AHashSet::new();
        let mut bottom_top = AHashSet::new();

        for (y, line) in value
            .split(|&b| b == b'\n')
            .enumerate()
            .skip(1)
            .take(height - 2)
        {
            for (x, &c) in line.iter().enumerate() {
                match c {
                    b'>' => left_right.insert((x % width_period, y)),
                    b'<' => right_left.insert((x % width_period, y)),
                    b'v' => top_bottom.insert((x, y % height_period)),
                    b'^' => bottom_top.insert((x, y % height_period)),
                    _ => continue,
                };
            }
        }

        Ok(Storm {
            width,
            height,
            width_period,
            height_period,
            combined_period,
            left_right,
            right_left,
            top_bottom,
            bottom_top,
        })
    }
}

pub fn part1(input: &[u8]) -> Result<String> {
    let storm = Storm::try_from(input)?;
    let goal = (storm.width - 2, storm.height - 1);

    let mut todo = VecDeque::new();
    todo.push_back((0, (1, 0)));

    let mut visited = AHashSet::new();

    while let Some((time, pos)) = todo.pop_front() {
        let mut enqueue = |pos| {
            let new_time = time + 1;

            if storm.can_stand(new_time, pos)
                && visited.insert((new_time % storm.combined_period, pos))
            {
                todo.push_back((new_time, pos));
            }
        };

        // Waiting is perhaps an option
        enqueue(pos);

        if pos.0 > 1 {
            enqueue((pos.0 - 1, pos.1));
        }

        if pos.1 > 0 && pos.0 < storm.width - 2 {
            enqueue((pos.0 + 1, pos.1));
        }

        if pos.1 > 1 {
            enqueue((pos.0, pos.1 - 1));
        }

        if pos.0 > 0 && pos.1 < storm.height - 2 {
            enqueue((pos.0, pos.1 + 1));
        }

        if (pos.0, pos.1 + 1) == goal {
            return Ok((time + 1).to_string());
        }
    }

    anyhow::bail!("Did not find a route to {goal:?}")
}

pub fn part2(_input: &[u8]) -> Result<String> {
    anyhow::bail!("not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("./samples/24.txt");

    #[test]
    fn test_can_stand() {
        let storm = Storm::try_from(SAMPLE).unwrap();
        dbg!(&storm);

        // Test a storm moving right to left
        assert!(storm.can_stand(0, (4, 2)));
        assert!(!storm.can_stand(1, (4, 2)));
        assert!(!storm.can_stand(0, (6, 2)));
        assert!(storm.can_stand(1, (6, 2)));

        // Test a storm moving bottom to top
        assert!(!storm.can_stand(0, (4, 4)));
        assert!(storm.can_stand(1, (4, 4)));

        // Simple moving to the right
        assert!(!storm.can_stand(0, (1, 1)));
        assert!(storm.can_stand(1, (1, 1)));

        assert!(storm.can_stand(0, (1, 2)));
        assert!(!storm.can_stand(1, (1, 2)));
    }

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), "18");
    }
}
