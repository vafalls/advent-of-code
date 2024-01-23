use advent_of_code::common::day10::get_coordinates_of_s;
use advent_of_code::read_file_into_arrays;


fn main() {
    let lines = read_file_into_arrays("./src/input/day10/input.txt");
    let mut s_coordinate = get_coordinates_of_s(&lines);

    let mut counter = 0;
    loop {
        s_coordinate.step_in_direction(&lines);
        counter += 1;
        if s_coordinate.is_starting_position {
            break;
        }
    }
    println!("{}", counter / 2);
}
