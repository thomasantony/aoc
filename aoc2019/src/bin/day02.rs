use ::aoc2019::*;

fn main() 
{
    let input = read_stdin();
    let data: Vec<i32> = parse_numbers_with_delimiter(&input, ',').collect();
    println!("data: {:?}", data);
}