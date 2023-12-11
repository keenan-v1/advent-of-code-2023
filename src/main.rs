use std::env;

extern crate day1;
extern crate day2;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1].parse::<usize>().unwrap();
    println!("Running day {}", day);
    match day {
        1 => day1::day1(),
        2 => day2::day2(),
        _ => println!("Invalid day"),
    }
}
