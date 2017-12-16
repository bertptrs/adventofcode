use std::io;
use std::collections::VecDeque;

fn hash(input: &str, output: &mut[u8])
{
    let mut buffer = [0u8; 256];
    for i in {0..256} {
        buffer[i] = i as u8;
    }

    let mut actual_key = String::from(input);
    // Needed to convert to hex
    actual_key += "\x11\x1f\x49\x2f\x17";

    let mut index = 0u8;
    let mut skip = 0u8;

    for _ in 0..64 {
        for c in actual_key.chars() {
            let offset = c as u8;

            for i in 0..(offset / 2) {
                let i1 = index.wrapping_add(i) as usize;
                let i2 = index.wrapping_add(offset).wrapping_sub(i).wrapping_sub(1) as usize;

                let temp = buffer[i1];
                buffer[i1] = buffer[i2];
                buffer[i2] = temp;
            }

            index = index.wrapping_add(offset).wrapping_add(skip);
            skip = skip.wrapping_add(1);
        }
    }

    for (i, b) in buffer.iter().enumerate() {
        output[i / 16] ^= b;
    }
}

fn count_ones(input: &[[u8; 16]]) -> u32
{
    let mut total: u32 = 0;
    for row in input {
        let row_total: u32 = row.iter().map(|x| x.count_ones()).sum();
        total += row_total;
    }

    return total;
}

fn get_hash(input: &str) -> [[u8; 16]; 128]
{
    let mut hash_buffer = [[0u8; 16]; 128];

    for i in 0..128 {
        let mut hash_key = String::from(input);
        hash_key += "-";
        hash_key += &i.to_string();

        hash(&hash_key, &mut hash_buffer[i]);
    }

    return hash_buffer;
}

fn is_used(hashes: &[[u8; 16]], y: usize, x: usize) -> bool
{
    let byte = hashes[y][x / 8];

    return (byte & (1 << (7 - (x % 8)))) != 0;
}

fn count_groups(hashes: &[[u8; 16]]) -> u32
{
    let mut visited = [[false; 128]; 128];

    let mut groups = 0u32;

    for x in 0..128 {
        for y in 0..128 {
            if visited[x][y] || !is_used(hashes, x, y) {
                continue;
            }
            visited[x][y] = true;
            groups += 1;

            let mut todo = VecDeque::new();
            todo.push_back((x, y));

            while !todo.is_empty() {
                let (cur_x, cur_y) = todo.pop_front().unwrap();

                let x_min = if cur_x == 0 { 0 } else { cur_x - 1 };
                let y_min = if cur_y == 0 { 0 } else { cur_y - 1 };
                let x_max = if cur_x == 127 { 128 } else { cur_x + 2 };
                let y_max = if cur_y == 127 { 128 } else { cur_y + 2 };

                for x_new in x_min..(x_max) {
                    for y_new in y_min..(y_max) {
                        if (cur_x != x_new) ^ (cur_y != y_new) && !visited[x_new][y_new] && is_used(hashes, x_new, y_new) {
                            visited[x_new][y_new] = true;
                            todo.push_back((x_new, y_new));
                        }
                    }
                }
            }
        }
    }

    return groups;
}


fn main()
{
    let reader = io::stdin();

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    let hashes = get_hash(input.trim());

    println!("Part 1: {}", count_ones(&hashes));
    println!("Part 2: {}", count_groups(&hashes));
}
