use std::fmt::Debug;
use std::str::FromStr;
pub fn parse_with_comma<T: FromStr>(input: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    input
        .trim()
        .split(',')
        .map(str::parse::<T>)
        .map(Result::unwrap)
        .collect::<Vec<_>>()
}

pub fn parse_numbers(input: &str) -> Vec<i32> {
    input
        .trim()
        .split_ascii_whitespace()
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<_>>()
}
