use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;

use regex::bytes::Regex;

use crate::common::Lines;
use crate::Solution;

#[derive(Clone, Debug)]
struct Tile {
    grid: Vec<Vec<bool>>,
    id: u64,
    // NESW order
    sides: [Vec<bool>; 4],
    rotations: u8,
    flipped: bool,
}

impl Tile {
    /// Rotate the slice once to the left. Affects
    fn rotate(&mut self) {
        self.rotations = (self.rotations + 1) % 4;
        self.sides.rotate_right(1);
        self.sides[0].reverse();
        self.sides[2].reverse();
    }

    /// Horizontally flip the tile
    fn flip(&mut self) {
        debug_assert!(!self.flipped);
        self.flipped = !self.flipped;
        self.sides[0].reverse();
        self.sides[2].reverse();
        self.sides.swap(1, 3);
    }

    fn len(&self) -> usize {
        self.grid.len() - 2
    }
}

fn read_input(input: &mut dyn Read) -> HashMap<u64, Tile> {
    let mut tiles = HashMap::new();
    let mut lines = Lines::new(input);

    while let Some(line) = lines.next() {
        let id = line[5..(line.len() - 1)].parse().unwrap();
        drop(line);

        let mut grid: Vec<Vec<_>> = Vec::new();

        // Clippy gets it wrong, this cannot be a for loop
        #[allow(clippy::while_let_on_iterator)]
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            grid.push(line.chars().map(|c| c == '#').collect());
        }

        let sides = [
            grid[0].clone(),
            grid.iter().map(|v| *v.last().unwrap()).collect(),
            grid.last().cloned().unwrap(),
            grid.iter().map(|v| v[0]).collect(),
        ];

        let tile = Tile {
            grid,
            sides,
            id,
            rotations: 0,
            flipped: false,
        };

        tiles.insert(id, tile);
    }

    tiles
}

fn compute_matching<'a>(tiles: impl IntoIterator<Item = &'a Tile>) -> HashMap<Vec<bool>, Vec<u64>> {
    let mut matching: HashMap<Vec<_>, Vec<u64>> = HashMap::new();

    for tile in tiles {
        for side in &tile.sides {
            let neighbours = matching.entry(side.clone()).or_default();

            if !neighbours.contains(&tile.id) {
                neighbours.push(tile.id);
            }

            let mut rev = side.clone();
            rev.reverse();

            let neighbours = matching.entry(rev).or_default();

            if !neighbours.contains(&tile.id) {
                neighbours.push(tile.id);
            }
        }
    }

    // Check if my tile assumption is violated
    debug_assert!(matching.values().map(Vec::len).max().unwrap_or(0) <= 2);

    matching
}

fn compute_neighbours<'a>(
    cliques: impl IntoIterator<Item = &'a Vec<u64>>,
) -> HashMap<u64, Vec<u64>> {
    let mut neighbours: HashMap<u64, Vec<u64>> = HashMap::new();

    for clique in cliques {
        for &a in clique {
            let related = neighbours.entry(a).or_default();

            for &b in clique {
                if a != b && !related.contains(&b) {
                    related.push(b);
                }
            }
        }
    }

    neighbours
}

fn rev_eq(a: &[bool], b: &[bool]) -> bool {
    a.iter().zip(b.iter().rev()).all(|(&a, &b)| a == b)
}

fn complete_row(
    mut first: u64,
    neighbours: &HashMap<u64, Vec<u64>>,
    tiles: &mut HashMap<u64, Tile>,
    used_tiles: &mut HashSet<u64>,
) -> Vec<u64> {
    let mut row = vec![first];
    used_tiles.insert(first);

    loop {
        let last = first;
        let mut next = None;

        for n in &neighbours[&last] {
            if used_tiles.contains(n) {
                continue;
            }

            for _ in 0..4 {
                if tiles[&last].sides[1] == tiles[n].sides[3] {
                    break;
                } else if rev_eq(&tiles[&last].sides[1], &tiles[n].sides[3]) {
                    let tile = tiles.get_mut(n).unwrap();

                    // Vertical flip == horizontal flip + 180
                    tile.rotate();
                    tile.rotate();
                    tile.flip();
                    break;
                } else {
                    tiles.get_mut(&n).unwrap().rotate();
                }
            }

            if tiles[&last].sides[1] == tiles[n].sides[3] {
                // This tile matches, add it
                next = Some(*n);
                break;
            }
        }

        if let Some(next) = next {
            row.push(next);
            used_tiles.insert(next);
            first = next;
        } else {
            return row;
        }
    }
}

