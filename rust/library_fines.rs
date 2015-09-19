use std::io;

struct Date {
    year: i32,
    month: i32,
    day: i32
}

impl Date {
    fn is_after(&self, that: &Date) -> bool {
        let result =
            if self.year > that.year {
                true
            } else if self.month > that.month && self.year == that.year {
                true
            } else if self.day > that.day && self.month == that.month
                                          && self.year == that.year {
                true
            } else {
                false
            };
        result
    }
}

fn parse_date(date: &String) -> Date {
    let result: Vec<i32> = date
        .trim()
        .split(" ")
        .map(|x: &str| x.parse::<i32>().unwrap())
        .collect();
    Date {
        year: result[2],
        month: result[1],
        day: result[0]
    }
}

fn main () {
    let mut reader = io::stdin();
    let mut return_date = String::new();
    let mut due_date = String::new();
    reader.read_line(&mut return_date).ok().expect("Failed to read line.");
    reader.read_line(&mut due_date).ok().expect("Failed to read line.");
    let ref return_date_parsed = parse_date(&return_date);
    let ref due_date_parsed = parse_date(&due_date);
    let is_late = return_date_parsed.is_after(&due_date_parsed);
    if !is_late {
        println!("{}", 0);
    } else if due_date_parsed.year == return_date_parsed.year &&
               due_date_parsed.month == return_date_parsed.month {
        println!("{}", (return_date_parsed.day - due_date_parsed.day) * 15);
    } else if due_date_parsed.year == return_date_parsed.year {
        println!("{}", (return_date_parsed.month - due_date_parsed.month) * 500);
    } else {
        println!("{}", 10000);
    }
}
