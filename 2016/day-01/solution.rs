use std::collections::HashSet;
use std::error::Error;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str;

fn dist(pos: (i32, i32)) -> i32
{
    let (x, y) = pos;

    return x.abs() + y.abs()
}

fn walk(pos: (i32, i32), dir: i32) -> (i32, i32)
{
    let backwards = if dir & 2 == 2 { -1 } else { 1 };
    let (x, y) = pos;

    (x + backwards * (1 - (dir & 1)), y + backwards * (dir & 1))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(file) => file,
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(_) => {},
    };

    let mut pos = (0, 0);
    let mut dir = 0;
    let mut found = false;

    let mut positions = HashSet::new();
    positions.insert(pos);

    for instruction in content.split(", ") {
        let turn = &instruction[..1];
        let steps_opt: Option<i32> = instruction[1..].trim().parse().ok();
        let steps = match steps_opt {
            Some(num) => num,
            None => panic!("Could note parse number of steps"),
        };
        if turn == "R" {
            dir += 1;
        } else {
            dir += 3;
        }
        dir %= 4;

        for _ in 0..steps {
            pos = walk(pos, dir);
            if !found && positions.contains(&pos) {
                println!("Visited twice at dist {}", dist(pos)) ;
                found = true;
            } else {
                positions.insert(pos);
            }
        }
    }

    println!("Total distance is {}", dist(pos));
}
