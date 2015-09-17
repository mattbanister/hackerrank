use std::io;

fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn main() {
    let mut reader = io::stdin();
    let mut input_text = String::new();
    reader.read_line(&mut input_text).ok().expect("failed to read line");
    let input: Option<i32> = input_text.trim().parse::<i32>().ok();
    for _ in 0..input.unwrap() {
        let mut line =  String::new();
        reader.read_line(&mut line).ok().expect("failed to read line");
        let line_slice: &str = &*line;
        let v: Vec<&str> = line_slice.trim().split(' ').collect();
        let x = v[0].parse().unwrap();
        let y = v[1].parse().unwrap();
        println!("{}", add(x, y));
    }
    println!("{}", input.unwrap());
}