fn compute_image(
    neighbours: &HashMap<u64, Vec<u64>>,
    matching: HashMap<Vec<bool>, Vec<u64>>,
    tiles: &mut HashMap<u64, Tile>,
) -> Vec<Vec<u64>> {
    let mut corner = Vec::new();

    for (&id, connected) in neighbours {
        if connected.len() == 2 && !corner.contains(&id) {
            corner.push(id);
        }
    }

    // Randomly put down the first corner
    let mut used_tiles = HashSet::new();

    let corner_tile = tiles.get_mut(&corner[0]).unwrap();

    // Rotate it until it fits
    while matching[&corner_tile.sides[2]].len() != 2 {
        corner_tile.rotate();
    }

    if matching[&corner_tile.sides[1]].len() != 2 {
        corner_tile.flip();
    }

    debug_assert_eq!(matching[&corner_tile.sides[1]].len(), 2);
    debug_assert_eq!(matching[&corner_tile.sides[2]].len(), 2);

    let rotation = corner_tile.rotations;
    let sides = corner_tile.sides.clone();

    let mut rows = vec![complete_row(corner[0], &neighbours, tiles, &mut used_tiles)];

    debug_assert_eq!(rotation, tiles[&rows[0][0]].rotations);
    debug_assert_eq!(sides, tiles[&rows[0][0]].sides);

    while used_tiles.len() < tiles.len() {
        let prev = rows.last().unwrap()[0];

        // Should be just one tile that can go there.
        let next = neighbours[&prev]
            .iter()
            .find(|&n| !used_tiles.contains(n))
            .unwrap();

        for _ in 0..4 {
            if tiles[&prev].sides[2] == tiles[next].sides[0] {
                break;
            } else if rev_eq(&tiles[&prev].sides[2], &tiles[next].sides[0]) {
                tiles.get_mut(next).unwrap().flip();
                break;
            } else {
                tiles.get_mut(next).unwrap().rotate();
            }
        }

        debug_assert_eq!(&tiles[&prev].sides[2], &tiles[next].sides[0]);

        rows.push(complete_row(*next, &neighbours, tiles, &mut used_tiles));
    }

    rows
}

fn combine_tiles(rows: &[Vec<u64>], tiles: &mut HashMap<u64, Tile>) -> Vec<Vec<u8>> {
    // Fix orientation
    for tile in tiles.values_mut() {
        let to_rotate = tile.rotations;

        for _ in 0..to_rotate {
            tile.grid = rotate(&tile.grid);
        }

        if tile.flipped {
            reverse(&mut tile.grid);
        }

        tile.rotations = 0;
        tile.flipped = false;
    }

    rows.iter()
        .flat_map(|row| {
            let len = tiles[row.first().unwrap()].len();

            let mut rows = vec![Vec::new(); len];

            for id in row {
                let tile = &tiles[id];
                for (r, line) in tile.grid.iter().skip(1).take(len).enumerate() {
                    rows[r].extend(
                        line.iter()
                            .skip(1)
                            .take(len)
                            .map(|&b| if b { b'#' } else { b'.' }),
                    );
                }
            }

            rows
        })
        .collect()
}

fn monster_regex() -> [Regex; 3] {
    [
        Regex::new(&"                  # ".replace(' ', ".")).unwrap(),
        Regex::new(&"#    ##    ##    ###".replace(' ', ".")).unwrap(),
        Regex::new(&" #  #  #  #  #  #   ".replace(' ', ".")).unwrap(),
    ]
}

fn rotate<T: Copy>(image: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut new = vec![Vec::new(); image[0].len()];

    for (c, target) in new.iter_mut().enumerate() {
        target.extend(image.iter().rev().map(|r| r[c]));
    }

    new
}

fn reverse<T>(image: &mut [Vec<T>]) {
    for row in image {
        row.reverse();
    }
}

