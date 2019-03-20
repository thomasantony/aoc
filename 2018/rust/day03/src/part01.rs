extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate itertools;

use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::Read;

fn parse_line(line: &str) -> (u32, u32, u32, u32) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").expect("Error initializing regex!");
    }
    let cap = RE.captures(line).expect("Error parsing line");

    (
        cap[2]
            .to_string()
            .parse()
            .expect("Error parsing left coordinate"),
        cap[3]
            .to_string()
            .parse()
            .expect("Error parsing top coordinate"),
        cap[4].to_string().parse().expect("Error parsing width"),
        cap[5].to_string().parse().expect("Error parsing height"),
    )
}

pub fn read_input() -> HashMap<(u32, u32), u32> {
    let mut buffer = String::new();
    let mut counter: HashMap<(u32, u32), u32> = HashMap::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .ok()
        .expect("read error");
    let points = buffer
        .lines()
        .map(parse_line)
        .map(|(l, t, w, h)| iproduct!(l..(l + w), t..(t + h)))
        .flatten();

    for pt in points {
        *counter.entry(pt).or_insert(0) += 1;
    }
    counter
}
fn main() {
    let freq_map = read_input();
    let answer: usize = freq_map.values().filter(|v| *v >= &2).count();
    println!("Answer: {}", answer);
}
