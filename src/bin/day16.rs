use std::fmt::{Display, Formatter};
use colored::Colorize;
use advent_of_code::read_file_into_arrays;
use crate::Direction::LeftToRight;
use crate::Direction::RightToLeft;
use crate::Direction::AboveToBellow;
use crate::Direction::BellowToAbove;

struct Space {
    _char: char,
    energized: bool,
    traversed_from: Vec<Direction>
}

impl Display for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.energized {
            print!("{}", self._char.to_string().on_green());
        } else {
            print!("{}", self._char);
        }
        write!(f, "")
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    LeftToRight,
    RightToLeft,
    AboveToBellow,
    BellowToAbove
}

#[derive(Debug)]
struct Beam {
    previous_step: Direction,
    x: usize,
    y: usize,
    delete_me: bool
}

fn is_within_grid(x: i16, y: i16, grid: &Vec<Vec<Space>>) -> bool{
    return 0 <= x &&
        x <= grid[0].len() as i16 - 1 &&
        0 <= y &&
        y <= grid.len() as i16 - 1
}


impl Beam {

    fn _get_new_coordinates(&mut self, grid: &Vec<Vec<Space>>) -> Option<(i16, i16)> {
        match (&self.previous_step, &grid[self.y][self.x]._char) {
            (LeftToRight, '.') | (LeftToRight, '-') => Some((self.x as i16 + 1, self.y as i16)),
            (LeftToRight, '/') => {
                self.previous_step = BellowToAbove;
                Some((self.x as i16, self.y as i16 - 1))
            },
            (LeftToRight, '\\') => {
                self.previous_step = AboveToBellow;
                Some((self.x as i16, self.y as i16 + 1))
            },

            (RightToLeft, '.') | (RightToLeft, '-') => Some((self.x as i16 - 1, self.y as i16)),
            (RightToLeft, '/') => {
                self.previous_step = AboveToBellow;
                Some((self.x as i16, self.y as i16 + 1))
            },
            (RightToLeft, '\\') => {
                self.previous_step = BellowToAbove;
                Some((self.x as i16, self.y as i16 - 1))
            },

            (AboveToBellow, '.') | (AboveToBellow, '|') => Some((self.x as i16, self.y as i16 + 1)),
            (AboveToBellow, '/') => {
                self.previous_step = RightToLeft;
                Some((self.x as i16 - 1, self.y as i16))
            },
            (AboveToBellow, '\\') => {
                self.previous_step = LeftToRight;
                Some((self.x as i16 + 1, self.y as i16))
            },

            (BellowToAbove, '.') | (BellowToAbove, '|') => Some((self.x as i16, self.y as i16 - 1)),
            (BellowToAbove, '/') => {
                self.previous_step = LeftToRight;
                Some((self.x as i16 + 1, self.y as i16))
            },
            (BellowToAbove, '\\') => {
                self.previous_step = RightToLeft;
                Some((self.x as i16 - 1, self.y as i16))
            },

            (_, '|') | (_, '-') => None,
            _ => {panic!("asdf")}
        }
    }

    fn _create_new_beams(&mut self, grid: &Vec<Vec<Space>>) -> Option<Beam>{
        match &grid[self.y][self.x]._char {
            '|' => {
                let above = is_within_grid(self.x as i16, self.y as i16 - 1, grid);
                let bellow = is_within_grid(self.x as i16, self.y as i16 + 1, grid);
                return match (above, bellow) {
                    (true, true) => {
                        self.y += 1;
                        self.previous_step = AboveToBellow;
                        Some(Beam {
                            previous_step: BellowToAbove,
                            x: self.x,
                            y: self.y - 2,
                            delete_me: false
                        })
                    },
                    (false, true) => {
                        self.y += 1;
                        self.previous_step = AboveToBellow;
                        None
                    }
                    (true, false) => {
                        self.y -= 1;
                        self.previous_step = BellowToAbove;
                        None

                    },
                    (_, _) => {panic!("what the hell!")}
                }
            },
            '-' => {
                let right = is_within_grid(self.x as i16 + 1, self.y as i16, grid);
                let left = is_within_grid(self.x as i16 - 1, self.y as i16, grid);
                return match (right, left) {
                    (true, true) => {
                        self.x += 1;
                        self.previous_step = LeftToRight;
                        Some(Beam {
                            previous_step: RightToLeft,
                            x: self.x - 2,
                            y: self.y,
                            delete_me: false
                        })
                    },
                    (false, true) => {
                        self.x -= 1;
                        self.previous_step = RightToLeft;
                        None
                    }
                    (true, false) => {
                        self.x += 1;
                        self.previous_step = LeftToRight;
                        None
                    },
                    (_, _) => {panic!("what the hell!")}
                }
            },
            _ => {panic!("foobar")}
        }
    }

    fn step(&mut self, grid: &mut Vec<Vec<Space>>) -> Option<Beam> {
        grid[self.y][self.x].energized = true;
        if grid[self.y][self.x].traversed_from.contains(&self.previous_step) {
            self.delete_me = true;
            return None
        } else {
            grid[self.y][self.x].traversed_from.push(self.previous_step.clone())
        }
        return match self._get_new_coordinates(grid) {
            None => {
                self._create_new_beams(grid)
            }
            Some((new_x, new_y)) => {
                if is_within_grid(new_x, new_y, grid) {
                    self.x = new_x as usize;
                    self.y = new_y as usize;
                    None
                } else {
                    self.delete_me = true;
                    None
                }
            }
        }
    }
}

fn main() {
    let mut grid = read_file_into_arrays("./src/input/day16/input.txt")
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
