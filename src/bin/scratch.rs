fn main() {
    a()
}

fn a() {
    let mut foo = vec![1,2,3];
    foo.retain(|mut v| {
        true
    });
    println!("{:?}", foo)
}