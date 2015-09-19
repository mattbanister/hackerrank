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
        let n = read_line();
        let n_int = n.trim().parse::<i64>().unwrap();
        let count = n.trim().split("")
            .filter(|x| x.len() > 0)
            .map(|x: &str| x.parse::<i64>().unwrap() )
            .fold(0, |acc, x|
                  if x == 0 {
                      acc
                  } else if n_int % x == 0 {
                      acc + 1
                  } else {
                      acc
                  });
        println!("{}", count);
    }
}
