use ::aoc2019::{parse_digits, read_stdin};
use ndarray::prelude::*;
use std::iter::repeat;

fn nth_digit_pattern(n: usize, len: usize) -> impl Iterator<Item = i32> {
    let p0 = repeat(0).take(n - 1);
    let p1 = repeat(1).take(n);
    let p2 = repeat(0).take(n);
    let p3 = repeat(-1).take(n);
    let p4 = repeat(0).take(n);

    p0.chain(p1.chain(p2).chain(p3).chain(p4).cycle()).take(len)
}
fn compute_fft(input: &Vec<i32>, phases: i32) -> Vec<i32> {
    let mut input_vec = create_input_vec(&input);
    let n = input.len();
    let pattern = compute_pattern_matrix(n, 0);

    let mut output = Array::zeros((n, 1));
    for _ in 0..phases {
        output = compute_fft_phase_ndarray(&input_vec, &pattern);
        input_vec.assign(&output);
    }
    output.iter().cloned().collect()
}
fn create_input_vec(input: &Vec<i32>) -> Array2<i32> {
    let n = input.len();
    Array::from_shape_vec((n, 1), input.clone()).unwrap()
}
fn compute_pattern_matrix(n: usize, offset: usize) -> Array2<i32> {
    let mut pattern = Array::zeros((n, n));

    for i in offset..n {
        let pattern_row =
            Array::from_shape_vec((1, n), nth_digit_pattern(i + 1, n).collect()).unwrap();
        pattern.slice_mut(s![i.., ..]).assign(&pattern_row);
    }
    pattern
}
fn compute_fft_phase_ndarray(input_vec: &Array2<i32>, pattern_mat: &Array2<i32>) -> Array2<i32> {
    let n = input_vec.len();
    println!("n is {}", n);

    let mut output = pattern_mat.dot(input_vec);
    output.mapv_inplace(|d| (d % 10).abs());

    output
}
fn solve_part_b(input: &Vec<i32>) -> String {
    let n = input.len();

    let offset = input
        .iter()
        .take(7)
        .rev()
        .enumerate()
        .fold(0, |acc, (i, d)| acc + 10i32.pow(i as u32) * d);

    let real_input: Vec<i32> = input.iter().cloned().cycle().take(n * 10000).collect();

    assert!(
        offset as i32 > (real_input.len() as i32) / 2,
        "This shortcut depends on offset being > n/2"
    );
    let mut input: Vec<_> = real_input.iter().skip(offset as usize).cloned().collect();
    let n = input.len();
    let mut fft = vec![0; n];
    for _ in 0..100 {
        fft = vec![0; n];
        let mut cumulative_sum: i32 = input[n - 1];
        fft[n - 1] = cumulative_sum % 10;
        for i in 1..n {
            cumulative_sum = cumulative_sum + input[n - i - 1];
            fft[n - i - 1] = cumulative_sum % 10;
            input[n - i - 1] = fft[n - i - 1];
        }
    }
    let message: Vec<_> = fft.iter().take(8).collect();
    let part_b = message
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("");

    part_b
}
fn main() {
    let input = read_stdin();
    let digits: Vec<i32> = parse_digits(&input).map(|i| i as i32).collect();

    let out = compute_fft(&digits, 100);
    let part_a = out
        .iter()
        .take(8)
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("Part A: {}", part_a);

    let part_b = solve_part_b(&digits);
    println!("Part B: {}", part_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day16_base_pattern() {
        let output: Vec<_> = nth_digit_pattern(1, 8).collect();
        assert_eq!(output, vec![1, 0, -1, 0, 1, 0, -1, 0]);

        let output: Vec<_> = nth_digit_pattern(2, 8).collect();
        assert_eq!(output, vec![0, 1, 1, 0, 0, -1, -1, 0]);

        let output: Vec<_> = nth_digit_pattern(3, 11).collect();
        assert_eq!(output, vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1]);
    }
    #[test]
    fn test_day16_fft() {
        let input = parse_digits("80871224585914546619083218645595")
            .map(|i| i as i32)
            .collect();
        let output = compute_fft(&input, 100)
            .into_iter()
            .take(8)
            .collect::<Vec<_>>();
        assert_eq!(output, vec![2, 4, 1, 7, 6, 1, 7, 6]);

        let input = parse_digits("19617804207202209144916044189917")
            .map(|i| i as i32)
            .collect();
        let output = compute_fft(&input, 100)
            .into_iter()
            .take(8)
            .collect::<Vec<_>>();
        assert_eq!(output, vec![7, 3, 7, 4, 5, 4, 1, 8]);

        let input = parse_digits("69317163492948606335995924319873")
            .map(|i| i as i32)
            .collect();
        let output = compute_fft(&input, 100)
            .into_iter()
            .take(8)
            .collect::<Vec<_>>();
        assert_eq!(output, vec![5, 2, 4, 3, 2, 1, 3, 3]);
    }

    #[test]
    fn test_day16_partb() {
        let input = parse_digits("03036732577212944063491565474664")
            .map(|i| i as i32)
            .collect();
        assert_eq!(solve_part_b(&input), "84462026");

        let input = parse_digits("02935109699940807407585447034323")
            .map(|i| i as i32)
            .collect();
        assert_eq!(solve_part_b(&input), "78725270");

        let input = parse_digits("03081770884921959731165446850517")
            .map(|i| i as i32)
            .collect();
        assert_eq!(solve_part_b(&input), "53553731");
    }
}
