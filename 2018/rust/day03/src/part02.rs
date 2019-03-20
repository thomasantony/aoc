extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate itertools;

use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::Read;

fn parse_line(line: &str) -> (u32, u32, u32, u32, u32) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").expect("Error initializing regex!");
    }
    let cap = RE.captures(line).expect("Error parsing line");

    (
        cap[1]
            .to_string()
            .parse()
            .expect("Error parsing ID"),
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

pub fn read_input() -> u32 {
    let mut buffer = String::new();
    let mut counter: HashMap<(u32, u32), u32> = HashMap::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .ok()
        .expect("read error");
    let claims: Vec<(u32, u32, u32, u32, u32)> = buffer
        .lines()
        .map(parse_line).collect();

    for (_, l, t, w, h) in &claims
    {
        for point in iproduct!(*l..(l + w), *t..(t + h))
        {
            *counter.entry(point).or_insert(0) += 1;
        }
    }   
    let answer = claims.iter()
                          .filter(|(_id, l, t, w, h)| {
                              iproduct!((*l..(l+w)), (*t..(t+h))).all(|pt| {
                                  *counter.entry(pt).or_insert(0) == 1
                              })
                          })
                          .next().expect("No answer found");
    answer.0
}
fn main() {
    let answer = read_input();
    println!("Answer: {}", answer);
}
