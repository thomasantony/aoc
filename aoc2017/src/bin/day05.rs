/// Day 05 - A Maze of Twisty Trampolines, All Alike
/// 
/// Part 1
/// Find number of instruction cycles before exiting given instruction block
/// 
/// Part 2
/// Same as part 1 but slightly different jumping rules
/// 
fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .split_ascii_whitespace()
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<_>>()
}
fn solve_part01(input: &str) -> i32 {
    let mut jumps = parse_input(input);

    let mut idx = 0i32;
    let mut counter = 0;
    while idx >= 0 {
        if let Some(jump_dist) = jumps.get(idx as usize).cloned() {
            jumps[idx as usize] += 1;
            idx = idx + jump_dist;
        } else {
            break;
        }
        counter += 1;
    }
    counter
}
fn solve_part02(input: &str) -> i32 {
    let mut jumps = parse_input(input);

    let mut idx = 0i32;
    let mut counter = 0;
    while idx >= 0 {
        if let Some(jump_dist) = jumps.get(idx as usize).cloned() {
            if jump_dist >= 3 {
                jumps[idx as usize] -= 1;
            } else {
                jumps[idx as usize] += 1;
            }

            idx = idx + jump_dist;
        } else {
            break;
        }
        counter += 1;
    }
    counter
}
fn main() {
    let input = include_str!("../../inputs/day05.txt");

    let part01_sol = solve_part01(input);
    println!("Part 1: {}", part01_sol);

    let part02_sol = solve_part02(input);
    println!("Part 1: {}", part02_sol);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_01() {
        assert_eq!(solve_part01("0 3 0 1 -3"), 5);
    }
    #[test]
    fn test_part_02() {
        assert_eq!(solve_part02("0 3 0 1 -3"), 10);
    }
}
