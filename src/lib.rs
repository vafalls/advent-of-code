pub mod common;
pub mod utils;

use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Lines};

pub fn read_lines_from_file(filename: &str) -> Lines<BufReader<File>> {
    let file = File::open(filename).expect("Could not read file");
    BufReader::new(file).lines()
}

pub fn read_file_into_arrays(filename: &str) -> Vec<Vec<char>> {
    read_to_string(filename)
        .expect("Could not read file")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn read_file_into_strings(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .expect("Could not read file")
        .lines()
        .map(String::from)
        .collect()
}
