use ::aoc2019::intcode::*;
use ::aoc2019::{parse_numbers_with_delimiter, read_stdin};

fn main()
{
    let input = read_stdin();
    let data: Vec<i64> = parse_numbers_with_delimiter(&input, ',').collect();

    let mut vm = IntComputer::new();
    vm.set_ram_size(2048);
    let output_a = vm.load_program(&data).push_input(1).execute();
    println!("Part A: {}", output_a[0]);

    // let mut vm = IntComputer::new();
    vm.reset();
    let output_b = vm.load_program(&data).push_input(2).execute();
    println!("Part B: {}", output_b[0]);
}