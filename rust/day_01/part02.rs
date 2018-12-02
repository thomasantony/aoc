use std::io;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut buffer = String::new();
    let mut freq_map = HashMap::new();

    // Read numbers
    io::stdin().read_to_string(&mut buffer).ok().expect("read error");
    let numbers: Vec<i32> = buffer
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Sum the numbers
    let mut total = 0;
    for num in &numbers {
        total += num;
        freq_map.insert(total, 1);
    }

    // Find repeated sum
    for num in numbers.iter().cycle()
    {
        total += num;
        let _res = match freq_map.get(&total) {
            Some(_val) => true,
            None => false
        };
        if _res {
            break;
        }
    }
    println!("Total = {}", total);
}
