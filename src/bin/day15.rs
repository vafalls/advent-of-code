use advent_of_code::read_file_into_strings;

fn run_hash_algorithm(step: &str) -> u32 {
    let mut res = 0;
    for i in step.chars() {
        res += i as u32;
        res = (res * 17) % 256;
    }
    res
}

fn main() {
    let line = read_file_into_strings("./src/input/day15/input.txt")[0].clone();
    let res = line.split(',').map(run_hash_algorithm).sum::<u32>();
    println!("{:?}", res);
}