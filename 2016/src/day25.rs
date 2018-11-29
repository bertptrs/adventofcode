use std::io;
use common;

#[derive(Default)]
pub struct Day25 {
}

impl Day25 {

    pub fn new() -> Day25 {
        Default::default()
    }

}

impl common::Solution for Day25 {

    fn part1(&mut self, _input: &mut io::Read) -> String {
        let initial = 0b101010101010 - 362 * 7;
        println!("Initial value: {}", initial);
        sender_program(initial);
    }
}

/// This function is an approximation of what the original code is doing.
pub fn sender_program(mut a: i32) -> ! {
    // Placeholder variables
    let mut b;
    let mut d;

    d = a;
    d += 362 * 7;
    loop {
        if a == 0 {
            a = d;
        }
        b = a;
        a = b / 2;
        b = b % 2;
        println!("{}", b);
    }
}
