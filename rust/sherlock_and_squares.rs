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
        let line = read_line();
        let parse_line: Vec<f64> = line.trim().split(" ")
            .map(|x| x.parse::<f64>().unwrap())
            .collect();
        let lower_bound = parse_line[0];
        let upper_bound = parse_line[1];
        let lb_sqrt: i32 = lower_bound.sqrt() as i32;
        let ub_sqrt: i32 = upper_bound.sqrt() as i32 + 1;

        let num_perfect_squares: usize =
            (lb_sqrt..ub_sqrt)
            .filter(|&x| x * x >= lower_bound as i32 && x * x <= upper_bound as i32)
            .count();

        println!("{}", num_perfect_squares);
    }
}
