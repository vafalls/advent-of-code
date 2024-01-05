use advent_of_code::{read_file_into_strings};
use std::iter::Iterator;

struct Equation {
    t: f64,
    d: f64,
}

impl Equation {
    /// returns the number of ways to beat the distance d for a given time t
    fn solve_quadratic(&self) -> u64 {
        let first = -self.t / 2.0;
        let second = ((self.t / 2.0).powi(2) - self.d).powf(0.5);
        let start = (first - second).ceil() as u64;
        let stop = (first + second).floor() as u64;
        stop - start + 1
    }
}

fn parse_lines(lines: Vec<String>) -> Vec<Equation> {
    let times: Vec<f64> = lines.first().unwrap().split(":").last().unwrap().trim()
        .split(" ").map(|x| x.parse::<f64>().unwrap()).collect();

    let distanses: Vec<f64> = lines.last().unwrap().split(":").last().unwrap().trim()
        .split(" ").map(|x| x.parse::<f64>().unwrap()).collect();

    let mut res: Vec<Equation> = Vec::new();
    for i in 0..times.len() {
        // a very small fraction is added to the distance to
        // make sure that we "go farther" than the current record
        res.push(Equation { t: -times[i], d: distanses[i] + 10f64.powi(-1000000) })
    }
    res
}

/// This is the same solution for both part 1 & 2
fn main() {
    // let lines = read_file_into_strings("./src/input/day6/input.txt");
    let lines = read_file_into_strings("./src/input/day6/input_part2.txt");

    let equations = parse_lines(lines);
    let res: u64 = equations.iter().map(Equation::solve_quadratic).product();
    println!("{}", res)
}
