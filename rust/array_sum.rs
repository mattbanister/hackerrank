use std::io;


fn main() {
    let mut reader = io::stdin();
    let mut input_text = String::new();
    reader.read_line(&mut input_text).ok().expect("failed to read line");
    // let's just read this line and then forget about it
    // let input: Option<i32> = input_text.trim().parse::<i32>().ok();
    let mut array_text = String::new();
    reader.read_line(&mut array_text).ok().expect("failed to read line");
    let array_text_slice = &*array_text;
    let sum = array_text_slice
        .trim()
        .split(' ')
        .map(|x: &str| -> u32 {x.parse().unwrap()})
        .fold(0, |acc, el| el + acc);
    println!("{}", sum);
}
