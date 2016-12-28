const INPUT: &'static str = ".^^^.^.^^^^^..^^^..^..^..^^..^.^.^.^^.^^....^.^...^.^^.^^.^^..^^..^.^..^^^.^^...^...^^....^^.^^^^^^^";

fn count_row(row: &[bool]) -> usize
{
    return row.iter().filter(|&&b| !b).count();
}

fn get_neighbours(row: &[bool], index: usize) -> [bool; 3]
{
    let mut output = [false; 3];
    output[0] = if index > 0 { row[index - 1] } else { false };
    output[1] = row[index];
    output[2] = if index < row.len() - 1 { row[index + 1] } else { false };

    return output;
}

fn solve_for(n: i32)
{
    let mut row: Vec<bool> = INPUT.chars().map(|c| c == '^').collect();
    let mut count = count_row(&row);

    for _ in 1..n {
        let mut new_row = row.clone();

        for j in 0..row.len() {
            let neighbours = get_neighbours(&row, j);
            new_row[j] = (neighbours[0] && neighbours[1] && !neighbours[2])
                || (!neighbours[0] && neighbours[1] && neighbours[2])
                || (neighbours[0] && !neighbours[1] && !neighbours[2])
                || (!neighbours[0] && !neighbours[1] && neighbours[2]);
        }

        count += count_row(&new_row);
        row = new_row;
    }

    println!("{} safe spaces in {} rows", count, n);
}

fn main() {
    solve_for(40);
    solve_for(400_000);
}
