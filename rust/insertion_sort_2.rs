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

fn sort_item_at_index(vec: &mut Vec<i32>, curr: usize) -> Vec<i32> {
    let unsorted_el = vec[curr];
    let mut pos = curr;
    while pos != 0 &&  unsorted_el < vec[(pos - 1)] {
        vec[pos] = vec[(pos - 1)];
        pos -= 1;
    }
    vec[pos] = unsorted_el;
    vec.to_owned()
}

fn main () {
    read_line();
    let vec = read_line().trim().split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut mut_vec = vec.clone();
    for (i, _) in vec.iter().enumerate() {
        if i > 0 {
            mut_vec = sort_item_at_index(&mut mut_vec, i);
            print_vec_as_line(&mut_vec);
        }
    }
}
