/// Day 02 - Corruption Checksum
/// 
/// Part 1
/// 
/// Find checksum of "spreadsheet". Find difference between max and min value in each row.
/// Cksum is the sum of all these differences.
/// 
/// Part 2
/// Find the only two "evenly divisible" numbers in each row. Sum up all these quotients.
fn main()
{
    let input = include_str!("../../inputs/day02.txt");
    let input = input.lines()
                .map(|row| row.split_ascii_whitespace()
                                   .map(|i| i.parse::<i32>().unwrap())
                                   .collect::<Vec<_>>())
                .collect::<Vec<_>>();
    
    let part01_cksum:i32 = input.iter().cloned().map(|mut row| {
        row.sort();
        row.last().unwrap() - row.first().unwrap()
    }).sum();
    println!("Part 1 : {}", part01_cksum);

    let part02_cksum:i32 = input.into_iter().map(|mut row| {
        row.sort_by(|a, b| b.cmp(a));
        for i in 0..row.len() {
            for j in (i+1)..row.len()
            {
                if row[i]%row[j] == 0
                {
                    return row[i]/row[j];
                }
            }
        }
        0
    }).sum();
    println!("Part 2 : {}", part02_cksum);
}
