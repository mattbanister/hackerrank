use std::io;

fn main () {
    let mut reader = io::stdin();
    let mut input = String::new();
    reader.read_line(&mut input).ok().expect("Failed to read line.");
    let input_slice = *&input.trim();
    let std_time: Vec<i32> = input_slice[0..8]
        .split(":")
        .map(|x: &str| x.parse::<i32>().unwrap())
        .collect();
    let meridian: Option<char> = input_slice.chars().nth(8);
    let result = match meridian {
        Some(c) => match c {
            'A' => {
                let am_hour = match std_time[0] {
                    12 => 0,
                    _ => std_time[0]
                };
                Some([am_hour, std_time[1], std_time[2]])
            },
            'P' => {
                let pm_hour = match std_time[0] {
                    12 => 12,
                    _ => std_time[0] + 12
                };
                Some([pm_hour, std_time[1], std_time[2]])
            },
            _ => None
        },
        None => None
    };
    let parsed_time = match result {
        Some(t) => format!("{hour:00$}:{min:00$}:{sec:00$}", 2, hour=t[0], min=t[1], sec=t[2]),
        None => "Error parsing time".to_string()
    };
    println!("{}", parsed_time);
}
