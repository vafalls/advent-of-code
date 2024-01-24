use itertools::enumerate;

pub struct Galaxy {
    pub x: usize,
    pub y: usize,
}

pub struct GalaxyPair<'a> {
    pub first: &'a Galaxy,
    pub second: &'a Galaxy,
    pub diff: u32
}

pub fn get_galaxies(lines: &Vec<Vec<char>>) -> Vec<Galaxy> {
    let mut res: Vec<Galaxy> = Vec::new();
    for (y_idx, y) in enumerate(lines) {
        for (x_idx, x) in enumerate(y) {
            if *x == '#' {
                res.push(Galaxy {x: x_idx, y: y_idx});
            }
        }
    }
    res
}

pub fn get_pairs(galaxies: &Vec<Galaxy>) -> Vec<GalaxyPair> {
    let mut res: Vec<GalaxyPair> = Vec::new();
    for (idx, _) in enumerate(galaxies) {
        for i in idx+1..galaxies.len() {
            res.push(GalaxyPair{ first: &galaxies[idx], second: &galaxies[i], diff: 0})
        }
    }
    res
}
