use ::aoc2019::{parse_digits, read_stdin};
use std::iter::repeat;

fn nth_digit_pattern(n: usize) -> impl Iterator<Item=i32>
{
    let p0 = repeat(0).take(n-1);
    let p1 = repeat(1).take(n);
    let p2 = repeat(0).take(n);
    let p3 = repeat(-1).take(n);
    let p4 = repeat(0).take(n);

    p0.chain(p1.chain(p2).chain(p3).chain(p4).cycle())
}
fn main()
{
    let input = read_stdin();
    let digits = parse_digits(&input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day16_base_pattern()
    {
        let output: Vec<_> = nth_digit_pattern(1).take(8).collect();
        assert_eq!(output, vec![1, 0, -1, 0, 1, 0, -1, 0]);

        let output: Vec<_> = nth_digit_pattern(2).take(8).collect();
        assert_eq!(output, vec![0, 1, 1, 0, 0, -1, -1, 0]);

        let output: Vec<_> = nth_digit_pattern(3).take(11).collect();
        assert_eq!(output, vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1]);
    }
}