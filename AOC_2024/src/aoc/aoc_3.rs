/*

=== Problem:

However, because the program's memory has been corrupted, there are also many invalid characters that should be ignored,
even if they look like part of a mul instruction. Sequences like mul(4*, mul(6,9!, ?(12,34), or mul ( 2 , 4 ) do nothing.

For example, consider the following section of corrupted memory:

xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
Only the four highlighted sections are real mul instructions. Adding up the result of each instruction produces 161
(2*4 + 5*5 + 11*8 + 8*5).

Scan the corrupted memory for uncorrupted mul instructions. What do you get if you add up all of the results of the
multiplications?

=== Problem 2:

There are two new instructions you'll need to handle:

The do() instruction enables future mul instructions.
The don't() instruction disables future mul instructions.
Only the most recent do() or don't() instruction applies. At the beginning of the program, mul instructions are enabled.



*/

use super::AOC;
use super::PATH;

use regex::Regex;
use std::fs;

static FILE: &'static str = "aoc_3.txt";

static REGEX_A: &'static str = r"mul\(\d*,\d*\)";
static REGEX_B: &'static str = r"do\(\)|mul\(\d*,\d*\)|don't\(\)";

#[allow(dead_code)]
impl AOC {
    pub fn aoc_3_a() -> i32 {
        let data = fs::read_to_string(format!("{PATH}/{FILE}")).unwrap();
        let rx = Regex::new(REGEX_A).unwrap();

        let mut total = 0;

        let muls: Vec<(i32, i32)> = rx
            .find_iter(&data)
            .map(|m| {
                let res = m.as_str();
                let extract = res.strip_prefix("mul(").unwrap().strip_suffix(")").unwrap();

                let nums: Vec<&str> = extract.split(',').collect();

                (nums[0].parse().unwrap(), nums[1].parse().unwrap())
            })
            .collect();

        for (num_a, num_b) in muls {
            total += num_a * num_b;
        }

        total
    }

    pub fn aoc_3_b() -> i32 {
        let data = fs::read_to_string(format!("{PATH}/{FILE}")).unwrap();
        let rx = Regex::new(REGEX_B).unwrap();

        let mut total = 0;

        let mut enabled = true;

        let muls: Vec<(i32, i32)> = rx
            .find_iter(&data)
            .map(|m| {
                let res = m.as_str();

                match res {
                    "do()" => {
                        enabled = true;
                        (0, 0)
                    }
                    "don't()" => {
                        enabled = false;
                        (0, 0)
                    }
                    _ => {
                        if enabled {
                            let extract =
                                res.strip_prefix("mul(").unwrap().strip_suffix(")").unwrap();
                            let nums: Vec<&str> = extract.split(',').collect();
                            (nums[0].parse().unwrap(), nums[1].parse().unwrap())
                        } else {
                            (0, 0)
                        }
                    }
                }
            })
            .collect();

        for (num_a, num_b) in muls {
            total += num_a * num_b;
        }

        total
    }
}
