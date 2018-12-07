use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <day> <part>", env::args().nth(0).unwrap());
        process::exit(1);
    }
    let day: i32 = env::args().nth(1).unwrap().parse().unwrap();
    let part: i32 = env::args().nth(2).unwrap().parse().unwrap();
    if part < 1 || part > 2 {
        eprintln!("Part must be 1 or 2");
        std::process::exit(1);
    }

    let filename = format!("input/day{}.txt", day);
    let mut f = File::open(filename).unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    match day {
        1 => day1::run(part, &input),
        2 => day2::run(part, &input),
        3 => day3::run(part, &input),
        4 => day4::run(part, &input),
        5 => day5::run(part, &input),
        7 => day7::run(part, &input),
        _ => eprintln!("Day {} not implemented", day)
    };
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day7;
