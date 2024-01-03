use advent_of_code::{read_file_into_strings};


fn parse_game(line: &String) -> usize {
    let splits: Vec<_> = line.split(":").collect();
    let foo = splits.last().unwrap();
    let numbers_str = foo.replace(" | ", " ").replace("  ", " ");
    let mut numbers: Vec<_> = numbers_str.trim().split(" ").collect();
    let length_before = numbers.len();
    numbers.sort();
    numbers.dedup();
    return length_before - numbers.len();
}

struct LotteryLine {
    nr_of_wins: u32,
    copy: u32
}

impl LotteryLine {
    fn from_line(line: &String) -> LotteryLine {
        return LotteryLine { nr_of_wins: parse_game(line) as u32, copy: 1 }
    }
}


fn main() {
    let mut lottery_lines:Vec<LotteryLine> = read_file_into_strings("./src/input/day4/input.txt")
        .iter()
        .map(LotteryLine::from_line)
        .collect();

    for i in 0..lottery_lines.len() {
        for _ in 0..lottery_lines[i].copy {
            for j in 1..=lottery_lines[i].nr_of_wins as usize {
                lottery_lines[i+j].copy += 1;
            }
        }
    }
    let results: Vec<u32> = lottery_lines.iter().map(|lottery_line| lottery_line.copy).collect();

    println!("{}", results.iter().sum::<u32>())
}
