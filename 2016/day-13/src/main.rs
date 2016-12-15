use std::collections::HashSet;
use std::collections::VecDeque;

const INPUT: i64 = 1352;

fn is_wall(x: i64, y: i64) -> bool
{
    if x < 0 || y < 0 {
        return true;
    }

    let num = x*x + 3*x + 2*x*y + y + y*y + INPUT;

    num.count_ones() % 2 == 1
}

fn main() {

    let target = (31, 39);

    let mut visited = HashSet::new();
    visited.insert((1i64, 1i64));

    let mut todo = VecDeque::new();
    todo.push_back((0, 1i64, 1i64));

    let mut count = 1;

    while !todo.is_empty()
    {
        let (dist, x, y) = todo.pop_front().unwrap();

        for i in -1..2 { // -1, 0, 1
            for j in -1..2 {
                let new_pos = (x + i, y + j);

                if i.abs() == j.abs() || is_wall(x + i, y + j) || visited.contains(&new_pos) {
                    continue;
                }

                if new_pos == target {
                    println!("Reached pos {:?} at dist {}", new_pos, dist + 1);
                    break;
                }

                todo.push_back((dist + 1, x + i, y + j));

                visited.insert(new_pos);
                if dist < 50 {
                    count += 1;
                }
            }
        }
    }

    println!("Visisted {} locations withing 50 steps", count);
}
