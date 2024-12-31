/*

Problem A:

The map shows the current position of the guard with ^, <, >, v (to indicate the guard is currently facing up from the
perspective of the map). Any obstructions - crates, desks, alchemical reactors, etc. - are shown as #.

- If there is something directly in front of you, turn right 90 degrees.
- Otherwise, take a step forward.

Predict the path of the guard. How many distinct positions will the guard visit before leaving the mapped area?

Problem B:

They'd like to place a single new obstruction in such a way that the guard will get stuck in a loop, making the rest of
the lab safe to search.

How many different positions could you choose for this obstruction?

*/

use std::cmp::max;

use super::AOC;
use super::{read_input_file, PATH};

static FILE: &'static str = "aoc_6.txt";

#[allow(dead_code)]
impl AOC {
    pub fn aoc_6_a() -> i32 {
        let data = read_input_file(format!("{}/{}", PATH, FILE));
        let map = Self::transform_data_6(data);

        let mut cp_map = Vec::new();

        for _ in 0..map.len() {
            let mut inner_vec = Vec::new();
            for _ in 0..map[0].len() {
                inner_vec.push(' ');
            }
            cp_map.push(inner_vec);
        }

        let mut unique = 0;

        let (mut row, mut col) = Self::find_start_pos(&map);

        let mut degree; // 0 -> right, 90 -> up, 180 -> left, 270 -> down
        match &map[row][col] {
            '>' => degree = 0,
            '^' => degree = 90,
            '<' => degree = 180,
            'v' => degree = 270,
            _ => unreachable!(),
        };

        let move_pos: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

        let mut max_visited = 0;

        loop {
            let (move_x, move_y) = move_pos[degree / 90];

            // Move then add to count?

            if cp_map[row][col] == ' ' {
                unique += 1;
                cp_map[row][col] = '#';
            }

            let new_row = row as i32 + move_x;
            let new_col = col as i32 + move_y;

            if new_row >= map.len() as i32
                || new_row < 0
                || new_col >= map[0].len() as i32
                || new_col < 0
            {
                break;
            }

            if map[new_row as usize][new_col as usize] == '#' {
                degree = (degree + 270) % 360;
                let ptr_cp = &mut cp_map[new_row as usize][new_col as usize];

                if *ptr_cp == ' ' {
                    *ptr_cp = '1';
                } else {
                    *ptr_cp = char::from_u32(*ptr_cp as u32 + 1).unwrap();
                }

                max_visited = max(max_visited, *ptr_cp as u32);

                continue;
            }

            row = new_row as usize;
            col = new_col as usize;
        }

        unique
    }

    pub fn aoc_6_b() -> i32 {
        // Brute Force idea: put one on every single one of the dude's path, every combination,
        // e.g.
        // For every index (except the starting point) he traverses, put a temporaray obstructions on
        // it and see if that will result in a infinite loop.

        // Conditions for an infinite loop: if I touch an obstruction more than 2

        // Loop through inital run, get info.

        let paths = Self::helper_6b();

        let data = read_input_file(format!("{}/{}", PATH, FILE));
        let map = Self::transform_data_6(data);

        let mut unique = 0;

        for path in paths {
            if Self::is_infinite(&map, path) {
                unique += 1;
            }
        }

        unique
    }

    fn is_infinite(map: &Vec<Vec<char>>, new_obstacle: (usize, usize)) -> bool {
        let (mut row, mut col) = Self::find_start_pos(&map);
        let mut degree; // 0 -> right, 90 -> up, 180 -> left, 270 -> down
        match &map[row][col] {
            '>' => degree = 0,
            '^' => degree = 90,
            '<' => degree = 180,
            'v' => degree = 270,
            _ => unreachable!(),
        };

        let mut cp_map = Vec::new();

        for _ in 0..map.len() {
            let mut inner_vec = Vec::new();
            for _ in 0..map[0].len() {
                inner_vec.push(' ');
            }
            cp_map.push(inner_vec);
        }

        let move_pos: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

        loop {
            let (move_x, move_y) = move_pos[degree / 90];

            // The key idea is to check if we have touched something more than two times. if so then it
            // is infinite.

            let new_row = row as i32 + move_x;
            let new_col = col as i32 + move_y;

            // If it is not a infinite loop.
            if new_row >= map.len() as i32
                || new_row < 0
                || new_col >= map[0].len() as i32
                || new_col < 0
            {
                return false;
            }

            let obstacle_row = new_obstacle.0;
            let obstacle_col = new_obstacle.1;

            if map[new_row as usize][new_col as usize] == '#'
                || (new_row as usize == obstacle_row && new_col as usize == obstacle_col)
            {
                degree = (degree + 270) % 360;

                // Update the counter for the obstacle on the map.

                let counter = &mut cp_map[new_row as usize][new_col as usize];

                if *counter == ' ' {
                    *counter = '1';
                } else {
                    let ascii_num = *counter as u32 + 1;
                    if ascii_num > 51 {
                        return true;
                    }

                    *counter = char::from_u32(ascii_num).unwrap();
                }

                continue;
            }

            row = new_row as usize;
            col = new_col as usize;
        }
    }

    fn helper_6b() -> Vec<(usize, usize)> {
        let data = read_input_file(format!("{}/{}", PATH, FILE));
        let map = Self::transform_data_6(data);

        let mut cp_map = Vec::new();

        for _ in 0..map.len() {
            let mut inner_vec = Vec::new();
            for _ in 0..map[0].len() {
                inner_vec.push(' ');
            }
            cp_map.push(inner_vec);
        }

        let (mut row, mut col) = Self::find_start_pos(&map);

        let mut degree; // 0 -> right, 90 -> up, 180 -> left, 270 -> down
        match &map[row][col] {
            '>' => degree = 0,
            '^' => degree = 90,
            '<' => degree = 180,
            'v' => degree = 270,
            _ => unreachable!(),
        };

        let move_pos: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

        loop {
            let (move_x, move_y) = move_pos[degree / 90];

            // Move then add to count?

            if cp_map[row][col] == ' ' {
                cp_map[row][col] = '#';
            }

            let new_row = row as i32 + move_x;
            let new_col = col as i32 + move_y;

            if new_row >= map.len() as i32
                || new_row < 0
                || new_col >= map[0].len() as i32
                || new_col < 0
            {
                break;
            }

            if map[new_row as usize][new_col as usize] == '#' {
                degree = (degree + 270) % 360;
                continue;
            }

            row = new_row as usize;
            col = new_col as usize;
        }

        let mut paths = Vec::new();

        for row in 0..cp_map.len() {
            for col in 0..cp_map[0].len() {
                if cp_map[row][col] == '#' {
                    paths.push((row, col));
                }
            }
        }

        paths
    }

    fn find_start_pos(map: &Vec<Vec<char>>) -> (usize, usize) {
        for row in 0..map.len() {
            for col in 0..map[0].len() {
                match map[row][col] {
                    'v' | '<' | '>' | '^' => return (row, col),
                    _ => {}
                }
            }
        }
        unreachable!()
    }

    fn transform_data_6(data: Vec<String>) -> Vec<Vec<char>> {
        let iter = data.into_iter();

        iter.map(|line| line.chars().collect()).collect()
    }
}
