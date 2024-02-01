use std::fmt::{Display, Formatter};
use itertools::enumerate;
use advent_of_code::{read_file_into_strings};

#[derive(Debug, Clone)]
struct Row {
    lava: Vec<char>,
    springs: Vec<u8>,
}

impl Display for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}",
            self.lava.iter().collect::<String>(),
            self.springs.iter().map(|item| item.to_string()).collect::<String>()
        )
    }
}

fn parse_rows(lines: Vec<String>) -> Vec<Row> {
    let mut res: Vec<Row> = Vec::new();
    for line in lines {
        let parts: Vec<_> = line.split(' ').collect();
        let tmp = *parts.first().unwrap();
        res.push(Row {
            lava: tmp.chars().collect(),
            springs: parts.last()
                .unwrap()
                .split(',')
                .map(|c| c.parse::<u8>().unwrap())
                .collect(),
        });
    }
    res
}

fn calculate_spring(spring: u8, possible_places: &Vec<char>) {


}


fn main() {
    let lines = read_file_into_strings("./src/input/day12/test_input.txt");

    let rows = parse_rows(lines);
    for row in rows {
        for (idx, spring) in enumerate(row.springs) {
            calculate_spring(spring, );

        }
    }

    // let foo = find_possible_placements(&rows[1]);
    // let bar = calculate_possible(foo);
    // let arrangements: Vec<Vec<Arrangement>> = rows.iter().map(|row| find_possible_arrangements(row)).collect();
    // println!("{}", rows[0]);
}


