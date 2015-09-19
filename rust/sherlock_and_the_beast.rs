use std::io;

fn read_line() -> String {
    let mut reader = io::stdin();
    let mut input = String::new();
    reader.read_line(&mut input).ok().expect("error reading line");
    input
}

fn repeat(n: i32, x: i32) -> String {
    let mut res = String::new();
    for _ in 0..n {
        res = res + &x.to_string()
    }
    res
}

fn main () {
    let num_tests = read_line().trim().parse::<i32>().unwrap();
    for _ in 0..num_tests {
        let n = read_line().trim().parse::<i32>().unwrap();
        let descent_num =
            if n < 3 {
                "-1".to_string()
            } else if n % 3 == 0 {
                repeat(n, 5)
            } else {
                let mut num_fives = n;
                let mut i = 0;
                while num_fives % 3 != 0 && num_fives >= 0 {
                    num_fives -= 5;
                    i += 1;
                }
                if num_fives < 0 {
                    "-1".to_string()
                } else {
                    let num_threes = i * 5;
                    repeat(num_fives, 5) + &repeat(num_threes, 3)
                }
            };
        println!("{}", descent_num);
    }
}
