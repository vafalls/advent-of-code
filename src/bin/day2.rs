use advent_of_code::read_lines_from_file;
use itertools::join;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum CubeColor {
    Red = 12,
    Green = 13,
    Blue = 14,
}

impl FromStr for CubeColor {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "red" => Ok(CubeColor::Red),
            "green" => Ok(CubeColor::Green),
            "blue" => Ok(CubeColor::Blue),
            _ => Err(()),
        }
    }
}

fn the_fn(input: &str) -> bool{
    let _text = join(input.chars().filter(|a| a.is_alphabetic()), "");
    let _num = join(input.chars().filter(|a| a.is_digit(10)), "").parse::<u8>();

    return match (CubeColor::from_str(_text.as_str()), _num) {
        (Ok(cube_color), Ok(_num)) => {
            if _num > (cube_color as u8) {
                return false;
            }
            true
        }
        _ => {
            false
        }
    }
}

fn is_game_set_possible(game_set_str: &str) -> bool {
    for game_set in game_set_str.split(",") {
        if !the_fn(game_set) {
            return false;
        }
    }
    return true;
}

fn parse_game(line: String) -> u8 {
    let splits: Vec<_> = line.split(":").collect();
    let game_str = splits.first().unwrap().split(" ").collect::<Vec<_>>();

    for game_set_str in splits.last().unwrap().split(";") {
        if !is_game_set_possible(game_set_str) {
            return 0;
        }
    }
    return game_str.last().unwrap().parse::<u8>().unwrap();
}

fn main() {
    let mut res: u32 = 0;
    for line in read_lines_from_file("./src/input/day2/input.txt") {
        let one_line = line.expect("nope");
        res += parse_game(one_line) as u32;
    }
    println!("{}", res)
}
