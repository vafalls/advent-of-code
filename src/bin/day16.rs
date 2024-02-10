use std::fmt::Display;
use advent_of_code::common::day16::{Beam, Space};
use advent_of_code::common::day16::Direction::LeftToRight;
use advent_of_code::read_file_into_arrays;

fn main() {
    let mut grid: Vec<Vec<Space>> = read_file_into_arrays("./src/input/day16/input.txt")
        .iter()
        .map(|row| {
            row
            .iter()
            .map(|space| Space{_char: *space, energized: false, traversed_from: Vec::new()})
            .collect()
        }).collect();

    let mut beams = vec![Beam{
        previous_step: LeftToRight,
        x: 0,
        y: 0,
        delete_me: false
    }];

    loop {
        let mut new_beams = Vec::new();

        for beam in &mut beams {
            match beam.step(&mut grid) {
                Some(new_beam) => {
                    new_beams.push(new_beam);
                }
                None => {}
            }
        }
        beams.retain(|beam| !beam.delete_me);
        beams.append(&mut new_beams);

        if beams.len() == 0 {
            break;
        }
    }
    let mut res = 0;
    for row in grid {
        for space in row {
            if space.energized {
                res += 1;
            }
        }
    }
    println!("{}", res);
}
