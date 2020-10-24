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
    let amplifiers: Vec<IntComputer> = phase.iter().map(|&p| {
            let mut vm = IntComputer::new();
            vm.load_program(program).push_input(p);
            vm
        }).collect();
    
    0
}

/// Tries all combinations of phase and finds the maximum signal output possible
fn find_optimal_thruster_signal(program: &Vec<i32>) -> i32
{
    let mut max_output = 0;

    for phase in (0..5).permutations(5)
    {
        let output = compute_thruster_signal(&phase, program);
        if output > max_output
        {
            max_output = output;
        }
    }
    max_output
}

fn main() {
    let input = read_stdin();
    let data: Vec<i32> = parse_numbers_with_delimiter(&input, ',').collect();

    let output_a = find_optimal_thruster_signal(&data);
    println!("Part A: {:?}", output_a);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unit_tests_day_07() {
        let program_1 = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        assert_eq!(compute_thruster_signal(& [4, 3, 2, 1, 0], &program_1), 43210);
        assert_eq!(find_optimal_thruster_signal(&program_1), 43210);

        let program_2 = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,
                            101,5,23,23,1,24,23,23,4,23,99,0,0];
        assert_eq!(compute_thruster_signal(& [0, 1, 2, 3, 4], &program_2), 54321);
        assert_eq!(find_optimal_thruster_signal(&program_2), 54321);
        

        let program_3 = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                            1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        assert_eq!(compute_thruster_signal(& [1, 0, 4, 3, 2], &program_3), 65210);
        assert_eq!(find_optimal_thruster_signal(&program_3), 65210);
    }
}