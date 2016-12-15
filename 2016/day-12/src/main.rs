use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

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

fn get_value(value: &str, registers: &[i32; 4]) -> i32
{
    match register_num(value) {
        Some(num) => registers[num],
        _ => value.parse().unwrap(),
    }
}

fn run(program: &Vec<Vec<String>>, mut registers: [i32; 4]) -> [i32; 4]
{
    let mut iptr: i32 = 0;

    while iptr < program.len() as i32 {
        let ref instruction = program[iptr as usize];
        //println!("{} ({:?}): {:?}", iptr, registers, instruction);
        match instruction[0].as_ref() {
            "cpy" => {
                let val = get_value(&instruction[1], &registers);
                let dest = register_num(&instruction[2]).unwrap();
                registers[dest] = val;
            },
            "jnz" => {
                let val = get_value(&instruction[1], &registers);
                if val != 0 {
                    let jump: i32 = instruction[2].parse().unwrap();
                    iptr += jump;
                    continue;
                }
            },
            "inc" => {
                let dest = register_num(&instruction[1]).unwrap();
                registers[dest] += 1;
            },
            "dec" => {
                let dest = register_num(&instruction[1]).unwrap();
                registers[dest] -= 1;
            },
            _ => panic!("Invalid instruction: {:?}", instruction),
        }
        iptr += 1;
    }

    return registers;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).expect("Could not open file");
    let reader = BufReader::new(f);

    let mut program = Vec::new();

    for line in reader.lines() {
        let contents = line.unwrap();
        let parts: Vec<String> = contents.split(" ").map(|part| String::from(part)).collect();

        program.push(parts);
    }

    let result1 = run(&program, [0; 4]);
    println!("Run 1: register a contains {}", result1[0]);

    let result2 = run(&program, [0, 0, 1, 0]);
    println!("Run 2: register a contains {}", result2[0]);
}
