use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn is_splitpoint(c: char) -> bool
{
    match c {
        ' ' => true,
        'x' => true,
        'y' => true,
        '=' => true,
        _ => false,
    }
}

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).expect("Could not open file");
    let reader = BufReader::new(f);

    let mut lights = [[false; HEIGHT]; WIDTH];
    for line in reader.lines() {
        let contents = line.unwrap();
        let parts: Vec<&str> = contents.split(|c| is_splitpoint(c)).collect();

        match parts[0] {
            "rect" => {
                let width: usize = parts[1].parse().unwrap();
                let height: usize = parts[2].parse().unwrap();
                for x in 0..width {
                    for y in 0..height {
                        lights[x][y] = true;
                    }
                }
            },
            "rotate" => {
                let index: usize = parts[4].parse().expect("Invalid index");
                let amount: usize = parts[7].parse().expect("Invalid row");
                match parts[1] {
                    "row" => {
                        let mut copy = [false; WIDTH];
                        for x in 0..WIDTH {
                            copy[x] = lights[x][index];
                        }
                        for x in 0..WIDTH {
                            lights[(x + amount) % WIDTH][index] = copy[x];
                        }
                    },
                    "column" => {
                        let mut copy = [false; HEIGHT];
                        for y in 0..HEIGHT {
                            copy[y] = lights[index][y];
                        }
                        for y in 0..HEIGHT {
                            lights[index][(y + amount) % HEIGHT] = copy[y];
                        }
                    }
                    _ => panic!("{} is not a supported rotation", parts[1]),
                }
            },
            _ => panic!("{} is not a supported operation", parts[0]),
        }
    }

    let mut count = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut c = ' ';
            if lights[x][y] {
                count += 1;
                c = '★';
            }
            print!("{}", c);

            // spacing between letters
            if x % 5 == 4 {
                print!(" ");
            }
        }
        println!("");
    }

    println!("{} lights active.", count);
}
