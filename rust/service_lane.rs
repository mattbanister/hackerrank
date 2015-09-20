use std::io;

fn read_line() -> String {
    let mut reader = io::stdin();
    let mut input = String::new();
    reader.read_line(&mut input).ok().expect("error reading line");
    input
}

fn main () {
    let num_tests = read_line().trim().split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .nth(1).unwrap();
    let widths = read_line().trim().split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    for _ in 0..num_tests {
        let line = read_line().trim().split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let ioline = &line;
        let wline = &widths;
        let start = ioline[0] as usize;
        let end = ioline[1] as usize;
        let min_width = &wline[start..end + 1]
            .iter()
            .fold(std::i32::MAX, |acc, &x| {
            if x < acc {
                x
            } else {
                acc
            }
        });
        println!("{}", min_width);
    }
}
