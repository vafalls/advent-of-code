use std::str::FromStr;
use once_cell::sync::Lazy;
use regex::Regex;
use advent_of_code::{read_file_into_strings};

#[derive(Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'U' => Direction::U,
            'D' => Direction::D,
            'L' => Direction::L,
            'R' => Direction::R,
            _ => { panic!("Couldn't parse Direction") }
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: i16,
}

static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?<direction>[A-Z]) (?<distance>[0-9]+) ").unwrap());

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = REGEX.captures(s).expect(s);
        Ok(Instruction {
            direction: Direction::from(captures["direction"].parse::<char>().unwrap()),
            distance: *&captures["distance"].parse::<i16>().unwrap(),
        })
    }
}

impl Instruction {
    fn _get_range(&self, start_coordinate: &Coordinate) -> Box<dyn Iterator<Item=i16>> {
        match self.direction {
            Direction::U => Box::new((start_coordinate.y - self.distance..=start_coordinate.y - 1).rev()),
            Direction::D => Box::new(start_coordinate.y + 1..=start_coordinate.y + self.distance),
            Direction::L => Box::new((start_coordinate.x - self.distance..=start_coordinate.x - 1).rev()),
            Direction::R => Box::new(start_coordinate.x + 1..=start_coordinate.x + self.distance),
        }
    }

    fn get_coordinates(&self, start_coordinate: &Coordinate) -> Vec<Coordinate> {
        let mut res: Vec<Coordinate> = Vec::new();
        let range = self._get_range(start_coordinate);
        match self.direction {
            Direction::U | Direction::D => {
                for y in range {
                    res.push(Coordinate {
                        y,
                        x: start_coordinate.x,
                    });
                }
            }
            Direction::L | Direction::R => {
                for x in range {
                    res.push(Coordinate {
                        x,
                        y: start_coordinate.y,
                    });
                }
            }
        }
        res
    }
}

struct Coordinate {
    x: i16,
    y: i16,
}

struct MinMax {
    min_x: i16,
    max_x: i16,
    min_y: i16,
    max_y: i16,
    x_offset: i16,
    y_offset: i16,
}

impl MinMax {
    fn init(coordinates: &Vec<Coordinate>) -> Self {
        let mut min_max = MinMax {
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            x_offset: 0,
            y_offset: 0,
        };

        for coordinate in coordinates {
            if coordinate.x < min_max.min_x {
                min_max.min_x = coordinate.x
            }
            if coordinate.x > min_max.max_x {
                min_max.max_x = coordinate.x
            }
            if coordinate.y < min_max.min_y {
                min_max.min_y = coordinate.y
            }
            if coordinate.y > min_max.max_y {
                min_max.max_y = coordinate.y
            }
        }
        min_max.x_offset += min_max.min_x.abs();
        min_max.y_offset += min_max.min_y.abs();
        min_max
    }
}

fn adjust_coordinates_and_get_terrain(coordinates: &mut Vec<Coordinate>) -> Vec<Vec<char>> {
    let min_max = MinMax::init(coordinates);
    coordinates.iter_mut().for_each(|coordinate| {
        coordinate.x += min_max.x_offset;
        coordinate.y += min_max.y_offset;
    });
    let mut terrain: Vec<Vec<char>> = vec![
        vec!['.'; (min_max.max_x + min_max.x_offset + 1) as usize];
        (min_max.max_y + min_max.y_offset + 1) as usize as usize
    ];
    for coordinate in coordinates {
        terrain[coordinate.y as usize][coordinate.x as usize] = '#';
    }
    terrain
}

fn main() {
    let dig_plan: Vec<Instruction> = read_file_into_strings("./src/input/day18/input.txt")
        .iter_mut()
        .map(|row| Instruction::from_str(row.as_str()).unwrap()).collect();

    let mut coordinates: Vec<Coordinate> = vec![Coordinate { x: 0, y: 0 }];

    for instruction in dig_plan {
        let mut tmp = instruction.get_coordinates(coordinates.last().unwrap());
        coordinates.append(&mut tmp);
    }
    let mut terrain = adjust_coordinates_and_get_terrain(&mut coordinates);

    dig_out_terrain(&mut terrain);

    let mut res = 0;
    for row in terrain {
        for _char in row {
            if _char == '#' {
                res += 1;
            }
        }
    }
    println!("{}", res);
}

fn something(start_y: isize, start_x: isize, terrain: &mut Vec<Vec<char>>) {
    let mut to_be_checked: Vec<(isize, isize)> = vec![(start_x, start_y)];
    while !to_be_checked.is_empty() {
        let (x, y) = to_be_checked.pop().unwrap();

        if terrain[y as usize][x as usize] == '.' {
            terrain[y as usize][x as usize] = '#';
        } else {
            continue;
        }
        let foo = vec![
            (x-1, y-1),
            (x-1, y),
            (x-1, y+1),
            (x, y-1),
            (x, y+1),
            (x+1, y-1),
            (x+1, y),
            (x-1, y+1)
        ];
        foo.iter()
            .filter(|(x, y)| *x>=0 && *x<terrain[0].len() as isize && *y>=0 && *y<terrain.len() as isize)
            .for_each(|x| to_be_checked.push(*x));
    }

}

fn dig_out_terrain(terrain: &mut Vec<Vec<char>>) {
    for i in 1..terrain.len() {
        for j in 1..terrain[0].len() {
            if terrain[i][j] == '.' && terrain[i][j-1] == '#' {
                something(i as isize, j as isize, terrain);
                return
            }
        }
    }

}
