/*

=== Problem:

The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems
can only tolerate levels that are either gradually increasing or gradually decreasing. So, a report
only counts as safe if both of the following are true:

- The levels are either all increasing or all decreasing.
- Any two adjacent levels differ by at least one and at most three.

=== Problem 2:

Now, the same rules apply as before, except if removing a single level from an unsafe report would make it safe,
the report instead counts as safe.

*/

use super::AOC;
use super::{read_input_file, PATH};

static FILE: &'static str = "aoc_2.txt";

// Could have just had numbesr too. That would have also worked.
#[derive(Debug, PartialEq)]
enum IncOrDec {
    Inc,
    Dec,
}

#[allow(dead_code)]
impl AOC {
    pub fn aoc_2_a() -> i32 {
        let inputs = read_input_file(format!("{PATH}/{FILE}"));
        let data = Self::transform_data_2(inputs);

        let mut total_safe = 0;

        for i in 0..data.len() {
            let mut inc_or_dec: Option<IncOrDec> = None;

            let mut rules_broken = false;

            // Loop each each report ( O(1) )
            for r in 1..data[i].len() {
                let prev = data[i][r - 1];
                let curr = data[i][r];

                // Init stage
                let diff = curr - prev;

                if let None = inc_or_dec {
                    inc_or_dec = if diff.is_positive() {
                        Some(IncOrDec::Inc)
                    } else {
                        Some(IncOrDec::Dec)
                    }
                }

                // Actual test stage

                let curr_direction = if diff.is_positive() {
                    IncOrDec::Inc
                } else {
                    IncOrDec::Dec
                };

                if !curr_direction.eq(inc_or_dec.as_ref().unwrap())
                    || diff.abs() > 3
                    || diff.abs() == 0
                {
                    rules_broken = true;
                    break;
                }
            }

            if !rules_broken {
                total_safe += 1;
            }
        }

        total_safe
    }

    pub fn aoc_2_b() -> i32 {
        let inputs = read_input_file(format!("{PATH}/{FILE}"));
        let data = Self::transform_data_2(inputs);

        let mut total_safe = 0;

        /* Potential O(N) solution idea:
         * IDK for now.
         */

        // It is a O(N^2) solution.
        for i in 0..data.len() {
            // Go through every combination where 1 is removed.
            for r in 0..data[i].len() {
                // Each iteration is considered as a **new** iteration.
                let mut inc_or_dec: Option<IncOrDec> = None;
                let mut prev = i32::MAX;
                let mut is_valid = true;

                for r2 in 0..data[i].len() {
                    if r == r2 {
                        continue;
                    }
                    if prev == i32::MAX {
                        prev = data[i][r2];
                        continue;
                    }

                    let diff = data[i][r2] - prev;

                    if let None = inc_or_dec {
                        inc_or_dec = if diff.is_positive() {
                            Some(IncOrDec::Inc)
                        } else {
                            Some(IncOrDec::Dec)
                        }
                    }

                    let curr_direction = if diff.is_positive() {
                        IncOrDec::Inc
                    } else {
                        IncOrDec::Dec
                    };

                    if !curr_direction.eq(inc_or_dec.as_ref().unwrap())
                        || diff.abs() > 3
                        || diff.abs() == 0
                    {
                        is_valid = false;
                        break;
                    }

                    prev = data[i][r2];
                }

                if is_valid {
                    total_safe += 1;
                    break;
                }
            }
        }

        total_safe
    }

    fn transform_data_2(inputs: Vec<String>) -> Vec<Vec<i32>> {
        let mut data = Vec::new();

        inputs.iter().for_each(|line| {
            let report: Vec<i32> = line
                .split(' ')
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            data.push(report);
        });

        data
    }
}
