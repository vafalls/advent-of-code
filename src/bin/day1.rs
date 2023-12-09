use std::iter::Iterator;
use advent_of_code::read_lines_from_file;


fn main() {
    let lines = read_lines_from_file("./src/input/day1/input_day_1.txt");
    let mut result_sum: u64 = 0;
    for line in lines {
        let asdf = line.unwrap();

        let first = get_number_char(asdf.chars());
        let last = get_number_char(asdf.chars().rev());

        let mut tmp = String::new();
        tmp.push(first);
        tmp.push(last);

        result_sum += tmp.parse::<u64>().unwrap();
    }
    println!("The result is: {}", result_sum)
}

fn get_number_char<I: Iterator<Item=char>>(chars: I) -> char {
    for c in chars {
        if c.is_digit(10) {
            return c;
        }
    }
    panic!("foobar")
}
