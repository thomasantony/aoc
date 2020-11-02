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
fn compute_fft_phase(input: &Vec<i32>) -> Vec<i32>
{
    let n = input.len();
    // let input_vec = Array::from_shape_vec((1, n), input).unwrap();
    // let mut pattern = Array::zeros((n, n));
    // let mut input_mat = Array::zeros((n, n));
    // // input_mat.fill(input_vec);

    // println!("{:?}", input_mat);
    // Vec::new()

    let mut output = Vec::new();
    for i in 0..n
    {
        let pattern_gen = nth_digit_pattern(i+1).take(n);
        let out_digit = input.iter().zip(pattern_gen)
                                .fold(0, |acc, (x, y)| acc + x * y);
        output.push((out_digit % 10).abs());
    }
    output
}
fn compute_fft(input: &Vec<i32>, phases: i32) -> Vec<i32>
{
    let mut input = input;
    let mut output = Vec::new();
    for _ in 0..phases
    {
        output = compute_fft_phase(&input);
        input = &output;
    }
    output.iter().cloned().collect()
}
fn main()
{
    let input = read_stdin();
    let digits: Vec<i32> = parse_digits(&input).map(|i| i as i32).collect();
    let out = compute_fft(&digits, 100);
    let part_a = out.iter().take(8).map(|i| i.to_string()).collect::<Vec<String>>().join("");
    println!("Part A: {}", part_a);
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
    #[test]
    fn test_day16_fft()
    {
        let input = vec![1,2,3,4,5,6,7,8];
        let out = compute_fft_phase(&input);
        assert_eq!(out, vec![4, 8, 2, 2, 6, 1, 5, 8]);
        let out = compute_fft_phase(&out);
        assert_eq!(out, vec![3, 4, 0, 4, 0, 4, 3, 8]);

        let input = parse_digits("80871224585914546619083218645595")
                                .map(|i| i as i32).collect();
        let output = compute_fft(&input, 100).into_iter().take(8).collect::<Vec<_>>();
        assert_eq!(output, vec![2, 4, 1, 7, 6, 1, 7, 6]);

        let input = parse_digits("19617804207202209144916044189917")
                                .map(|i| i as i32).collect();
        let output = compute_fft(&input, 100).into_iter().take(8).collect::<Vec<_>>();
        assert_eq!(output, vec![7, 3, 7, 4, 5, 4, 1, 8]);

        let input = parse_digits("69317163492948606335995924319873")
                                .map(|i| i as i32).collect();
        let output = compute_fft(&input, 100).into_iter().take(8).collect::<Vec<_>>();
        assert_eq!(output, vec![5, 2, 4, 3, 2, 1, 3, 3]);
    }
}