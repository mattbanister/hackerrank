use std::io;

fn main () {
    let mut reader = io::stdin();
    let mut input = String::new();
    reader.read_line(&mut input).ok().expect("Failed to read line.");
    let num_stairs = *&input.trim().parse::<i32>().unwrap();
    for i in 0..num_stairs {
        for j in 0..num_stairs {
            if i >= (num_stairs - 1 - j) {
                print!("{}", "#");
            } else {
                print!("{}", " ");
            }
            if j == num_stairs - 1 {
                println!("{}", "");
            }
        }
    }
}
