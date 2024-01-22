use itertools::enumerate;
use advent_of_code::read_file_into_arrays;

struct Coordinate {
    x: usize,
    y: usize,
    next_direction: Direction,
    is_starting_position: bool,
}

fn get_coordinates_of_s(lines: &Vec<Vec<char>>) -> Coordinate {
    for (y_idx, y) in enumerate(lines) {
        for (x_idx, _) in enumerate(y) {
            if lines[y_idx][x_idx] == 'S' {
                // The starting direction is hardcoded
                return Coordinate { x: x_idx, y: y_idx, next_direction: Direction::Bellow, is_starting_position: false };
            }
        }
    }
    panic!("Couldn't find start value 'S'")
}

enum Direction {
    Above,
    Bellow,
    ToTheLeft,
    ToTheRight,
}

#[derive(PartialEq)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Beginning
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Tile::NorthSouth,
            '-' => Tile::EastWest,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            'S' => Tile::Beginning,
            _ => panic!("Couldn't parse tile")
        }
    }
}

impl Coordinate {
    fn step_in_direction(&mut self, lines: &Vec<Vec<char>>) {
        match self.next_direction{
            Direction::Above => self.y -= 1,
            Direction::Bellow => self.y += 1,
            Direction::ToTheLeft => self.x -= 1,
            Direction::ToTheRight => self.x += 1
        }
        let tile = Tile::from(lines[self.y][self.x]);

        if tile == Tile::Beginning {
            self.is_starting_position = true;
            return;
        }
        self.next_direction = match (&self.next_direction, &tile) {
            (Direction::Above, Tile::NorthSouth) => Direction::Above,
            (Direction::Above, Tile::SouthEast) => Direction::ToTheRight,
            (Direction::Above, Tile::SouthWest) => Direction::ToTheLeft,

            (Direction::Bellow, Tile::NorthSouth) => Direction::Bellow,
            (Direction::Bellow, Tile::NorthEast) => Direction::ToTheRight,
            (Direction::Bellow, Tile::NorthWest) => Direction::ToTheLeft,

            (Direction::ToTheLeft, Tile::EastWest) => Direction::ToTheLeft,
            (Direction::ToTheLeft, Tile::SouthEast) => Direction::Bellow,
            (Direction::ToTheLeft, Tile::NorthEast) => Direction::Above,

            (Direction::ToTheRight, Tile::EastWest) => Direction::ToTheRight,
            (Direction::ToTheRight, Tile::SouthWest) => Direction::Bellow,
            (Direction::ToTheRight, Tile::NorthWest) => Direction::Above,
            (_, _) => panic!("Couldn't find the next direction")
        };
    }
}

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
