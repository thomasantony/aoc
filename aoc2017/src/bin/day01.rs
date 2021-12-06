/// Day 01 - Inverse Captcha
/// 
/// Find the numbers that have the same number right after it (with loop-back at end of list). 
/// Part 1 - Sum up the numbers
/// 
/// Concatenate first element at end of list. Loop over pairs of elements and add up the ones that match.
/// 
/// Part 2 - Halfway around circle
/// 
/// check if digit == digit "halfway around circle",
/// index of digit halfway around = (i+len/2)%len

fn main()
{
    let input = include_str!("../../inputs/day01.txt");
    let input = input.trim().chars().collect::<Vec<_>>();

    // Part 1
    
    let input_p1 = [input.as_slice(), &[input[0]]].concat();

    let part1_sol:i32 = input_p1.windows(2)
        .filter(|c| c[0] == c[1])
        .map(|c| c[0] as i32 - 0x30)
        .sum();
    
    println!("Part 1 : {}", part1_sol);

    // Part 2
    let n = input.len();
    let part2_sol:i32 = (0..n).map(|i| (i, (i+n/2)%n))  // Create index halfway around circle
                            .filter_map(|(i, j)|{
                                if input[i] == input[j]
                                {
                                    Some(input[i])
                                }else{
                                    None
                                }
                            }).map(|c| c as i32 - 0x30)
                            .sum();
    println!("Part 2 : {}", part2_sol);
}
