use std::io;

fn read_line() -> String {
    let mut reader = io::stdin();
    let mut input = String::new();
    reader.read_line(&mut input).ok().expect("error reading line");
    input
}

fn main () {
    let el = read_line().trim().parse::<i32>().unwrap();
    read_line();
    let input = read_line().trim().split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut list = input.iter();
    let mut i = 0;
    let mut res = -1;
    while let Some(x) = list.next() {
        if el == *x {
            res = i;
            break;
        }
        i += 1;
    }
    println!("{}", res);
}
