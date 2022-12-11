use std::{env::args, fs::{read_to_string}};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
fn main() {
    let mut args = args();
    args.next();
    let day = args.next().unwrap();
    let file = read_to_string(args.next().unwrap()).unwrap();
    match day.as_ref() {
        "1.0" => println!("result: {}", day1::solve_part_one(file)),
        "1.1" => println!("result: {}", day1::solve_part_two(file)),
        "2.0" => println!("result: {}", day2::solve_part_one(file)),
        "2.1" => println!("result: {}", day2::solve_part_two(file)),
        "3.0" => println!("result: {}", day3::solve_part_one(file)),
        "3.1" => println!("result: {}", day3::solve_part_two(file)),
        "4.0" => println!("result: {}", day4::solve_part_one(file)),
        "4.1" => println!("result: {}", day4::solve_part_two(file)),
        "5.0" => println!("result: {}", day5::solve_part_one(file)),
        "5.1" => println!("result: {}", day5::solve_part_two(file)),
        "6.0" => println!("result: {}", day6::solve_part_one(file)),
        "6.1" => println!("result: {}", day6::solve_part_two(file)),
        "7.0" => println!("result: {}", day7::solve_part_two(file)),
        "7.1" => println!("result: {}", day7::solve_part_two(file)),
        _ => unimplemented!()
    };
}
