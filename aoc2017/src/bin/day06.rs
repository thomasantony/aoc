use std::collections::HashSet;

/// Day 06 - Memory Reallocation
/// 
/// Memory blocks get "balanced" by taking elements from bank with most blocks and adding them
/// to others one at a time starting with the very next block.
/// 
/// Part 1
/// 
/// Find number of such redistributions before getting back one that was seen before (cycle detection).
/// Use hashset or hare-tortoise algorithm.
/// 

fn argmax<'a, T: std::cmp::Ord + Copy>(array: &'a Vec<T>) -> (usize, T)
{
    array.iter().cloned().enumerate().max_by_key(|(_, item)| *item).unwrap()
}
fn redistribute(memory: &Vec<i32>) -> Vec<i32>
{
    let mut output = memory.clone();

    let (max_block_idx, mut max_block_val)= argmax(&output);
    output[max_block_idx] = 0;

    let mut idx = max_block_idx;
    let num_vals = output.len();
    while max_block_val > 0
    {
        idx += 1;
        output[idx%num_vals] += 1;
        max_block_val -= 1;
    }

    output
}
fn main()
{
    let input = include_str!("../../inputs/day06.txt");

    let input = "0 2 7 0";

    let memory = input
        .trim()
        .split_ascii_whitespace()
        .map(str::trim)
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let mut cycle_counter = 0;
    let mut mem = memory.clone();

    let mut seen_before = HashSet::new();
    loop {
        let new_mem = redistribute(&mem);
        
        if new_mem == vec![2, 4, 1, 2]
        {
            println!("First seen at: {}", cycle_counter);
        }
        if seen_before.contains(&new_mem){
            println!("{:?}", new_mem);
            break;
        }
        seen_before.insert(new_mem.clone());
        cycle_counter += 1;
        mem = new_mem;
    }
    println!("Part 1: {}", cycle_counter);
}
