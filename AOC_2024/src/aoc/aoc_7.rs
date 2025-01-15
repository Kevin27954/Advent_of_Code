/*

Problem A:

Each line represents a single equation. The test value appears before the colon on each line; it is your job
to determine whether the remaining numbers can be combined with operators to produce the test value.

Operators allowed: +, *

e.g.
190: 10 19

The engineers just need the total calibration result, which is the sum of the test values from just the equations
that could possibly be true.

Problem B:

The concatenation operator (||) combines the digits from its left and right inputs into a single number. For example,
12 || 345 would become 12345. All operators are still evaluated left-to-right.

Operators allowed: +, *, ||

What is their total calibration result?

*/

use super::AOC;
use super::{read_input_file, PATH};

static FILE: &'static str = "aoc_7.txt";

enum Operations {
    Multiply,
    Addition,
    Concat,
}

#[allow(dead_code)]
impl AOC {
    pub fn aoc_7_a() -> usize {
        let data = read_input_file(format!("{}/{}", PATH, FILE));
        let values = Self::transform_data_7(data);

        let mut total = 0;
        for value in values {
            if Self::aoc_7_a_recursion(value.0, &value.1, 1, value.1[0]) {
                total += value.0;
            }
        }

        total
    }

    pub fn aoc_7_b() -> usize {
        let data = read_input_file(format!("{}/{}", PATH, FILE));
        let values = Self::transform_data_7(data);

        let mut total = 0;
        for value in values {
            if Self::aoc_7_b_recursion(value.0, &value.1, 1, value.1[0]) {
                total += value.0;
            }
        }

        total
    }

    /// The **curr_value** should be the first value in the values list.
    /// e.g. if the values list is [12, 34, 11, 1];
    /// Then **curr_value** should be *12* initally.
    ///
    /// This also means that **idx** should be starting at *1* instead of *0*.
    fn aoc_7_b_recursion(
        target: usize,
        values: &Vec<usize>,
        idx: usize,
        curr_value: usize,
    ) -> bool {
        if idx == values.len() {
            if curr_value == target {
                return true;
            } else {
                return false;
            }
        }

        let operations = vec![
            Operations::Multiply,
            Operations::Addition,
            Operations::Concat,
        ];

        for operation in operations {
            match operation {
                Operations::Multiply => {
                    let new_curr = curr_value * values[idx];
                    if Self::aoc_7_b_recursion(target, values, idx + 1, new_curr) {
                        return true;
                    }
                }
                Operations::Addition => {
                    let new_curr = curr_value + values[idx];
                    if Self::aoc_7_b_recursion(target, values, idx + 1, new_curr) {
                        return true;
                    }
                }
                Operations::Concat => {
                    // The goal here is to check if the curr_value has the same numbers as the
                    // front most numbers. e.g. curr_value = 1; we want to see if value has 1 in
                    // the largest digit.

                    // Nvm I thought too far, it was much simpler than that I initally was
                    // thinking.

                    let mut num_digits = 1;
                    let mut temp = values[idx];
                    while temp != 0 {
                        temp = temp / 10;
                        num_digits *= 10;
                    }

                    let new_curr = (curr_value * num_digits) + values[idx];

                    // This gets changed into it's own sub problem again, starting fresh I meant.
                    // New values and stuff
                    if Self::aoc_7_b_recursion(target, values, idx + 1, new_curr) {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// The **curr_value** should be the first value in the values list.
    /// e.g. if the values list is [12, 34, 11, 1];
    /// Then **curr_value** should be *12* initally.
    ///
    /// This also means that **idx** should be starting at *1* instead of *0*.
    fn aoc_7_a_recursion(
        target: usize,
        values: &Vec<usize>,
        idx: usize,
        curr_value: usize,
    ) -> bool {
        if idx == values.len() {
            if curr_value == target {
                return true;
            } else {
                return false;
            }
        }

        let operations = vec![Operations::Multiply, Operations::Addition];

        for operation in operations {
            match operation {
                Operations::Multiply => {
                    let new_curr = curr_value * values[idx];
                    if Self::aoc_7_a_recursion(target, values, idx + 1, new_curr) {
                        return true;
                    }
                }
                Operations::Addition => {
                    let new_curr = curr_value + values[idx];
                    if Self::aoc_7_a_recursion(target, values, idx + 1, new_curr) {
                        return true;
                    }
                }
                _ => {}
            }
        }

        false
    }

    fn transform_data_7(data: Vec<String>) -> Vec<(usize, Vec<usize>)> {
        // I think I want it as a vec of tuple of numb and vec

        data.iter()
            .map(|line| {
                let temp: Vec<&str> = line.split(':').collect();

                let res = temp[0].parse::<usize>().unwrap();
                let mut values = temp[1].split(' ').into_iter();

                values.next();
                let temp: Vec<usize> = values
                    .map(|values| values.parse::<usize>().unwrap())
                    .collect();

                (res, temp)
            })
            .collect::<Vec<(usize, Vec<usize>)>>()
    }
}
