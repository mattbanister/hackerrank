use std::io;

fn read_line() -> String {
    let mut reader = io::stdin();
    let mut input = String::new();
    reader.read_line(&mut input).ok().expect("error reading line");
    input
}

fn update_sticks(sticks: &Vec<i32>) -> Vec<i32> {
    let smallest = sticks
        .iter()
        .fold(std::i32::MAX, |acc, &x| {
            if x < acc {
                x
            } else {
                acc
            }
        });
    let next_sticks = sticks
        .iter()
        .map(|x| x - smallest)
        .filter(|&x| x > 0)
        .collect::<Vec<i32>>();
    next_sticks
}

fn main () {
    read_line();
    let mut sticks = read_line().trim().split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut num_sticks = sticks.len();
    while num_sticks > 0 {
        println!("{}", num_sticks);
        sticks = update_sticks(&sticks);
        num_sticks = sticks.len();
    }
}
