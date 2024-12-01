use std::fs;

pub mod aoc_1;

/// Reads the file given the **path**
/// Splits the string by `\n` and returns
/// a vec containing all the string
fn read_input_file(pwd: &str) -> Vec<String> {
    let input = fs::read_to_string(pwd).expect("Unable to read file");

    let mut inputs: Vec<String> = input.split('\n').map(|line| line.to_string()).collect();

    inputs.remove(inputs.len() - 1);

    inputs
}
