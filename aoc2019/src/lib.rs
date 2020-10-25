extern crate anyhow;

use std::io;
use std::io::Read;
pub mod intcode;

pub fn read_stdin() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .ok()
        .expect("read error");
    buffer
}

pub fn get_lines<'a>(input: &'a String) -> impl Iterator<Item = &'a str> {
    input.split_ascii_whitespace()
}

pub fn parse_numbers<'a>(input: &'a String) -> impl Iterator<Item = i64> + '_ {
    parse_numbers_with_delimiter(input, '\n')
}

pub fn parse_numbers_with_delimiter<'a>(
    input: &'a String,
    delim: char,
) -> impl Iterator<Item = i64> + '_ {
    let lines = input.trim().split(delim).map(|s| s.parse().unwrap());
    lines
}

pub fn parse_digits<'a>(input: &'a str) -> impl Iterator<Item = u32> + '_
{
    input.trim().chars().map(|c| c.to_digit(10).unwrap())
}
pub mod graph;
pub mod tree;
pub mod vec3;