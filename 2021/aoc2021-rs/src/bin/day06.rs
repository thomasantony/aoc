/// Day 06 - Lanternfish
/// 
/// Simulate the growth in number of lanternfish given a "starting state"
/// 
/// Each "timer", crossing zero results in new fish
/// After corssing zero, the timer resets to 6 (as zero is a valid value)
/// 
/// Simulate number of fish after 80 days for given input
/// 
/// It seems like each distinct value of starting timer results in a 
/// certain number of fish. So it is enough to compute it once for each value
/// and then sum up the count for each instance in input.
/// 
/// Thought process: This process is Markovian and only depends on previous state.
/// Use memoization to speed up (particularly for part 2).
/// Verify results from some specific starting cases to fix off-by-one bugs
/// e.g. starting state of 3 and 8 as shown below.
/// 
/// 3 -> n0
/// 2
/// 1
/// 0
/// 6 8  <- day 4 (1 offspring)
/// 5 7
/// 4 6
/// 3 5
/// 2 4
/// 1 3
/// 0 2
/// 6 1 8 <- day 11 (2 offspring)
/// 5 0 7
/// 4 6 6 8
/// 3 5 5 7
/// 2 4 4 6 
/// 1 3 3 5
/// 0 2 2 4
/// 6 1 1 3 8 <- day 18 (4 offspring)
/// 5 0 0 2 7
/// 4 6 6 1 6 8
/// 
/// 8 -> n0
/// 7
/// 6
/// 5
/// 4
/// 3
/// 2
/// 1
/// 0
/// 6 8 -> day 9 (1 offspring)
/// 5 7
/// 4 6
/// 3 5
/// 2 4
/// 1 3
/// 0 2   -> day 15
/// 6 1 8 -> day 16 (2 offspring)
/// 5 0 7
/// 4 6 6 8 -> day 18 (3 offpsring)

use cached::proc_macro::cached;
#[cached]
fn compute_offspring_count(n0: i64, sim_time: i64) -> i64
{
    // Computes number of offspring for given starting state and sim time
    // total given is at the *end of* given sim time
    // Time from first doubling
    let children_sim_time = sim_time - n0 - 1;
    if children_sim_time < 0
    {
        return 0;
    }
    
    // Add one for initial offspring from n0->0
    let direct_offspring = (children_sim_time as f32/7.0).floor() as i64 + 1;
    // let number_of_new_fish =  number_children(n0, sim_time);
    
    let mut total = direct_offspring as i64;
    for i in 0..direct_offspring
    {
        // Compute total for each child fish starting with state 8
        total += compute_offspring_count(8, children_sim_time - i*7);
    }
    total
}
fn compute_total_fish(initial_state: &Vec<i64>, time: i64) -> i64
{
    initial_state.iter().map(|s| compute_offspring_count(*s, time)).sum::<i64>()
        + initial_state.len() as i64
}
fn main()
{
    // Some test cases that were checked manually
    assert!(compute_offspring_count(8, 8) == 0);
    assert!(compute_offspring_count(8, 9) == 1);
    assert!(compute_offspring_count(8, 15) == 1);
    assert!(compute_offspring_count(8, 16) == 2);
    assert!(compute_offspring_count(8, 19) == 3);

    assert!(compute_offspring_count(3, 4) == 1);
    assert!(compute_offspring_count(3, 11) == 2);
    assert!(compute_offspring_count(3, 18) == 4);

    let demo_states = vec![3,4,3,1,2];
    assert_eq!(compute_total_fish(&demo_states, 18), 26);
    assert_eq!(compute_total_fish(&demo_states, 80), 5934);

    let input_str = include_str!("../../../inputs/day06.txt");
    let states = input_str.trim()
                                    .split(',')
                                    .map(str::parse::<i64>)
                                    .map(Result::unwrap)
                                    .collect::<Vec<_>>();

    let part01_sol = compute_total_fish(&states, 80);
    println!("Part 1: {}", part01_sol);

    // Test case for part 2
    assert_eq!(compute_total_fish(&demo_states, 256), 26984457539);
    let part02_sol = compute_total_fish(&states, 256);
    println!("Part 2: {}", part02_sol);
}
