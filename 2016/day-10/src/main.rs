extern crate regex;

use regex::Regex;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp::min;
use std::cmp::max;

fn give_value(values: &mut HashMap<i32, i32>, todo: &mut VecDeque<(i32, i32, i32)>, bot: i32, value: i32)
{
    if values.contains_key(&bot) {
        let current = values[&bot];
        values.remove(&bot);

        todo.push_back((bot, value, current))
    } else {
        values.insert(bot, value);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).expect("Could not open file");
    let reader = BufReader::new(f);

    let input_regex = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
    let action_regex = Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)").unwrap();

    let mut bot_values = HashMap::new();
    let mut bot_actions = HashMap::new();
    let mut action_queue = VecDeque::new();

    for line in reader.lines() {
        let contents = line.unwrap();
        match action_regex.captures(&contents) {
            Some(caps) => {
                let bot: i32 = caps[1].parse().unwrap();
                let t1 = &caps[2] == "bot";
                let n1: i32 = caps[3].parse().unwrap();
                let t2 = &caps[4] == "bot";
                let n2: i32 = caps[5].parse().unwrap();

                bot_actions.insert(bot, (t1, n1, t2, n2));
            },
            _ => {
                let caps = input_regex.captures(&contents).unwrap();
                let value: i32 = caps[1].parse().unwrap();
                let bot: i32 = caps[2].parse().unwrap();

                give_value(&mut bot_values, &mut action_queue, bot, value);
            }
        }
    }

    let mut bins = HashMap::new();

    while !action_queue.is_empty() {
        let (bot, val1, val2) = action_queue.pop_front().unwrap();

        let lo = min(val1, val2);
        let hi = max(val1, val2);

        if lo == 17 && hi == 61 {
            println!("Bot {} handling 17 and 61", bot);
        }

        let (tlo, nlo, thi, nhi) = bot_actions[&bot];

        if tlo {
            give_value(&mut bot_values, &mut action_queue, nlo, lo);
        } else {
            bins.insert(nlo, lo);
        }

        if thi {
            give_value(&mut bot_values, &mut action_queue, nhi, hi);
        } else {
            bins.insert(nhi, hi);
        }
    }

    let product = bins[&0] * bins[&1] * bins[&2];
    println!("Product: {}", product);
}
