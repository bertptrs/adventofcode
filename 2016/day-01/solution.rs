use std::collections::HashSet;
use std::error::Error;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str;

fn dist(pos: (i32, i32)) -> i32 {
    let (x, y) = pos;

    return x.abs() + y.abs()
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

    let mut posx = 0;
    let mut posy = 0;
    let mut dir = 0;
    let mut found = false;

    let mut positions = HashSet::new();
    positions.insert((posx, posy));

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
            dir -= 1;
        }
        dir = (dir + 4) % 4;

        let backwards = if dir & 2 == 2 { -1 } else { 1 };
        if dir & 1 == 1 {
            // Move in y direction
            for y in 1..(steps + 1) {
                if positions.contains(&(posx, posy + y * backwards)) && !found {
                    found = true;
                    println!("Arrived at the same position, dist {}", dist((posx, posy + y * backwards)));
                } else {
                    positions.insert((posx, posy + y * backwards));
                }
            }
            posy += steps * backwards;
        } else {
            // Move in x direction
            for x in 1..(steps + 1) {
                if positions.contains(&(posx + x * backwards, posy)) && !found {
                    println!("Arrived at the same position, dist {}", dist((posx + x * backwards, posy)));
                    found = true;
                } else {
                    positions.insert((posx + x * backwards, posy));
                }
            }
            posx += steps * backwards;
        }
    }

    println!("Total distance is {}", dist((posx, posy)))

}
