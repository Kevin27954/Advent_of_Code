/*

Problem:

Pair up the smallest number in the left list with the smallest number in the right list, then
the second-smallest left number with the second-smallest right number, and so on.

Within each pair, figure out how far apart the two numbers are; you'll need to add up all of those distances.
For example, if you pair up a 3 from the left list with a 7 from the right list, the distance apart is 4;
if you pair up a 9 with a 3, the distance apart is 6.

*/

use std::collections::HashMap;

use super::AOC;
use super::{read_input_file, PATH};

static FILE: &'static str = "aoc_1.txt";

impl AOC {
    pub fn aoc_1_a() -> i32 {
        let inputs = read_input_file(format!("{PATH}/{FILE}"));
        let (mut col1, mut col2) = Self::transform_data_1(inputs);

        // Maybe consider implement a quick sort real quick
        // So I can re-learn some sorting algorithms
        col1.sort();
        col2.sort();

        let mut sum: i32 = 0;

        for x in 0..col1.len() {
            let diff = col1[x] - col2[x];
            sum += diff.abs();
        }

        sum
    }

    // ran

    pub fn aoc_1_b() -> i32 {
        let inputs = read_input_file(format!("{PATH}/{FILE}"));
        let (col1, col2) = Self::transform_data_1(inputs);

        let mut hm = HashMap::<i32, i32>::with_capacity(1000);

        for i in 0..col2.len() {
            *hm.entry(col2[i]).or_insert(0) += 1;
        }

        let mut sum: i32 = 0;

        for i in 0..col1.len() {
            match hm.get(&col1[i]) {
                Some(num) => {
                    sum += num * col1[i];
                }
                None => {}
            }
        }

        sum
    }

    /// Turns Vec<String> into Vec<Vec<u32>> where there are only two indices
    /// 0 = column 1
    /// 1 = column 2
    fn transform_data_1(input: Vec<String>) -> (Vec<i32>, Vec<i32>) {
        let mut column_1 = Vec::new();
        let mut column_2 = Vec::new();

        let mut input_iter = input.iter().map(|line| {
            let temp: Vec<&str> = line.split("   ").collect();
            column_1.push(temp[0].parse().unwrap());
            column_2.push(temp[1].parse().unwrap());
        });

        while let Some(_) = input_iter.next() {}
        (column_1, column_2)
    }
}
