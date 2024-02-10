use std::fmt::{Display, Formatter};
use colored::Colorize;
use crate::common::day16::Direction::LeftToRight;
use crate::common::day16::Direction::RightToLeft;
use crate::common::day16::Direction::BellowToAbove;
use crate::common::day16::Direction::AboveToBellow;

#[derive(Clone)]
pub struct Space {
    pub _char: char,
    pub energized: bool,
    pub traversed_from: Vec<Direction>
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
pub enum Direction {
    LeftToRight,
    RightToLeft,
    AboveToBellow,
    BellowToAbove
}

#[derive(Debug)]
pub struct Beam {
    pub previous_step: Direction,
    pub x: usize,
    pub y: usize,
    pub delete_me: bool
}

pub fn is_within_grid(x: i16, y: i16, grid: &Vec<Vec<Space>>) -> bool{
    return 0 <= x &&
        x <= grid[0].len() as i16 - 1 &&
        0 <= y &&
        y <= grid.len() as i16 - 1
}


impl Beam {

    pub fn _get_new_coordinates(&mut self, grid: &Vec<Vec<Space>>) -> Option<(i16, i16)> {
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

    pub fn _create_new_beams(&mut self, grid: &Vec<Vec<Space>>) -> Option<Beam>{
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

    pub fn step(&mut self, grid: &mut Vec<Vec<Space>>) -> Option<Beam> {
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
