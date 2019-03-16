use std::io;
use std::io::Read;

pub fn read_input() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).ok().expect("read error");
    let _numbers: Vec<String> = buffer
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    _numbers
}
