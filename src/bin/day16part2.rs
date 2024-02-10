use std::fmt::Display;
use advent_of_code::common::day16::{Beam, Direction, Space};
use advent_of_code::common::day16::Direction::{AboveToBellow, BellowToAbove, LeftToRight, RightToLeft};
use advent_of_code::read_file_into_arrays;


fn try_one_start_path(
    start_x: usize,
    start_y: usize,
    start_direction: Direction,
    mut grid: Vec<Vec<Space>>
) -> i32 {
    let mut beams = vec![Beam{
        previous_step: start_direction,
        x: start_x,
        y: start_y,
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
    res
}

fn main() {
    let grid: Vec<Vec<Space>> = read_file_into_arrays("./src/input/day16/input.txt")
        .iter()
        .map(|row| {
            row
            .iter()
            .map(|space| Space{_char: *space, energized: false, traversed_from: Vec::new()})
            .collect()
        }).collect();
    let mut res = 0;

    for i in 0..grid.len() {
        let tmp = try_one_start_path(0, i as usize, LeftToRight, grid.clone());
        if tmp > res {
            res = tmp;
        }

        let tmp = try_one_start_path(grid[0].len()-1 as usize, i as usize, RightToLeft, grid.clone());
        if tmp > res {
            res = tmp;
        }
    }

    for i in 0..grid[0].len() {
        let tmp = try_one_start_path(i as usize, 0, AboveToBellow, grid.clone());
        if tmp > res {
            res = tmp;
        }

        let tmp = try_one_start_path(i as usize, grid.len()-1 as usize, BellowToAbove, grid.clone());
        if tmp > res {
            res = tmp;
        }
    }
    // let res = try_one_start_path(grid.clone());
    println!("{}", res)
}
