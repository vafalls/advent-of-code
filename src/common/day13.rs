pub fn transpose(rows: Vec<Vec<char>>) -> Vec<Vec<char>> {
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
