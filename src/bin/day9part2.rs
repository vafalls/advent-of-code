use advent_of_code::read_file_into_strings;

fn convert_strings_to_lists_of_ints(lines: Vec<String>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        res.push(line.split(" ").map(|num| num.parse::<i32>().unwrap()).collect())
    }
    res
}

fn calculate_sequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    res.push(nums);

    loop {
        let mut tmp: Vec<i32> = Vec::new();
        let last_vec: &Vec<i32> = res.last().unwrap();
        for i in 1..last_vec.len() {
            tmp.push(last_vec[i]-last_vec[i-1])
        }
        let all_zeroes = tmp.iter().all(|&num| num == 0);
        res.push(tmp);
        if all_zeroes {
            break;
        }
    }
    res
}

fn extrapolate_history(sequences: &mut Vec<Vec<i32>>) -> i32 {
    sequences.last_mut().unwrap().push(0);

    for i in (0..sequences.len() - 1).rev() {
        let bellow = sequences[i + 1].first().unwrap().clone();
        let right = sequences[i][0];
        let mut tmp = vec![right - bellow];
        tmp.append(&mut sequences[i]);
        sequences[i] = tmp;
    }
    sequences[0].first().unwrap().clone()
}

fn main() {
    let lines: Vec<String> = read_file_into_strings("./src/input/day9/input.txt");
    let lines: Vec<Vec<i32>> = convert_strings_to_lists_of_ints(lines);

    let mut res = 0;
    for line in lines {
        let mut sequences: Vec<Vec<i32>> = calculate_sequences(line);
        res += extrapolate_history(&mut sequences);
    }
    println!("{}", res)
}