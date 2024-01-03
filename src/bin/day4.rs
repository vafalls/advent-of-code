use advent_of_code::{read_lines_from_file};


fn parse_game(line: String) -> u32 {
    let splits: Vec<_> = line.split(":").collect();
    let foo = splits.last().unwrap();
    let numbers_str = foo.replace(" | ", " ").replace("  ", " ");
    let mut numbers: Vec<_> = numbers_str.trim().split(" ").collect();
    let length_before = numbers.len();
    numbers.sort();
    numbers.dedup();

    let mut tmp = 0;
    for i in 0..(length_before as u32 - numbers.len() as u32) {
        if i == 0 {
            tmp = 1;
        } else {
            tmp = tmp*2;
        }
    }
    return tmp
}


fn main() {
    let mut res: u32 = 0;
    for line in read_lines_from_file("./src/input/day4/input.txt") {
        let one_line = line.expect("nope");
        res += parse_game(one_line);
    }
    println!("{}", res)
}
