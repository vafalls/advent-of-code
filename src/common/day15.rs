pub fn run_hash_algorithm(step: &str) -> u32 {
    let mut res = 0;
    for i in step.chars() {
        res += i as u32;
        res = (res * 17) % 256;
    }
    res
}
