use ::aoc2019::*;
use ::aoc2019::intcode::*;

fn main() 
{
    let input = read_stdin();
    let data: Vec<i32> = parse_numbers_with_delimiter(&input, ',')
                             .collect();
    let mut vm = IntComputer::new();

    let output_a = vm.load_program(&data).set_input(1).execute();
    println!("Part A: {:?}", output_a.1);
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn unit_tests_day_05() {
//         assert_eq!(run_vm(&vec![1,0,0,0,99], 0, 0), vec![2,0,0,0,99]);
//         assert_eq!(run_vm(&vec![2,3,0,3,99], 3, 0), vec![2,3,0,6,99]);
//         assert_eq!(run_vm(&vec![2,4,4,5,99,0], 4, 4), vec![2,4,4,5,99,9801]);
//         assert_eq!(run_vm(&vec![1,1,1,4,99,5,6,0,99], 1, 1), vec![30,1,1,4,2,5,6,0,99]);
//         assert_eq!(run_vm(&vec![1,9,10,3,2,3,11,0,99,30,40,50], 9, 10), 
//                    vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
//     }
// }