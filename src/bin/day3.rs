use itertools::{enumerate};
use advent_of_code::read_file_into_arrays;


fn is_adjacent_number(
    start_idx: usize,
    end_idx: usize,
    previous_line: Option<&Vec<char>>,
    current_line: &Vec<char>,
    next_line: Option<&Vec<char>>,
) -> bool {
    let proximity_start_idx = if start_idx == 0 { 0 } else {
        if current_line[start_idx - 1] != '.' {
            return true;
        }
        start_idx - 1
    };

    let proximity_end_idx = if end_idx == current_line.len() - 1 { end_idx } else {
        if current_line[end_idx + 1] != '.' {
            return true;
        }
        end_idx + 1
    };

    let _inner = |line_option: Option<&Vec<char>>| {
        if let Some(line) = line_option {
            for i in proximity_start_idx..=proximity_end_idx {
                if line[i] != '.' {
                    return true;
                }
            }
        };
        false
    };

    if _inner(previous_line) || _inner(next_line) {
        return true;
    }

    false
}

fn calculate_part(number: &mut String, x_idx: usize, y_idx: usize, lines: &Vec<Vec<char>>, max_y_idx: usize) -> u32 {
    let start_idx = {
        let tmp: i32 = (x_idx as i32) - (number.len() as i32 - 1);
        if tmp < 0 {
            0
        } else {
            tmp
        }
    } as usize;
    let previous_line = if y_idx > 0 { Some(&lines[y_idx - 1]) } else { None };
    let next_line = if y_idx < max_y_idx { Some(&lines[y_idx + 1]) } else { None };

    if is_adjacent_number(start_idx, x_idx, previous_line, &lines[y_idx], next_line) {
        let res = number.parse::<u32>().unwrap();
        number.clear();
        res
    } else {
        number.clear();
        0
    }
}

fn main() {
    let lines = read_file_into_arrays("./src/input/day3/input.txt");

    let max_y_idx = lines.len() - 1;

    let mut res: u32 = 0;

    let mut number = String::new();

    for (y_idx, line) in enumerate(&lines) {
        for (x_idx, char) in enumerate(line) {
            if char.is_numeric() {
                number.push(*char);
                if x_idx == line.len() - 1 {
                    res += calculate_part(&mut number, x_idx, y_idx, &lines, max_y_idx)
                }
            } else if !number.is_empty() {
                res += calculate_part(&mut number, x_idx - 1, y_idx, &lines, max_y_idx)
            }
        }
    }
    println!("{}", res)
}
