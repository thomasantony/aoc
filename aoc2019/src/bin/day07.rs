use ::aoc2019::intcode::*;
use ::aoc2019::*;

fn compute_amplifier_signal(phase: &[i32], program: &Vec<i32>) -> i32
{
    
    let mut input = 0 as i32;
    let mut output: i32 = 0;

    // let mut vmA = IntComputer::new();
    // let mut vmB = IntComputer::new();
    // let mut vmC = IntComputer::new();
    // let mut vmD = IntComputer::new();
    // let mut vmE = IntComputer::new();

    // vmA.load_program(program).push_input(phase[0]);
    // vmA.push_input(0);
    // output = vmA.execute();

    // vmB.load_program(program).push_input(phase[1]);
    // vmB.push_input(output);
    // output = vmB.execute();

    // vmC.load_program(program).push_input(phase[2]);
    // vmC.push_input(output);
    // output = vmC.execute();

    // vmD.load_program(program).push_input(phase[3]);
    // vmD.push_input(output);
    // output = vmD.execute();

    // vmE.load_program(program).push_input(phase[4]);
    // vmE.push_input(output);
    // output = vmE.execute();

    for i in 0..5
    {
        let mut vm = IntComputer::new();
        vm.load_program(program);
        vm.push_input(phase[i]);
        vm.push_input(input);
        output = vm.execute();
        input = output;
    }
    // println!("Got output {} for {}, phase {}", output, input, phase[i]);
    output
}
fn next_permutation(phase: [i32; 5]) -> [i32; 5]
{
    let mut output: [i32; 5] = [0, 0, 0, 0, 0];

    let mut carry = 1;
    for i in 0..5
    {
        let idx = 5 - i - 1;

        let current_val = phase[idx];

        let mut new_val = current_val + carry;
        if new_val > 4
        {
            carry = 1;
            new_val = 0;
        }else{
            carry = 0;
        }
        output[idx] = new_val;
    }
    output
}
/// Tries all combinations of phase and finds the maximum signal output possible
fn find_optimal_thruster_signal(program: &Vec<i32>) -> i32
{
    let total_combos = 3125; // 5^5
    let mut phase = [0,0,0,0,0];
    let mut max_output = 0;

    use itertools::Itertools;

    // let perms = (0..5).combinations_with_replacement(5);

    // println!("{}", perms.collect::<Vec<Vec<i32>>>().len());
    for _ in 0..3125
    {
        let output = compute_amplifier_signal(&phase, program);
        println!("Got {} for {:?}", output, &phase);
        if output > max_output
        {
            max_output = output;
        }
        phase = next_permutation(phase);
    }
    max_output
}

fn main() {
    let input = read_stdin();
    let data: Vec<i32> = parse_numbers_with_delimiter(&input, ',').collect();

    let output_a = find_optimal_thruster_signal(&data);
    println!("Part A: {:?}", output_a);

    // let output_b = vm.load_program(&data).push_input(5).execute();
    // println!("Part B: {:?}", output_b);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day07_permutations() {
        assert_eq!(next_permutation([0,0,0,0,0]), [0,0,0,0,1]);
        assert_eq!(next_permutation([0,0,0,0,4]), [0,0,0,1,0]);
        assert_eq!(next_permutation([0,0,0,1,4]), [0,0,0,2,0]);
        assert_eq!(next_permutation([0,0,0,4,4]), [0,0,1,0,0]);
        assert_eq!(next_permutation([0,0,4,0,1]), [0,0,4,0,2]);
    }
    #[test]
    fn unit_tests_day_07() {
        let program_1 = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        // assert_eq!(compute_amplifier_signal(& [4, 3, 2, 1, 0], &program_1), 43210);
        // assert_eq!(find_optimal_thruster_signal(&program_1), 43210);

        let program_2 = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,
                            101,5,23,23,1,24,23,23,4,23,99,0,0];
        // compute_amplifier_signal(& [0, 0, 0, 0, 0], &program_2);
        assert_eq!(find_optimal_thruster_signal(&program_2), 54321);
        // assert_eq!(compute_amplifier_signal(& [0, 1, 2, 3, 4], &program_2), 54321);

        // let program_3 = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
        //                     1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        // assert_eq!(compute_amplifier_signal(& [1, 0, 4, 3, 2], &program_3), 65210);
    }
}