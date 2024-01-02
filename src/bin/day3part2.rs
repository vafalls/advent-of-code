use std::collections::HashMap;
use itertools::{enumerate};
use advent_of_code::read_file_into_arrays;


enum LinePosition {
    First,
    Middle,
    Last
}

impl LinePosition {
    fn get(idx: usize, max_size: usize) -> Self {
        return if idx == 0 {
            LinePosition::First
        } else if idx == max_size {
            LinePosition::Last
        } else {
            LinePosition::Middle
        }
    }
}

#[derive(Eq, Hash, Debug)]
struct Coordinates {
    x: usize,
    y: usize
}

impl PartialEq<Self> for Coordinates {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

#[derive(Debug)]
struct Gears {
    g1: Option<u32>,
    g2: Option<u32>,
}

fn calculate(gears: &Gears) -> u64 {
    return gears.g1.unwrap_or(0) as u64 * gears.g2.unwrap_or(0) as u64;
}


fn get_gear_coordinates(
    start_idx: usize,
    end_idx: usize,
    lines: &Vec<Vec<char>>,
    line_position: LinePosition,
    y_idx: usize,
) -> Option<Coordinates> {
    let proximity_start_idx = if start_idx == 0 { 0 } else {
        if lines[y_idx][start_idx - 1] == '*' {
            return Some(Coordinates { x: start_idx-1, y: y_idx });
        }
        start_idx - 1
    };

    let proximity_end_idx = if end_idx == lines[y_idx].len() - 1 { end_idx } else {
        if lines[y_idx][end_idx + 1] == '*' {
            return Some(Coordinates { x: end_idx+1, y: y_idx });
        }
        end_idx + 1
    };

    let _inner = |line: &Vec<char>| -> Option<usize> {
        for i in proximity_start_idx..=proximity_end_idx {
            if line[i] == '*' {
                return Some(i);
            }
        }
        None
    };

    match line_position {
        LinePosition::First => {
            if let Some(x) = _inner(&lines[y_idx+1]) {
                return Some(Coordinates { x, y: y_idx + 1 });
            }
        }
        LinePosition::Middle => {
            if let Some(x) = _inner(&lines[y_idx+1]) {
                return Some(Coordinates { x, y: y_idx + 1 });
            }
            if let Some(x) = _inner(&lines[y_idx-1]) {
                return Some(Coordinates { x, y: y_idx - 1 });
            }
        }
        LinePosition::Last => {
            if let Some(x) = _inner(&lines[y_idx-1]) {
                return Some(Coordinates { x, y: y_idx - 1 });
            }
        }
    }
    None
}

fn calculate_part(
    number: &mut String,
    x_idx: usize,
    y_idx: usize,
    lines: &Vec<Vec<char>>,
    max_y_idx: usize
) -> Option<(Coordinates, u32)> {
    let start_idx = {
        let tmp: i32 = (x_idx as i32) - (number.len() as i32 - 1);
        if tmp < 0 {
            0
        } else {
            tmp
        }
    } as usize;

    return match get_gear_coordinates(start_idx, x_idx, &lines, LinePosition::get(y_idx, max_y_idx), y_idx) {
        Some(v) => {
            let res = number.parse::<u32>().unwrap();
            number.clear();
            Some((v, res))
        }
        None => {
            number.clear();
            None
        }
    }
}

fn main() {
    let mut res: HashMap<Coordinates, Gears> = HashMap::new();

    let lines = read_file_into_arrays("./src/input/day3/input.txt");

    let max_y_idx = lines.len() - 1;
    let mut number = String::new();

    for (y_idx, line) in enumerate(&lines) {
        for (x_idx, char) in enumerate(line) {
            if char.is_numeric() {
                number.push(*char);
                if x_idx == line.len() - 1 {
                    if let Some(v) = calculate_part(&mut number, x_idx, y_idx, &lines, max_y_idx) {
                        res.entry(v.0).and_modify(|gears| gears.g2 = Some(v.1)).or_insert(Gears { g1: Some(v.1), g2: None });
                    }
                }
            } else if !number.is_empty() {
                if let Some(v) = calculate_part(&mut number, x_idx-1, y_idx, &lines, max_y_idx) {
                    res.entry(v.0).and_modify(|gears| gears.g2 = Some(v.1)).or_insert(Gears { g1: Some(v.1), g2: None });
                }
            }
        }
    }
    let final_res: u64 = res.iter().map(|(_, gear)| calculate(gear)).sum();

    println!("{}", final_res)
}
