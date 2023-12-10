use std::env;

extern crate day1;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let input_url = &args[1];
    let day = input_url.split("/").nth(5).unwrap();
    let response = reqwest::get(input_url).await.unwrap();
    let input = response.text().await.unwrap();
    match day {
        "1" => day1::day1(input),
        _ => println!("Invalid day"),
    }

    println!("Hello, world!");
}
