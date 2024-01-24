use std::fmt::Display;

pub fn print_vec_of_vecs<I: Display>(lines: &Vec<Vec<I>>) {
    println!();
    for i in lines {
        for j in i {
            print!("{}", j)
        }
        println!()
    }
}
