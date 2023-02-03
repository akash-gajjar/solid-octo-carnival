use std::io;

fn main() {
    println!("Hello, world!");
    let mut buf = String::new();

    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");

    println!("Sky {buf}");
}
