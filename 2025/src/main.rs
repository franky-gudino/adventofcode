mod days;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: &str = &args[1];
    let file_path = &args[2];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read file");

    match day {
        "1" => days::day_1::solve(contents),
        "2" => days::day_2::solve(contents),
        "3" => days::day_3::solve(contents),
        "4" => days::day_4::solve(contents),
        "5" => days::day_5::solve(contents),
        _ => println!("Day not supported"),
    }
}
