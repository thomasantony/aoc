/// Day 07 - The Treachery of Whales
///
/// Give a list of horizontal positions, find the position that minimizes the fuelance from every other point
///
/// min_x sum |x_i - x|
///
/// Part 1
/// Brute force it
///
/// Part 2
///
/// Fuel use increases with distance, 1 for first step, 2 for second step etc.
/// So fuel use for distance d is d*(d+1)/2
///
/// Some more insight:
/// Part 1 is the median. I guess I was close with looking at average which minimizes L2-norm. The median minimizes the L1-norm.
/// Part 2 is apparently (almost) the mean though I am not sure how this is derived as of now.
///
/// min sum (|x_i - x|)*(|x_i-x| + 1)/2
fn main() {
    let input = include_str!("../../../inputs/day07.txt");
    let mut numbers = aoc2021_rs::parse_with_comma::<i32>(input);
    numbers.sort();

    let a = numbers[0].clone();
    let b = numbers[numbers.len() - 1].clone();

    let (_, part1_sol) = (a..b)
        .map(|i| (i, numbers.iter().map(|n| (n - i).abs()).sum::<i32>()))
        .min_by_key(|(_, fuel)| *fuel)
        .unwrap();

    println!("Part 1: {}", part1_sol);

    let (_, part2_sol) = (a..b)
        .map(|i| {
            let fuel = numbers
                .iter()
                .map(|n| {
                    let d = (n - i).abs();
                    d * (d + 1) / 2
                })
                .sum::<i32>();
            (i, fuel)
        })
        .min_by_key(|(_, fuel)| *fuel)
        .unwrap();
    println!("Part 2: {}", part2_sol);
}
