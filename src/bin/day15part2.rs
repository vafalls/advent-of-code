use itertools::enumerate;
use regex::Regex;
use advent_of_code::common::day15::run_hash_algorithm;
use advent_of_code::read_file_into_strings;


#[derive(Debug, Clone)]
struct Lens {
    focal_length: u8,
    label: String
}

fn handle_step(step: &str, regex: &Regex, boxes: &mut Vec<Vec<Lens>>) {
    let captures = regex.captures(step).unwrap();
    let box_idx = run_hash_algorithm(&captures["letters"]) as usize;
    let label = &captures["letters"];
    match &captures["operation"] {
        "=" => {
            let new_focal_length: u8 = captures["number"].parse::<u8>().unwrap();
            let mut element_updated = false;
            boxes[box_idx] = boxes[box_idx].iter().cloned().map(|mut lens| {
                if lens.label == label {
                    lens.focal_length = new_focal_length;
                    element_updated = true;
                }
                lens
            }).collect();
            if !element_updated {
                boxes[box_idx].push(
                    Lens{
                        focal_length: new_focal_length,
                        label: label.to_string()
                    }
                )
            }
        },
        "-" => {
            boxes[box_idx] = boxes[box_idx]
                .iter()
                .cloned()
                .filter(|step| step.label != label)
                .collect()
        },
        &_ => {}
    }
}

fn main() {
    let line = read_file_into_strings("./src/input/day15/input.txt")[0].clone();
    let re = Regex::new(r"^(?<letters>[a-z]+)(?<operation>[-=])(?<number>[0-9]*)").unwrap();

    let mut boxes: Vec<Vec<Lens>> = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new());
    }

    line.split(',').for_each(|step| {
        handle_step(step, &re, &mut boxes);
    });

    let mut res = 0;
    for (box_idx, _box) in enumerate(boxes) {
        for (lens_idx, lens) in enumerate(_box) {
            res += (1 + box_idx) * (1 + lens_idx) * lens.focal_length as usize;
        }
    }
    println!("{}", res)

}