use ::aoc2019::intcode::*;
use ::aoc2019::*;
use itertools::Itertools;

fn compute_thruster_signal(phase: &[i32], program: &Vec<i32>) -> i32
{    
    let mut input = 0 as i32;
    let mut output: i32 = 0;

    for i in 0..5
    {
        let mut vm = IntComputer::new();
        vm.load_program(program);
        vm.push_input(phase[i]);
        vm.push_input(input);
        output = vm.execute();
        input = output;
    }
    output
}
fn compute_feedback_thruster_signal(phase: &[i32], program: &Vec<i32>) -> i32
{
    let mut amplifiers: Vec<IntComputer> = phase.iter().map(|&p| {
        let mut vm = IntComputer::new();
        vm.load_program(program).push_input(p);
        vm
    }).collect();

    
    let mut output = 0;
    let mut input = 0;

    loop {
        let final_amp_has_halted = amplifiers[4].is_halted();

        if final_amp_has_halted {
            break;
        }
        println!("Iteration start with input {}", input);
        for amp in amplifiers.iter_mut()
        {
            amp.push_input(input);
            output = amp.execute();
            input = output;
        }
        println!("Amp4 halted: {}", amplifiers[4].is_halted());
        println!("output after iter: {}", output);
    }
    output
}

/// Tries all combinations of phase and finds the maximum signal output possible
fn find_optimal_thruster_signal<F: Fn(&[i32], &Vec<i32>) -> i32>(program: &Vec<i32>, simulator: F) -> i32
{
    let mut max_output = 0;

    for phase in (0..5).permutations(5)
    {
        println!(":checking phase {:?}", &phase);
        let output = simulator(&phase, program);
        if output > max_output
        {
            println!("Resetting output to {}", output);
            max_output = output;
        }
    }
    max_output
}

fn main() {
    let input = read_stdin();
    let data: Vec<i32> = parse_numbers_with_delimiter(&input, ',').collect();

    // let output_a = find_optimal_thruster_signal(&data, compute_thruster_signal);
    // println!("Part A: {:?}", output_a);
    let output_b = find_optimal_thruster_signal(&data, compute_feedback_thruster_signal);
    println!("Part B: {:?}", output_b);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unit_tests_day_07_part_a() {
        let program_1 = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        assert_eq!(compute_thruster_signal(& [4, 3, 2, 1, 0], &program_1), 43210);
        assert_eq!(find_optimal_thruster_signal(&program_1, compute_thruster_signal), 43210);

        let program_2 = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,
                            101,5,23,23,1,24,23,23,4,23,99,0,0];
        assert_eq!(compute_thruster_signal(& [0, 1, 2, 3, 4], &program_2), 54321);
        assert_eq!(find_optimal_thruster_signal(&program_2, compute_thruster_signal), 54321);
        

        let program_3 = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                            1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        assert_eq!(compute_thruster_signal(& [1, 0, 4, 3, 2], &program_3), 65210);
        assert_eq!(find_optimal_thruster_signal(&program_3, compute_thruster_signal), 65210);
    }
    #[test]
    fn unit_tests_day_07_part_b() {
        let expected_output_1 = 139629729;
        let program_1 = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
                                       27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
        let output_1 = compute_feedback_thruster_signal(& [9, 8, 7, 6, 5], &program_1);
        assert_eq!(output_1, expected_output_1);
        assert_eq!(find_optimal_thruster_signal(&program_1, compute_feedback_thruster_signal), expected_output_1);

        let expected_output_2 = 18216;
        let program_2 = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
                                      -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                                      53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
        let output_2 = compute_feedback_thruster_signal(& [9, 7, 8, 5, 6], &program_2);
        assert_eq!(output_2, expected_output_2);
        // assert_eq!(find_optimal_thruster_signal(&program_2, compute_feedback_thruster_signal), expected_output_2);
    }
}