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
        let k = read_line()
            .trim()
            .split(" ")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let on_time_student_count = read_line().trim().split(" ")
            .map(|x: &str| x.parse::<i32>().unwrap())
            .filter(|x| x <= &0)
            .count();
        if on_time_student_count  >= k {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
