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
    let result = array_text_slice
        .trim()
        .split(' ')
        .map(|x: &str| -> i32 {x.parse().unwrap()})
        .fold([0, 0, 0, 0], |acc, el| match el {
            0 => [acc[0], acc[1], acc[2] + 1, acc[3] + 1],
            _ if el < 0 => [acc[0], acc[1] + 1, acc[2], acc[3] + 1],
            _ => [acc[0] + 1, acc[1], acc[2], acc[3] + 1]
        });
    let pos_count = result[0] as f32 / result[3] as f32;
    let neg_count = result[1] as f32 / result[3] as f32;
    let zero_count  = result[2] as f32 / result[3] as f32;
    println!("{}", pos_count);
    println!("{}", neg_count);
    println!("{}", zero_count);
}
