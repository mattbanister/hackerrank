use std::io;

fn main() {
    let mut reader = io::stdin();
    let mut input_text = String::new();
    reader.read_line(&mut input_text).ok().expect("failed to read line");
    let input: Option<i32> = input_text.trim().parse::<i32>().ok();
    let count = input.unwrap();
    let mut sum1 = 0;
    let mut sum2 = 0;
    for i in 0..count {
        let mut line =  String::new();
        reader.read_line(&mut line).ok().expect("failed to read line");
        let line_slice: &str = &*line;
        let v: Vec<&str> = line_slice
            .trim()
            .split(' ')
            .collect();
        sum1 += v[i as usize].parse::<i32>().unwrap();
        sum2 += v[((count - 1) - i) as usize].parse::<i32>().unwrap();
    }
    let result = (sum1 - sum2).abs();
    println!("{}", result);
 }
