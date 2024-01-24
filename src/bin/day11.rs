use advent_of_code::common::day11::{get_galaxies, get_pairs};
use advent_of_code::read_file_into_arrays;

fn expand_universe(lines: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = lines.clone();
    for i in (0..lines.len()).rev() {
        let line_contains_galaxy = lines[i].iter().any(|tile| *tile == '#');
        if !line_contains_galaxy {
            let line_to_be_inserted: Vec<char> = (0..lines[i].len()).map(|_| '.').collect();
            let mut lines_to_be_inserted = Vec::new();
            lines_to_be_inserted.push(line_to_be_inserted);
            res.splice(i+1..i+1, lines_to_be_inserted);
        }
    }

    let column_to_insert: Vec<char> = Vec::from(['.']);
    for i in (0..res[0].len()).rev() {
        let column_contains_galaxy = res.iter().map(|line| line[i]).any(|tile| tile == '#');
        if !column_contains_galaxy {
            for j in 0..res.len() {
                res[j].splice(i..i, column_to_insert.clone());
            }
        }
    }
    return res
}


fn main() {
    let lines = read_file_into_arrays("./src/input/day11/input.txt");
    let lines = expand_universe(lines);
    let galaxies = get_galaxies(&lines);
    let mut galaxy_pairs = get_pairs(&galaxies);

    galaxy_pairs.iter_mut().for_each(|galaxy_pair| {
        let x_diff = (galaxy_pair.first.x as i32) - (galaxy_pair.second.x as i32);
        let y_diff = (galaxy_pair.first.y as i32) - (galaxy_pair.second.y as i32);
        galaxy_pair.diff = x_diff.abs() as u32 + y_diff.abs() as u32;
    });
    let res: u32 = galaxy_pairs.iter().map(|pair| pair.diff).sum();
    println!("{}", res);
}