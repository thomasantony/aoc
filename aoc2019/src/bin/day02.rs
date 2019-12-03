use ::aoc2019::*;
use std::collections::HashMap;

// type OpcodeFn = Fn<>;
type OpcodeFn = fn(&mut IntComputer) -> ();


#[derive(Debug, PartialEq)]
enum CpuState
{
    RUNNING,
    HALTED
}

#[derive(Debug)]
struct IntComputer<'a> {
    cpu_state: CpuState,
    memory: &'a mut Vec<i32>,
    ip: i32
}

fn add_op(vm: &mut IntComputer)
{
    let ip = vm.ip as usize;
    let op1_addr = vm.memory[ip + 1] as usize;
    let op2_addr = vm.memory[ip + 2] as usize;
    let out_addr = vm.memory[ip + 3] as usize;
    assert!(op1_addr < vm.memory.len());
    assert!(op2_addr < vm.memory.len());
    assert!(out_addr < vm.memory.len());

    let op1 = vm.memory[op1_addr];
    let op2 = vm.memory[op2_addr];
    vm.memory[out_addr] = op1 + op2;

    vm.ip += 4;
}

fn mul_op(vm: &mut IntComputer)
{
    let ip = vm.ip as usize;
    let op1_addr = vm.memory[ip + 1] as usize;
    let op2_addr = vm.memory[ip + 2] as usize;
    let out_addr = vm.memory[ip + 3] as usize;
    assert!(op1_addr < vm.memory.len());
    assert!(op2_addr < vm.memory.len());
    assert!(out_addr < vm.memory.len());

    let op1 = vm.memory[op1_addr];
    let op2 = vm.memory[op2_addr];
    vm.memory[out_addr] = op1 * op2;

    vm.ip += 4;
}

fn halt_op(vm: &mut IntComputer)
{
    vm.cpu_state = CpuState::HALTED;
}
fn run_vm(program: &Vec<i32>) -> Vec<i32>
{
    let mut opcode_table: HashMap<i32, OpcodeFn> = HashMap::new();

    let mut memory = program.clone();
    opcode_table.insert(99, halt_op);
    opcode_table.insert(1, add_op);
    opcode_table.insert(2, mul_op);

    let mut vm = IntComputer { cpu_state: CpuState::RUNNING, memory: &mut memory, ip: 0};
    while vm.cpu_state != CpuState::HALTED
    {
        let opcode = vm.memory[vm.ip as usize];
        let opfn = opcode_table[&opcode];
        opfn(&mut vm);
    }
    return memory;
}
fn main() 
{
    let input = read_stdin();
    let data: Vec<i32> = parse_numbers_with_delimiter(&input, ',')
                             .collect();
    let output = run_vm(&data);
    println!("Output: {:?}", output);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unit_tests_day_02() {
        assert_eq!(run_vm(&vec![1,0,0,0,99]), vec![2,0,0,0,99]);
        assert_eq!(run_vm(&vec![2,3,0,3,99]), vec![2,3,0,6,99]);
        assert_eq!(run_vm(&vec![2,4,4,5,99,0]), vec![2,4,4,5,99,9801]);
        assert_eq!(run_vm(&vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99]);
        assert_eq!(run_vm(&vec![1,9,10,3,2,3,11,0,99,30,40,50]), 
                   vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
        
        // assert_eq!(calc_fuel(14), 2);
        // assert_eq!(calc_fuel(1969), 654);
        // assert_eq!(calc_fuel(100756), 33583);

        // assert_eq!(calc_fuel_cumulative(14), 2);
        // assert_eq!(calc_fuel_cumulative(1969), 966);
        // assert_eq!(calc_fuel_cumulative(100756), 50346);
    }
}