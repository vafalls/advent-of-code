use std::collections::HashMap;
use itertools::enumerate;
use advent_of_code::read_file_into_arrays;

enum Direction {
    North,
    West,
    South,
    East,
}

fn shift_rock_north_or_south(all_rows: &mut Vec<Vec<char>>, row_idx: usize, col_idx: usize, step: usize) {
    if all_rows[row_idx][col_idx] == 'O' && all_rows[step][col_idx] == '.' {
        all_rows[step][col_idx] = all_rows[row_idx][col_idx];
        all_rows[row_idx][col_idx] = '.';
    }
}

fn shift_rock_east_or_west(all_rows: &mut Vec<Vec<char>>, row_idx: usize, col_idx: usize, step: usize) {
    if all_rows[row_idx][col_idx] == 'O' && all_rows[row_idx][step] == '.' {
        all_rows[row_idx][step] = all_rows[row_idx][col_idx];
        all_rows[row_idx][col_idx] = '.';
    }
}


fn roll_rocks(all_rows: &mut Vec<Vec<char>>, direction: Direction) {
    match direction {
        Direction::North => {
            for i in 1..all_rows.len() {
                for j in (1..=i).rev() {
                    for s in 0..all_rows[j].len() {
                        shift_rock_north_or_south(all_rows, j, s, j - 1);
                    }
                }
            }
        }
        Direction::West => {
            for i in 1..all_rows[0].len() {
                for j in (1..=i).rev() {
                    for s in 0..all_rows.len() {
                        shift_rock_east_or_west(all_rows, s, j, j - 1);
                    }
                }
            }
        }
        Direction::South => {
            for i in (0..all_rows.len()-1).rev() {
                for j in i..all_rows.len()-1 {
                    for s in 0..all_rows[j].len() {
                        shift_rock_north_or_south(all_rows, j, s, j + 1);
                    }
                }
            }
        }
        Direction::East => {
            for i in (0..all_rows[0].len()-1).rev() {
                for j in i..all_rows[0].len()-1 {
                    for s in 0..all_rows.len() {
                        shift_rock_east_or_west(all_rows, s, j, j + 1);
                    }
                }
            }
        }
    }
}

fn roll_cycle(all_rows: &mut Vec<Vec<char>>) {
    roll_rocks(all_rows, Direction::North);
    roll_rocks(all_rows, Direction::West);
    roll_rocks(all_rows, Direction::South);
    roll_rocks(all_rows, Direction::East);
}

struct Foo {
    platform: Vec<Vec<char>>,
    north_load: usize
}
impl Foo {
    fn init(platform: Vec<Vec<char>>) -> Foo {
        Foo {
            north_load: calculate_north_load(&platform),
            platform,
        }
    }
}

fn main() {
    let mut all_rows = read_file_into_arrays("./src/input/day14/input.txt");

    let mut map: HashMap<u32, Foo> = HashMap::new();
    let mut counter = 1;
    let mut loop_found = false;

    const TOTAL_STEPS: u32 = 1e9 as u32;
    loop {
        roll_cycle(&mut all_rows);
        if counter == TOTAL_STEPS {
            break;
        }

        if !loop_found {
            for (key, value) in map.iter() {
                if *value.platform == all_rows {
                    let loop_len = counter - *key;
                    let x = (TOTAL_STEPS - counter)/loop_len;
                    let steps_to_take = loop_len*x;
                    counter += steps_to_take;
                    loop_found = true;
                }
            }
            map.insert(counter, Foo::init(all_rows.clone()));
        }

        counter += 1;
    }

    let north_load = calculate_north_load(&all_rows);
    println!("{}", north_load);
}


fn calculate_north_load(all_rows: &Vec<Vec<char>>) -> usize {
    let all_rows_length = all_rows.len();
    let mut res = 0;
    for (idx, row) in enumerate(all_rows) {
        for s in row {
            if *s == 'O' {
                res += 1 * (all_rows_length - idx);
            }
        }
    }
    res
}