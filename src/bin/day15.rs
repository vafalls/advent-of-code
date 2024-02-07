use advent_of_code::common::day15::run_hash_algorithm;
use advent_of_code::read_file_into_strings;

fn main() {
    let line = read_file_into_strings("./src/input/day15/input.txt")[0].clone();
    let res = line.split(',').map(run_hash_algorithm).sum::<u32>();
    println!("{:?}", res);
}