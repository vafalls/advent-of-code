use advent_of_code::read_lines_from_file;
use itertools::join;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

struct CubeColorCount {
    color: CubeColor,
    count: u8,
}

impl FromStr for CubeColorCount {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let _text = join(input.chars().filter(|a| a.is_alphabetic()), "");
        let _num = join(input.chars().filter(|a| a.is_digit(10)), "").parse::<u8>();

        match (_text.as_str(), _num) {
            ("red", Ok(_num)) => {
                if _num > 12 {
                    return Err(());
                }
                Ok(CubeColorCount {
                    color: CubeColor::Red,
                    count: _num,
                })
            }
            ("green", Ok(_num)) => {
                if _num > 13 {
                    return Err(());
                }
                Ok(CubeColorCount {
                    color: CubeColor::Green,
                    count: _num,
                })
            }
            ("blue", Ok(_num)) => {
                if _num > 14 {
                    return Err(());
                }
                Ok(CubeColorCount {
                    color: CubeColor::Blue,
                    count: _num,
                })
            }
            _ => Err(()),
        }
    }
}

fn is_game_set_possible(game_set_str: &str) -> bool {
    for game_set in game_set_str.split(",") {
        if CubeColorCount::from_str(game_set).is_err() {
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
