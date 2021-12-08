use ::aoc2019::intcode::*;
use ::aoc2019::*;
use itertools::Itertools;
use std::ops::Range;

fn compute_thruster_signal(phase: &[i64], program: &Vec<i64>) -> i64 {
    let mut input = 0 as i64;
    let mut output: i64 = 0;

    for i in 0..5 {
        let mut vm = IntComputer::new();
        vm.load_program(program);
        vm.push_input(phase[i]);
        vm.push_input(input);
        let result = vm.execute();
        output = result[0];
        input = output;
    }
    output
}
fn compute_feedback_thruster_signal(phase: &[i64], program: &Vec<i64>) -> i64 {
    let mut amplifiers: Vec<IntComputer> = phase
        .iter()
        .map(|&p| {
            let mut vm = IntComputer::new();
            vm.load_program(program).push_input(p);
            vm
        })
        .collect();

    let mut output = 0;
    let mut input = 0;

    loop {
        let final_amp_has_halted = amplifiers[4].is_halted();

        if final_amp_has_halted {
            break;
        }
        for amp in amplifiers.iter_mut() {
            amp.push_input(input);
            let result = amp.execute();
            output = result[0];
            input = output;
        }
    }
    output
}

/// Tries all combinations of phase and finds the maximum signal output possible
fn find_optimal_thruster_signal<F: Fn(&[i64], &Vec<i64>) -> i64>(
    program: &Vec<i64>,
    simulator: F,
    phase_range: Range<i64>,
) -> i64 {
    let mut max_output = 0;

    for phase in phase_range.permutations(5) {
        let output = simulator(&phase, program);
        if output > max_output {
            max_output = output;
        }
    }
    max_output
}

fn main() {
    let input = read_stdin();
    let data: Vec<i64> = parse_numbers_with_delimiter(&input, ',').collect();

    let output_a = find_optimal_thruster_signal(&data, compute_thruster_signal, 0..5);
    println!("Part A: {:?}", output_a);

    let output_b = find_optimal_thruster_signal(&data, compute_feedback_thruster_signal, 5..10);
    println!("Part B: {:?}", output_b);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unit_tests_day_07_part_a() {
        let program_1 = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(compute_thruster_signal(&[4, 3, 2, 1, 0], &program_1), 43210);
        assert_eq!(
            find_optimal_thruster_signal(&program_1, compute_thruster_signal, 0..5),
            43210
        );

        let program_2 = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(compute_thruster_signal(&[0, 1, 2, 3, 4], &program_2), 54321);
        assert_eq!(
            find_optimal_thruster_signal(&program_2, compute_thruster_signal, 0..5),
            54321
        );

        let program_3 = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(compute_thruster_signal(&[1, 0, 4, 3, 2], &program_3), 65210);
        assert_eq!(
            find_optimal_thruster_signal(&program_3, compute_thruster_signal, 0..5),
            65210
        );
    }
    #[test]
    fn unit_tests_day_07_part_b() {
        let expected_output_1 = 139629729;
        let program_1 = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let output = compute_feedback_thruster_signal(&[9, 8, 7, 6, 5], &program_1);
        assert_eq!(output, expected_output_1);
        let output =
            find_optimal_thruster_signal(&program_1, compute_feedback_thruster_signal, 5..10);
        assert_eq!(output, expected_output_1);

        let expected_output_2 = 18216;
        let program_2 = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let output = compute_feedback_thruster_signal(&[9, 7, 8, 5, 6], &program_2);
        assert_eq!(output, expected_output_2);
        let output =
            find_optimal_thruster_signal(&program_2, compute_feedback_thruster_signal, 5..10);
        assert_eq!(output, expected_output_2);
    }
}
