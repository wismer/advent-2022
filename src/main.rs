use std::{env::args, fs::{read_to_string}};

mod day1;
fn main() {
    let mut args = args();
    args.next();
    let day = args.next().unwrap();
    let file = read_to_string(args.next().unwrap()).unwrap();
    match day.as_ref() {
        "1.0" => day1::solve_part_one(file),
        "1.1" => day1::solve_part_two(file),
        _ => unimplemented!()
    };
}
