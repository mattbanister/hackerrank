use std::io;

fn read_line() -> String {
    let mut reader = io::stdin();
    let mut input = String::new();
    reader.read_line(&mut input).ok().expect("error reading line");
    input
}

fn main () {
    let num_tests = read_line().trim().parse::<i32>().unwrap();
    for _ in 0..num_tests {
        let num_cycles = read_line().trim().parse::<i32>().unwrap();
        let mut height = 1;
        for i in 0..num_cycles {
            if i % 2 == 0 {
                height = 2 * height
            } else {
                height = height + 1
            }
        }
        println!("{}", height);
    }
}
