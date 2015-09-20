use std::io;

fn read_line() -> String {
    let mut reader = io::stdin();
    let mut input = String::new();
    reader.read_line(&mut input).ok().expect("error reading line");
    input
}

fn print_vec_as_line(vec: &Vec<i32>) {
    let mut i = 1;
    for x in vec {
        if i == vec.len() {
            print!("{}", x);
        } else {
            print!("{} ", x);
        }
        i += 1;
    }
    print!("\n");
}

fn main () {
    read_line();
    let mut vec = read_line().trim().split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut pos = (vec.len() - 1) as usize;
    let unsorted_el = vec[pos];
    while pos != 0 &&  unsorted_el < vec[(pos - 1)] {
        vec[pos] = vec[(pos - 1)];
        pos -= 1;
        print_vec_as_line(&vec);
    }
    vec[pos] = unsorted_el;
    print_vec_as_line(&vec);
}
