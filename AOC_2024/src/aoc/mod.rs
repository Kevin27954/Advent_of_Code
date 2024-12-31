use std::fs;

pub mod aoc_1;
pub mod aoc_2;
pub mod aoc_3;
pub mod aoc_4;
pub mod aoc_5;
pub mod aoc_6;
pub mod aoc_7;

static PATH: &'static str = "./src/aoc_input";

pub struct AOC {}

/// Reads the file given the **path**
/// Splits the string by `\n` and returns
/// a vec containing all the string
fn read_input_file(pwd: String) -> Vec<String> {
    let input = fs::read_to_string(pwd).expect("Unable to read file");

    let mut inputs: Vec<String> = input.split('\n').map(|line| line.to_string()).collect();

    inputs.remove(inputs.len() - 1);

    inputs
}
