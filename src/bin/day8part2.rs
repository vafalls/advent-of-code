use std::collections::HashMap;
use advent_of_code::{read_file_into_strings};
use std::iter::{Cycle, Iterator};
use std::str::FromStr;
use core::slice::Iter;
use std::fmt::{Display, Formatter};
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Direction {
    L,
    R
}

impl Direction {
    fn follow_for(&self, value: &Value) -> String{
        match self {
            Direction::L => value.left.to_string(),
            Direction::R => value.right.to_string(),
        }
    }
}

struct Value {
    left: String,
    right: String,
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

struct TheZ<'a> {
    total_steps: u32,
    cycle: Cycle<Iter<'a, Direction>>,
    node: String,
    start_node: String
}

impl<'a> TheZ<'a> {
    fn init(
        directions: &'a Vec<Direction>,
        start_node: String
    ) -> TheZ<'a> {
        TheZ{ total_steps: 0, cycle: directions.iter().cycle(), node: start_node.clone(), start_node}
    }

    fn find_next(&mut self, nodes: &HashMap<String, Value>) {
        let mut count = 0;
        let mut current_node = self.node.clone();

        loop {
            count += 1;
            current_node = self.cycle.next().unwrap().follow_for(nodes.get(&current_node).unwrap());
            if current_node.chars().nth(2).unwrap() == 'Z' {
                break;
            }
        }
        self.total_steps += count;
        self.node = current_node;
    }
}

impl Display for TheZ<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TheZ ( start_node: {}, total_steps: {}, node: {} )", self.start_node, self.total_steps, self.node)
    }
}

fn main() {
    let lines = read_file_into_strings("./src/input/day8/input.txt");

    let groups_of_lines:Vec<_> = lines.split(|line| line.is_empty()).collect();

    let directions = parse_instructions(*groups_of_lines.first().unwrap());
    let nodes = parse_node_map(*groups_of_lines.last().unwrap());

    let current_nodes: Vec<String> = nodes.keys().filter(|&key| key.chars().nth(2).unwrap() == 'A').cloned().collect();

    let list_of_total_steps: Vec<u64> = current_nodes.iter()
        .map(|start_node| {
            let mut the_z = TheZ::init(&directions, start_node.clone());
            the_z.find_next(&nodes);
            the_z.total_steps as u64
        })
        .sorted()
        .rev()
        .collect();


    let largest_total_steps = list_of_total_steps.first().unwrap();

    'outer: for i in (*largest_total_steps..u64::MAX).step_by(*largest_total_steps as usize) {
        for n in &list_of_total_steps {
            if i%n != 0 {
                continue 'outer;
            }
        }
        println!("The result is: {}", i);
        break;
    }
}
