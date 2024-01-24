use advent_of_code::common::day11::{Galaxy, get_galaxies, get_pairs};
use advent_of_code::read_file_into_arrays;

const EXPANSION_FACTOR: u32 = 999999;


fn expand_rows(lines: &Vec<Vec<char>>, galaxies: &mut Vec<Galaxy>) {
    for i in (0..lines.len()).rev() {
        let no_galaxies = lines[i].iter().all(|tile| *tile != '#');
        if no_galaxies {
            for j in 0..galaxies.len() {
                if galaxies[j].y > i {
                    galaxies[j].y += EXPANSION_FACTOR as usize;
                }
            }

        }
    }
}

fn expand_columns(lines: &Vec<Vec<char>>, galaxies: &mut Vec<Galaxy>) {
    for i in (0..lines[0].len()).rev() {
        let no_galaxies_in_column = lines.iter().map(|line| line[i]).all(|tile| tile != '#');
        if no_galaxies_in_column {
            for j in 0..galaxies.len() {
                if galaxies[j].x > i {
                    galaxies[j].x += EXPANSION_FACTOR as usize;
                }
            }
        }
    }
}

fn main() {
    let lines = read_file_into_arrays("./src/input/day11/input.txt");
    let mut galaxies = get_galaxies(&lines);
    expand_rows(&lines, &mut galaxies);
    expand_columns(&lines, &mut galaxies);
    let mut galaxy_pairs = get_pairs(&galaxies);
    galaxy_pairs.iter_mut().for_each(|galaxy_pair| {
        let x_diff = (galaxy_pair.first.x as i32) - (galaxy_pair.second.x as i32);
        let y_diff = (galaxy_pair.first.y as i32) - (galaxy_pair.second.y as i32);
        galaxy_pair.diff = x_diff.abs() as u32 + y_diff.abs() as u32;
    });
    let res: u64 = galaxy_pairs.iter().map(|pair| pair.diff as u64).sum();
    println!("{}", res);
}

