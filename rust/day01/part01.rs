use std::io;
use std::io::Read;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).ok().expect("read error");
    let numbers: Vec<i32> = buffer
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut total = 0;
    for num in numbers {
        total += num;
    }
    println!("Total = {}", total);
}
