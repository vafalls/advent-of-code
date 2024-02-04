use itertools::enumerate;
use advent_of_code::read_file_into_arrays;


fn find_rows_mirroring_point(rows: &Vec<Vec<char>>) -> usize {
    let mut res = 0;
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

        let first_part: Vec<Vec<char>> = rows[strt_idx..=r-1].iter().cloned().collect();
        let second_part: Vec<Vec<char>> = rows[r..=end_idx].iter().rev().cloned().collect();
        if first_part != second_part {
            continue;
        }
        res = r;
        break;
    }
    res
}

fn transpose(rows: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = Vec::new();

    for c in 0..rows[0].len() {
        let mut new_row = Vec::new();
        for r in 0..rows.len() {
            new_row.push(rows[r][c]);
        }
        res.push(new_row);
    }
    res
}

fn main() {
    let all_rows = read_file_into_arrays("./src/input/day13/input.txt");

    let mut start_idx = 0;
    let mut res = 0;

    for (idx, row) in enumerate(&all_rows) {
        if row.len() == 0 {

            let group = all_rows[start_idx..idx].iter().cloned().collect();

            let row_res = find_rows_mirroring_point(&group) * 100;

            if row_res > 0 {
                res += row_res
            } else {
                let transposed = transpose(group);
                res += find_rows_mirroring_point(&transposed)
            }
            start_idx = idx+1;
        }
    }
    println!("{}", res)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_rows_mirroring_point_happy() {
        let input = vec![
            vec!['#','.','#','#','.','.','#'],
            vec!['.','.','#','#','.','.','.'],
            vec!['#','#','.','.','#','#','#'],
            vec!['#','.','.','.','.','#','.'],
            vec!['.','#','.','.','#','.','#'],
            vec!['.','#','.','.','#','.','#'],
            vec!['#','.','.','.','.','#','.'],
            vec!['#','#','.','.','#','#','#'],
            vec!['.','.','#','#','.','.','.'],
        ];
        let result = find_rows_mirroring_point(&input);
        assert_eq!(result, 5)
    }

    #[test]
    fn test_find_rows_mirroring_point() {
        let input = vec![
            vec!['#','.','#','#','.','.','#','#','.'],
            vec!['.','.','#','.','#','#','.','#','.'],
            vec!['#','#','.','.','.','.','.','.','#'],
            vec!['#','#','.','.','.','.','.','.','#'],
            vec!['.','.','#','.','#','#','.','#','.'],
            vec!['.','.','#','#','.','.','#','#','.'],
            vec!['#','.','#','.','#','#','.','#','.'],
        ];
        let result = find_rows_mirroring_point(&input);
        assert_eq!(result, 0)
    }

}
