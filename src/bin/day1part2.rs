use advent_of_code::read_lines_from_file;
use std::iter::Iterator;
use std::str::FromStr;
use itertools::enumerate;

enum LetterNumber {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9
}

impl FromStr for LetterNumber {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "one" => Ok(LetterNumber::One),
            "eno" => Ok(LetterNumber::One),
            "two" => Ok(LetterNumber::Two),
            "owt" => Ok(LetterNumber::Two),
            "three" => Ok(LetterNumber::Three),
            "eerht" => Ok(LetterNumber::Three),
            "four" => Ok(LetterNumber::Four),
            "ruof" => Ok(LetterNumber::Four),
            "five" => Ok(LetterNumber::Five),
            "evif" => Ok(LetterNumber::Five),
            "six" => Ok(LetterNumber::Six),
            "xis" => Ok(LetterNumber::Six),
            "seven" => Ok(LetterNumber::Seven),
            "neves" => Ok(LetterNumber::Seven),
            "eight" => Ok(LetterNumber::Eight),
            "thgie" => Ok(LetterNumber::Eight),
            "nine" => Ok(LetterNumber::Nine),
            "enin" => Ok(LetterNumber::Nine),
            _ => Err(()),
        }
    }
}


fn main() {
    let lines = read_lines_from_file("./src/input/day1/input.txt");
    let mut result_sum: u64 = 0;


    for line in lines {
        let line = line.unwrap();

        let a = get_number(line.clone(), false);
        let first = a.to_string();
        let last = get_number(line.clone(), true).to_string();

        let tmp = first + &*last;

        result_sum += tmp.parse::<u64>().unwrap();
    }
    println!("The result is: {}", result_sum)
}


fn get_number(line: String, reverse: bool) -> u8 {
    let line = {
        if reverse {
            line.chars().rev().collect::<String>()
        } else {
            line
        }
    };
    let iterate_this = line.clone().into_bytes();

    for (idx, c) in enumerate(iterate_this) {
        let foo = char::from(c);
        if foo.is_digit(10) {
            return String::from(foo).parse::<u8>().unwrap()
        }
        if idx >= 2 {
            if let Ok(val) = LetterNumber::from_str(&line[idx-2..=idx]) {
                return val as u8
            }
        }
        if idx >= 3 {
            if let Ok(val) = LetterNumber::from_str(&line[idx-3..=idx]) {
                return val as u8
            }
        }
        if idx >= 4 {
            if let Ok(val) = LetterNumber::from_str(&line[idx-4..=idx]) {
                return val as u8
            }
        }
    }
    panic!("foobar")
}
