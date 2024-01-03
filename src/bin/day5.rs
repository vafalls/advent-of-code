use std::str::FromStr;
use advent_of_code::{read_file_into_strings};


struct ConversionEntry {
    source_start: u64,
    range_length: u64,
    source_to_dest_diff: i64
}

impl FromStr for ConversionEntry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lst = s.split(" ").collect::<Vec<_>>();
        let destination_start = lst[0].parse::<u64>().expect("Couldn't parse destination start");
        let source_start = lst[1].parse::<u64>().expect("Couldn't parse source start");
        Ok(Self {
            source_start,
            range_length: lst[2].parse::<u64>().expect("Couldn't parse range length"),
            source_to_dest_diff: destination_start as i64 - source_start as i64
        })
    }
}

impl ConversionEntry {
    fn apply(&self, var: &mut u64) -> bool {
        if *var >= self.source_start && *var < self.source_start + self.range_length {
            if let Some(res) = var.checked_add_signed(self.source_to_dest_diff) {
                *var = res;
                return true;
            }
            panic!("what the hell!")
        }
        false
    }
}

fn get_conversion_entries(groups: Vec<&[String]>) -> Vec<Vec<ConversionEntry>> {
    let mut new_groups: Vec<Vec<ConversionEntry>> = Vec::new();

    for group in groups[1..].iter() {
        let mut group_vector = Vec::new();
        for _str in group[1..].iter() {
            group_vector.push(ConversionEntry::from_str(_str).unwrap())
        }
        new_groups.push(group_vector)
    }
    return new_groups
}

fn main() {
    let lines = read_file_into_strings("./src/input/day5/input.txt");
    let groups_of_lines:Vec<_> = lines.split(|line| line.is_empty()).collect();

    let mut seeds: Vec<_> = groups_of_lines[0].first().unwrap().split(":").last().unwrap().trim()
        .split(" ").map(|x| x.parse::<u64>().unwrap()).collect();


    let ce_groups = get_conversion_entries(groups_of_lines);

    for mut seed in &mut seeds {
        for ce_group in &ce_groups {
            '_group_loop: for ce in ce_group {
                if ce.apply(&mut seed) {
                    break '_group_loop;
                }
            }
        }
    }
    seeds.sort();

    println!("{:?}", seeds[0])
}
