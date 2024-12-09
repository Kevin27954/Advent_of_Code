/*

=== Problem:

This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping
other words. It's a little unusual, though, as you don't merely need to find one instance of XMAS - you need
to find all of them.


*/

use super::AOC;
use super::{read_input_file, PATH};

static FILE: &'static str = "aoc_4.txt";

impl AOC {
    pub fn aoc_4_a() -> usize {
        let data = read_input_file(format!("{PATH}/{FILE}"));
        let letters = Self::transform_data_4(data);

        let mut num_xmas = 0;

        for row in 0..letters.len() {
            for col in 0..letters[row].len() {
                match letters[row][col] {
                    'X' => {
                        num_xmas += Self::find_xmas(row as i32, col as i32, &letters);
                    }
                    _ => {}
                }
            }
        }

        num_xmas
    }

    pub fn aoc_4_b() -> usize {
        let data = read_input_file(format!("{PATH}/{FILE}"));
        let letters = Self::transform_data_4(data);

        let mut num_xmas = 0;

        for row in 0..letters.len() {
            for col in 0..letters[row].len() {
                match letters[row][col] {
                    'A' => {
                        num_xmas += Self::find_x_mas(row as i32, col as i32, &letters);
                    }
                    _ => {}
                }
            }
        }

        num_xmas
    }

    fn find_x_mas(row: i32, col: i32, letters: &Vec<Vec<char>>) -> usize {
        let mut num_x_mas = 0;

        let corners: [(i32, i32); 4] = [(-1, -1), (1, -1), (-1, 1), (1, 1)];

        let max_row = letters.len() as i32;
        let max_col = letters[0].len() as i32;

        let mut is_m;
        let mut is_s;

        for i in 0..2 {
            is_m = false;
            is_s = false;
            let (move_x, move_y) = corners[i];
            let new_col = col + move_x;
            let new_row = row + move_y;
            if new_col < 0 || new_col >= max_col || new_row < 0 || new_row >= max_row {
                return 0;
            }

            if letters[new_row as usize][new_col as usize] == 'M' {
                is_m = true;
            } else if letters[new_row as usize][new_col as usize] == 'S' {
                is_s = true;
            }

            let (move_x, move_y) = corners[corners.len() - 1 - i];
            let new_col = col + move_x;
            let new_row = row + move_y;
            if new_col < 0 || new_col >= max_col || new_row < 0 || new_row >= max_row {
                return 0;
            }

            if is_m && letters[new_row as usize][new_col as usize] == 'S' {
                num_x_mas = 1;
            } else if is_s && letters[new_row as usize][new_col as usize] == 'M' {
                num_x_mas = 1;
            } else {
                num_x_mas = 0;
                break;
            }
        }

        num_x_mas
    }

    fn find_xmas(row: i32, col: i32, letters: &Vec<Vec<char>>) -> usize {
        //(x, y)
        let move_dir: [(i32, i32); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        let find_letters = ['M', 'A', 'S'];
        let mut curr;
        let max_row = letters.len() as i32;
        let max_col = letters[0].len() as i32;

        let mut num_xmas = 0;

        for (move_x, move_y) in move_dir {
            let mut new_col = col + move_x;
            let mut new_row = row + move_y;
            curr = 0;

            loop {
                if curr >= find_letters.len() {
                    num_xmas += 1;
                    break;
                }

                if new_col < 0 || new_col >= max_col || new_row < 0 || new_row >= max_row {
                    break;
                }

                if letters[new_row as usize][new_col as usize] == find_letters[curr] {
                    curr += 1;
                } else {
                    break;
                }

                new_col = move_x + new_col;
                new_row = move_y + new_row;
            }
        }

        num_xmas
    }

    fn transform_data_4(data: Vec<String>) -> Vec<Vec<char>> {
        data.iter().map(|line| line.chars().collect()).collect()
    }
}
