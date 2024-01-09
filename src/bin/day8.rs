use std::collections::HashMap;
use advent_of_code::{read_file_into_strings};
use std::iter::{Cycle, Iterator};
use std::str::FromStr;

enum Direction {
    L,
    R
}

struct Value {
    left: String,
    right: String
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let base = s.to_string().replace("(","").replace(")", "");
        let parts: Vec<&str> = base.split(",").collect();
        Ok(Value {
            left: parts[0].to_string(),
            right: parts[1].to_string(),
        })
    }
}

fn parse_instructions(instruction_str: &[String]) -> Vec<Direction> {
    let mut foo: Vec<Direction> = Vec::new();
    for i in instruction_str[0].chars() {
        match i {
            'L' => foo.push(Direction::L),
            'R' => foo.push(Direction::R),
            _ => panic!("Unknown instruction character")
        }
    }
    foo
}

fn parse_node_map(node_strings: &[String]) -> HashMap<String, Value> {
    let mut map: HashMap<String, Value> = HashMap::new();
    for line in node_strings.iter() {
        let one = line.replace(" ", "");
        let splits: Vec<&str> = one.split("=").collect();
        let key: String = splits.first().unwrap().parse().unwrap();
        let value = Value::from_str(splits.last().unwrap()).unwrap();
        map.insert(key, value);
    }
    map
}

fn main() {
    let lines = read_file_into_strings("./src/input/day8/input.txt");

    let groups_of_lines:Vec<_> = lines.split(|line| line.is_empty()).collect();

    let instructions = parse_instructions(*groups_of_lines.first().unwrap());
    let nodes = parse_node_map(*groups_of_lines.last().unwrap());

    let mut cycle = instructions.iter().cycle();

    let mut current_key = String::from("AAA");

    let mut steps = 0;

    loop {
        let value: &Value = nodes.get(&current_key).expect("Couldn't find node");
        match cycle.next().expect("Couldn't get next instruction") {
            Direction::L => {
                current_key = value.left.to_string();
            }
            Direction::R => {
                current_key = value.right.to_string();
            }
        }
        steps += 1;
        if current_key == "ZZZ" {
            break;
        }
    }
    println!("{}", steps)

}

