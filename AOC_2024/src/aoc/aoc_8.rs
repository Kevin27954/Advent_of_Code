/*

Problem A:

Scanning across the city, you find that there are actually many such antennas. Each antenna is tuned to a
specific frequency indicated by a single lowercase letter, uppercase letter, or digit. You create a map
(your puzzle input) of these antennas.

The signal only applies its nefarious effect at specific antinodes based on the resonant frequencies of the
antennas. In particular, an antinode occurs at any point that is perfectly in line with two antennas of the
same frequency - but only when one of the antennas is twice as far away as the other.

e.x.

..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
..........

Antennas with different frequencies don't create antinodes; A and a count as different frequencies. However,
antinodes can occur at locations that contain antennas.

The first example has antennas with two different frequencies, so the antinodes they create look like this,
plus an antinode overlapping the topmost A-frequency antenna:

......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.

Because the topmost A-frequency antenna overlaps with a 0-frequency antinode, there are 14 total unique locations
that contain an antinode within the bounds of the map.



Problem B:

*/

use std::collections::HashMap;

use super::AOC;
use super::{read_input_file, PATH};

static FILE: &'static str = "aoc_8.txt";

impl AOC {
    pub fn aoc_8_a() -> i32 {
        let data = read_input_file(format!("{}/{}", PATH, FILE));
        let (mut map, antenna) = Self::transform_data_8(data);

        let mut unique = 0;

        for (_, points) in antenna {
            for i in 0..points.len() {
                for j in 0..points.len() {
                    if i == j {
                        continue;
                    }
                    let (x1, x2) = (points[i].1 as i32, points[j].1 as i32);
                    let (y1, y2) = (points[i].0 as i32, points[j].0 as i32);

                    if Self::is_valid_antinode(&mut map, x1, y1, x2, y2) {
                        unique += 1;
                    }
                }
            }
        }

        unique
    }

    fn is_valid_antinode(map: &mut Vec<Vec<char>>, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        let diff_x = (x1 - x2).abs();
        let diff_y = (y1 - y2).abs();

        let x_op = if x1 < x2 { '-' } else { '+' };
        let y_op = if y1 < y2 { '-' } else { '+' };

        let x;
        let y;

        match x_op {
            '+' => x = x1 + diff_x,
            '-' => x = x1 - diff_x,
            _ => {
                unreachable!()
            }
        }

        match y_op {
            '+' => y = y1 + diff_y,
            '-' => y = y1 - diff_y,
            _ => {
                unreachable!()
            }
        }

        if x < 0 || x >= map.len() as i32 {
            return false;
        }

        if y < 0 || y >= map[0].len() as i32 {
            return false;
        }

        let node = map[y as usize][x as usize];

        if node == '#' {
            return false;
        }

        // Ensure uniques only
        map[y as usize][x as usize] = '#';
        true
    }

    fn transform_data_8(data: Vec<String>) -> (Vec<Vec<char>>, HashMap<char, Vec<(usize, usize)>>) {
        let map: Vec<Vec<char>> = data.iter().map(|string| string.chars().collect()).collect();

        let mut points = HashMap::new();

        let row_len = data.len();
        let col_len = map[0].len();
        for row in 0..row_len {
            for col in 0..col_len {
                if map[row][col] != '.' {
                    let points_arr = points.entry(map[row][col]).or_insert(Vec::new());

                    points_arr.push((row, col));
                }
            }
        }

        (map, points)
    }
}
