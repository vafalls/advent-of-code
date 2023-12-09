use std::fs::File;
use std::io::{BufRead, BufReader, Lines};


pub fn read_lines_from_file(filename: &str) -> Lines<BufReader<File>> {
    let file = File::open(filename).expect("Could not read file");
    BufReader::new(file).lines()
}
