use std::fmt::Display;
use colored::Colorize;
use advent_of_code::read_file_into_arrays;


struct Node {
    sign: char,
    type_of_tile: TypeOfTile,
    checked: bool
}

impl Node {
    fn init(_char: char) -> Self {
        Node{
            sign: _char,
            type_of_tile: TypeOfTile::from(_char),
            checked: false
        }
    }

    fn check_if_ground_and_mark(&mut self) -> bool {
        self.checked = true;
        return self.type_of_tile == TypeOfTile::Ground;
    }
}


fn convert_chars_to_nodes(lines: &Vec<Vec<char>>) -> Vec<Vec<Node>>{
    let mut res = Vec::new();

    for line in lines {
        let mut inner_res: Vec<Node> = Vec::new();
        for _char in line {
            inner_res.push(Node::init(*_char));
        }
        res.push(inner_res);
    }
    res
}

fn ppp(map: &Vec<Vec<Node>>) {
    for m in map {
        for n in m {
            match n.type_of_tile {
                TypeOfTile::Side => print!("{}", n.sign.to_string().on_blue()),
                TypeOfTile::Corner => print!("{}", n.sign.to_string().on_blue()),
                TypeOfTile::Start => print!("{}", n.sign.to_string().on_red()),
                TypeOfTile::Ground => {
                    if n.checked {
                        print!("{}", n.sign.to_string().on_green())
                    } else {
                        print!("{}", n.sign.to_string())
                    }
                },
            }
        }
        println!()
    }

}

#[derive(PartialEq)]
pub enum TypeOfTile {
    Side,
    Corner,
    Start,
    Ground,
}

impl From<char> for TypeOfTile {
    fn from(value: char) -> Self {
        match value {
            '|' | '-' => TypeOfTile::Side,
            'L' | 'J' | '7' | 'F' => TypeOfTile::Corner,
            'S' => TypeOfTile::Start,
            '.' => TypeOfTile::Ground,
            _ => panic!("Couldn't parse tile")
        }
    }
}

fn algo(lines: &mut Vec<Vec<Node>>, y: usize, x: usize) {
    let foo = lines[y][x].check_if_ground_and_mark();
}

fn find_next(lines: &mut Vec<Vec<Node>>, y: usize, x: usize) -> (usize, usize) {
    let ret_y = {
        if y > 0 {
            ret_y = y-1;
        }

    };
    let ret_x: usize;
    if x < lines[y].len() - 1 {
        ret_x = x+1;
    }
    return (ret_y, ret_x)

}

fn main() {
    // let lines = read_file_into_arrays("./src/input/day10/input.txt");
    // let lines = read_file_into_arrays("./src/input/day10/test_input_part_2.txt");
    let lines = read_file_into_arrays("./src/input/day10/test_input_part_2_b.txt");
    let mut lines = convert_chars_to_nodes(&lines);

    algo(&mut lines, 0, 0);

    ppp(&lines)


}
