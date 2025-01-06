/*



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
                    let (x1, x2) = (points[i].0 as i32, points[j].0 as i32);
                    let (y1, y2) = (points[i].1 as i32, points[j].1 as i32);

                    // no need for slope, we instead just find the difference between the x and y
                    // and just add that differnece or ubstract it to the points that we have.
                    let diff_x = (x1 - x2).abs();
                    let diff_y = (y1 - y2).abs();

                    if points[i].0 < points[j].0 {
                        // minus i
                        // add j
                        if Self::is_valid_antinode(&mut map, x1 - diff_x, y1 - diff_y) {
                            unique += 1;
                        }
                        if Self::is_valid_antinode(&mut map, x2 + diff_x, y2 + diff_y) {
                            unique += 1;
                        }
                    } else if points[i].0 > points[j].0 {
                        // add i
                        // minus j
                        if Self::is_valid_antinode(&mut map, x1 + diff_x, y1 + diff_y) {
                            unique += 1;
                        }
                        if Self::is_valid_antinode(&mut map, x2 - diff_x, y2 - diff_y) {
                            unique += 1;
                        }
                    } else {
                        if points[i].1 < points[j].1 {
                            // minus i.y
                            // add j.y
                            if Self::is_valid_antinode(&mut map, x1, y1 - diff_y) {
                                unique += 1;
                            }
                            if Self::is_valid_antinode(&mut map, x2, y2 + diff_y) {
                                unique += 1;
                            }
                        } else if points[i].1 > points[j].1 {
                            // add i.y
                            // minus j.y
                            if Self::is_valid_antinode(&mut map, x1, y1 + diff_y) {
                                unique += 1;
                            }
                            if Self::is_valid_antinode(&mut map, x2, y2 - diff_y) {
                                unique += 1;
                            }
                        }
                    }
                }
            }
        }

        unique
    }

    fn is_valid_antinode(map: &mut Vec<Vec<char>>, x: i32, y: i32) -> bool {
        if x < 0 || x >= map.len() as i32 {
            return false;
        }

        if y < 0 || y >= map[0].len() as i32 {
            return false;
        }

        let node = map[x as usize][y as usize];
        if node != '.' {
            return false;
        }

        // Ensure uniques only
        map[x as usize][y as usize] = '#';
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