fn replace_monster(image: &mut [Vec<u8>]) {
    let searchers = monster_regex();

    for i in 1..(image.len() - 1) {
        let mut start = 0;

        while let Some(found) = searchers[1].find_at(&image[i], start) {
            start = found.start() + 1;
            let range = found.range();

            if searchers[2].is_match(&image[i + 1][range.clone()])
                && searchers[0].is_match(&image[i - 1][range.clone()])
            {
                image[(i - 1)..=(i + 1)]
                    .iter_mut()
                    .zip(&searchers)
                    .for_each(|(line, expr)| {
                        line[range.clone()]
                            .iter_mut()
                            .zip(expr.as_str().as_bytes().iter())
                            .for_each(|(b, &r)| {
                                if *b == r {
                                    *b = b'O';
                                }
                            })
                    });
            } else if searchers[2].is_match(&image[i - 1][range.clone()])
                && searchers[0].is_match(&image[i + 1][range.clone()])
            {
                image[(i - 1)..=(i + 1)]
                    .iter_mut()
                    .rev()
                    .zip(&searchers)
                    .for_each(|(line, expr)| {
                        line[range.clone()]
                            .iter_mut()
                            .zip(expr.as_str().as_bytes().iter())
                            .for_each(|(b, &r)| {
                                if *b == r {
                                    *b = b'O';
                                }
                            })
                    });
            }
        }
    }
}

#[derive(Default)]
pub struct Day20;

impl Solution for Day20 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let tiles = read_input(input);
        let matching = compute_matching(tiles.values());
        let neighbours = compute_neighbours(matching.values());

        neighbours
            .into_iter()
            .filter_map(|(i, n)| if n.len() == 2 { Some(i) } else { None })
            .product::<u64>()
            .to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let mut tiles = read_input(input);
        let matching = compute_matching(tiles.values());
        let neighbours = compute_neighbours(matching.values());

        let rows = compute_image(&neighbours, matching, &mut tiles);

        let mut image = combine_tiles(&rows, &mut tiles);

        replace_monster(&mut image);
        reverse(&mut image);
        replace_monster(&mut image);
        image = rotate(&image);
        replace_monster(&mut image);
        reverse(&mut image);
        replace_monster(&mut image);

        image
            .iter()
            .map(|b| bytecount::count(&b, b'#'))
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../samples/20.txt");

    #[test]
    fn sample_part1() {
        test_implementation!(Day20, 1, SAMPLE, 20899048083289u64);
    }

    #[test]
    fn sample_part2() {
        test_implementation!(Day20, 2, SAMPLE, 273);
    }

    #[test]
    fn test_tile_rotate() {
        let mut tile = Tile {
            grid: Vec::new(),
            id: 1,
            sides: [
                vec![false, true],
                vec![false, false, true],
                vec![false, true],
                vec![true, true, false],
            ],
            rotations: 0,
            flipped: false,
        };

        tile.rotate();

        assert_eq!(
            tile.sides,
            [
                vec![false, true, true],
                vec![false, true],
                vec![true, false, false],
                vec![false, true],
            ]
        );
    }

    #[test]
    fn test_tile_flip() {
        let mut tile = Tile {
            grid: Vec::new(),
            id: 1,
            sides: [
                vec![false, true],
                vec![false, false, true],
                vec![false, true],
                vec![true, true, false],
            ],
            rotations: 0,
            flipped: false,
        };

        tile.flip();

        assert_eq!(
            tile.sides,
            [
                vec![true, false],
                vec![true, true, false],
                vec![true, false],
                vec![false, false, true],
            ]
        );
    }

    #[test]
    fn test_rotate() {
        let sample: Vec<Vec<u8>> = vec![
            b"###".as_ref().to_owned(),
            b"  #".as_ref().to_owned(),
            b"# #".as_ref().to_owned(),
        ];

        let rotated = rotate(&sample);

        let correct: Vec<Vec<u8>> = vec![
            b"# #".as_ref().to_owned(),
            b"  #".as_ref().to_owned(),
            b"###".as_ref().to_owned(),
        ];

        assert_eq!(correct, rotated);
    }
}
