use std::io;
use std::io::Read;
use std::collections::HashMap;

pub fn read_input() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).ok().expect("read error");
    let _numbers: Vec<String> = buffer
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    _numbers
}


pub fn get_char_frequency(line : &String) -> HashMap<char, i32> {
    let mut frequency: HashMap<char, i32> = HashMap::new();
    for c in line.chars() {
        *frequency.entry(c).or_insert(0) += 1;
    }
    frequency
}