use std::ops::Range;
use std::str::FromStr;
use itertools::enumerate;
use advent_of_code::{read_file_into_strings};


struct ConversionEntry {
    destination_start: u64,
    destination_end: u64,
    modifier: i64
}

impl FromStr for ConversionEntry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lst = s.split(" ").collect::<Vec<_>>();
        let destination_start = lst[0].parse::<u64>().expect("Couldn't parse destination start");
        let source_start = lst[1].parse::<u64>().expect("Couldn't parse source start");
        let range_length = lst[2].parse::<u64>().expect("Couldn't parse range length");
        Ok(Self {
            destination_start,
            destination_end: destination_start+range_length,
            modifier: source_start as i64 - destination_start as i64
        })
    }
}

impl ConversionEntry {
    fn apply(&self, value: &mut u64) -> bool {
        if *value >= self.destination_start && *value < self.destination_end {
            if let Some(res) = value.checked_add_signed(self.modifier){
                *value = res;
                return true
            }
            panic!("What the hell?")
        }
        false
    }
}

fn get_conversion_entries(groups_of_lines: Vec<&[String]>) -> Vec<Vec<ConversionEntry>> {
    let mut new_groups: Vec<Vec<ConversionEntry>> = Vec::new();

    for group in groups_of_lines[1..].iter() {
        let mut group_of_conversion_entries = Vec::new();
        for mapping_line in group[1..].iter() {
            group_of_conversion_entries.push(ConversionEntry::from_str(mapping_line).unwrap());
        }
        new_groups.push(group_of_conversion_entries)
    }
    return new_groups
}

fn get_seeds(seeds: &Vec<u64>) -> Vec<Range<u64>> {
    let mut list_of_seed_ranges:Vec<Range<u64>> = Vec::new();

    for (idx, seed) in enumerate(seeds) {
        if idx%2 == 0 {
            continue
        }
        list_of_seed_ranges.push(seeds[idx-1]..seeds[idx-1]+seed);
    }
    return list_of_seed_ranges
}

fn apply_all_conversion_entries(ce_matrix: &Vec<Vec<ConversionEntry>>, value: &mut u64) {
    for conversion_entry_group in ce_matrix.iter().rev() {
        'outer: for conversion_entry in conversion_entry_group {
            if conversion_entry.apply(value) {
                break 'outer;
            }

        }
    }

}

fn is_within_any_seed_range(seed_ranges: &Vec<Range<u64>>, value: u64) -> bool {
    for seed_range in seed_ranges {
        if seed_range.contains(&value) {
            return true;
        }
    }
    false
}

fn main() {
    let lines = read_file_into_strings("./src/input/day5/input.txt");
    let groups_of_lines:Vec<_> = lines.split(|line| line.is_empty()).collect();

    let seedlings: Vec<_> = groups_of_lines[0].first().unwrap().split(":").last().unwrap().trim()
        .split(" ").map(|x| x.parse::<u64>().unwrap()).collect();

    let seed_ranges:Vec<Range<u64>> = get_seeds(&seedlings);

    let ce_matrix:Vec<Vec<ConversionEntry>> = get_conversion_entries(groups_of_lines);

    for location in 0..u64::MAX {
        let mut tmp = location;
        apply_all_conversion_entries(&ce_matrix, &mut tmp);

        if is_within_any_seed_range(&seed_ranges, tmp) {
            println!("location: {}, seed: {}", location, tmp);
            break
        }
    }
}
