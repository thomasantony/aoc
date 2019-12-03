use ::aoc2019::*;

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
    let param1_addr = vm.memory[ip + 1] as usize;
    let param2_addr = vm.memory[ip + 2] as usize;
    let out_addr = vm.memory[ip + 3] as usize;
    assert!(param1_addr < vm.memory.len());
    assert!(param2_addr < vm.memory.len());
    assert!(out_addr < vm.memory.len());

    let param1 = vm.memory[param1_addr];
    let param2 = vm.memory[param2_addr];
    vm.memory[out_addr] = param1 + param2;

    vm.ip += 4;
}

fn mul_op(vm: &mut IntComputer)
{
    let ip = vm.ip as usize;
    let param1_addr = vm.memory[ip + 1] as usize;
    let param2_addr = vm.memory[ip + 2] as usize;
    let out_addr = vm.memory[ip + 3] as usize;
    assert!(param1_addr < vm.memory.len());
    assert!(param2_addr < vm.memory.len());
    assert!(out_addr < vm.memory.len());

    let param1 = vm.memory[param1_addr];
    let param2 = vm.memory[param2_addr];
    vm.memory[out_addr] = param1 * param2;

    vm.ip += 4;
}

fn halt_op(vm: &mut IntComputer)
{
    vm.cpu_state = CpuState::HALTED;
}
fn run_vm(program: &Vec<i32>, noun: i32, verb: i32) -> Vec<i32>
{
    let mut memory = program.clone();
    memory[1] = noun;
    memory[2] = verb;

    let mut vm = IntComputer { cpu_state: CpuState::RUNNING, memory: &mut memory, ip: 0};
    while vm.cpu_state != CpuState::HALTED
    {
        let opcode = vm.memory[vm.ip as usize];
        match opcode {
            99 => halt_op(&mut vm),
            1 => add_op(&mut vm),
            2 => mul_op(&mut vm),
            _ => panic!("invalid opcode {}", opcode)
        };
    }
    return memory;
}
fn main() 
{
    let input = read_stdin();
    let data: Vec<i32> = parse_numbers_with_delimiter(&input, ',')
                             .collect();
    let output_a = run_vm(&data, 12, 2);
    println!("Part A: {:?}", output_a[0]);
    
    for noun in 0..99
    {
        for verb in 0..99
        {
            let output = run_vm(&data, noun, verb);
            if output[0] == 19690720
            {
                println!("Part B: {}", noun*100 + verb);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unit_tests_day_02() {
        assert_eq!(run_vm(&vec![1,0,0,0,99], 0, 0), vec![2,0,0,0,99]);
        assert_eq!(run_vm(&vec![2,3,0,3,99], 3, 0), vec![2,3,0,6,99]);
        assert_eq!(run_vm(&vec![2,4,4,5,99,0], 4, 4), vec![2,4,4,5,99,9801]);
        assert_eq!(run_vm(&vec![1,1,1,4,99,5,6,0,99], 1, 1), vec![30,1,1,4,2,5,6,0,99]);
        assert_eq!(run_vm(&vec![1,9,10,3,2,3,11,0,99,30,40,50], 9, 10), 
                   vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
    }
}