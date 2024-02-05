use itertools::enumerate;
use advent_of_code::read_file_into_arrays;

fn main() {
    let mut all_rows = read_file_into_arrays("./src/input/day14/input.txt");

    for i in 1..all_rows.len() {
        for j in (1..=i).rev() {
            for s in 0..all_rows[j].len() {
                if all_rows[j][s] == 'O' && all_rows[j-1][s] == '.' {
                    all_rows[j-1][s] = all_rows[j][s];
                    all_rows[j][s] = '.';
                }
            }
        }
    }

    let all_rows_length = all_rows.len();
    let mut res = 0;
    for (idx, row) in enumerate(all_rows) {
        for s in row {
            if s == 'O' {
                res += 1 * (all_rows_length - idx);
            }
        }
    }
    println!("{}", res);
}
