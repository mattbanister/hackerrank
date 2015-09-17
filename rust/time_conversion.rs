use std::io;

fn main () {
    let mut reader = io::stdin();
    let mut input = String::new();
    reader.read_line(&mut input).ok().expect("Failed to read line.");
    ref mut parsed: Vec<&str> = *&input.trim().split(":").collect();
    println!("{:?}", parsed);

}
