use advent_of_code::read_lines_from_file;
use itertools::join;
use std::fmt::Debug;
use std::str::FromStr;

enum CubeColor {
    Red,
    Green,
    Blue,
}

struct CubeColorCount {
    color: CubeColor,
    count: u8,
}

struct NoColorMatchError;

impl FromStr for CubeColorCount {
    type Err = NoColorMatchError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let _text = join(input.chars().filter(|a| a.is_alphabetic()), "");
        let _num = join(input.chars().filter(|a| a.is_digit(10)), "").parse::<u8>();

        match (_text.as_str(), _num) {
            ("red", Ok(_num)) => Ok(CubeColorCount {
                color: CubeColor::Red,
                count: _num,
            }),
            ("green", Ok(_num)) => Ok(CubeColorCount {
                color: CubeColor::Green,
                count: _num,
            }),
            ("blue", Ok(_num)) => Ok(CubeColorCount {
                color: CubeColor::Blue,
                count: _num,
            }),
            _ => Err(NoColorMatchError),
        }
    }
}

#[derive(Debug)]
struct MinValues {
    red: u32,
    green: u32,
    blue: u32,
}

impl Default for MinValues {
    fn default() -> Self {
        MinValues {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl MinValues {
    fn foobar(&mut self, color_count: CubeColorCount) {
        match color_count.color {
            CubeColor::Red => {
                if self.red < color_count.count as u32 {
                    self.red = color_count.count as u32
                }
            }
            CubeColor::Green => {
                if self.green < color_count.count as u32 {
                    self.green = color_count.count as u32
                }
            }
            CubeColor::Blue => {
                if self.blue < color_count.count as u32 {
                    self.blue = color_count.count as u32
                }
            }
        }
    }

    fn get_res(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn get_game_sets(game_set_str: &str) -> u32 {
    let cube_color_counts: Vec<_> = game_set_str
        .split(|c| c == ',' || c == ';')
        .map(CubeColorCount::from_str)
        .collect();
    let mut values = MinValues::default();
    for cube_color_count in cube_color_counts {
        if let Ok(val) = cube_color_count {
            values.foobar(val)
        }
    }
    values.get_res()
}

fn parse_game(line: String) -> u32 {
    let splits: Vec<_> = line.split(":").collect();
    get_game_sets(splits.last().unwrap())
}

fn main() {
    let mut res: u32 = 0;
    for line in read_lines_from_file("./src/input/day2/input.txt") {
        let one_line = line.expect("nope");
        res += parse_game(one_line);
    }
    println!("{}", res)
}
