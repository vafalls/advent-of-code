use advent_of_code::common::day13::transpose;
use advent_of_code::read_file_into_arrays;


fn get_nr_of_smudges(above: &Vec<Vec<char>>, bellow: &Vec<Vec<char>>) -> i32 {
    let mut res = 0;
    for i in 0..above.len() {
        for j in 0..above[i].len() {
            if above[i][j] != bellow[i][j] {
                res += 1;
            }
        }
    }
    res
}

fn find_new_idx(rows: &Vec<Vec<char>>) -> Option<usize> {
    let mut smudge_idx = None;
    for r in 1..rows.len() {
        let strt_idx = {
            if r*2 >= rows.len() {
                r-(rows.len()-1-r)-1
            } else {
                0
            }

        };
        let end_idx = {
            if r*2 >= rows.len() {
                rows.len()-1
            } else {
                (r*2)-1
            }
        };

        let rows_above: Vec<Vec<char>> = rows[strt_idx..=r-1].iter().cloned().collect();
        let rows_bellow: Vec<Vec<char>> = rows[r..=end_idx].iter().rev().cloned().collect();
        let nr_of_smudges = get_nr_of_smudges(&rows_above, &rows_bellow);

        if nr_of_smudges == 1 {
            smudge_idx = Some(r)
        }
    }
    smudge_idx
}

fn find_value_for_mirror(layout: &[Vec<char>]) -> u32 {
    let layout = layout.to_vec();

    let row_smudges = find_new_idx(&layout);
    let column_smudges = find_new_idx(&transpose(layout.clone()));
    return match (row_smudges, column_smudges) {
        (Some(_), Some(_)) => panic!("Found smudge for both columns and rows"),
        (Some(row), None) => 100 * row as u32,
        (None, Some(col)) => col as u32,
        (_, _) => panic!("Didn't find any smudge at all :/")
    }
}

fn main() {
    let all_rows = read_file_into_arrays("./src/input/day13/input.txt");

    let res: u32 = all_rows
        .split(|row| row.len() == 0)
        .map(find_value_for_mirror)
        .sum();

    println!("{}", res)
}
