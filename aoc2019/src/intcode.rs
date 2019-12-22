use std::convert::TryFrom;
use std::convert::From;

use anyhow::{Result, anyhow};

#[derive(Debug, PartialEq)]
pub enum CpuState
{
    RUNNING,
    HALTED
}

#[derive(Debug)]
pub enum Parameter {
    Position(usize),
    Immediate(i32)
}

impl Parameter {
    fn read(&self, memory: &Vec<i32>) -> i32 {
        match *self {
            Parameter::Position(addr) => memory[addr],
            Parameter::Immediate(value) => value
        }
    }
    fn write(&self, memory: &mut Vec<i32>, value: i32) -> Result<()>
    {
        match *self {
            Parameter::Position(addr) => {
                println!("Wrote {} to address {}", value, addr);
                memory[addr] = value;
                Ok(())
            },
            Parameter::Immediate(_) => Err(anyhow!("Attempted to write to immediate mode parameter"))
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum OpCodeType {
    Add,
    Mul,
    ReadFromAddr,
    StoreToAddr,
    Halt
}

#[derive(Debug, PartialEq)]
pub enum ParameterMode {
    Position,
    Immediate
}

#[derive(Debug)]
pub struct ParameterModes(Vec<ParameterMode>);

#[derive(Debug)]
pub struct OpCode {
    kind: OpCodeType,
    parameter_modes: Vec<ParameterMode>,
    validated: bool
}

impl TryFrom<i32> for OpCodeType {
    type Error = anyhow::Error;
    fn try_from(opcode_int: i32) -> Result<OpCodeType>
    {
        match opcode_int % 100 as i32 {
            1 => Ok(OpCodeType::Add),
            2 => Ok(OpCodeType::Mul),
            3 => Ok(OpCodeType::StoreToAddr),
            4 => Ok(OpCodeType::ReadFromAddr),
            99 => Ok(OpCodeType::Halt),
            _ => Err(anyhow!("Invalid opcode {}", opcode_int))
        }
    }
}

impl From<i32> for ParameterModes
{
    fn from(opcode_int: i32) -> ParameterModes
    {
        ParameterModes(opcode_int.to_string().chars().rev().skip(2).map(|m| match m {
            '0' => ParameterMode::Position,
            '1' => ParameterMode::Immediate,
            _ => panic!("Invalid parameter mode in opcode")
        }).collect::<Vec<ParameterMode>>())
    }
}


impl TryFrom<i32> for OpCode {
    type Error = anyhow::Error;
    fn try_from(opcode_int: i32) -> Result<OpCode>
    {
        let opcode_type = OpCodeType::try_from(opcode_int)?;
        let parameter_modes = ParameterModes::from(opcode_int);
        Ok(OpCode{
            kind: opcode_type,
            parameter_modes: parameter_modes.0,
            validated: false
        })
    }
}

impl OpCode {
    pub fn validate(self) -> Result<Self> {
        let required_param_count = match self.kind {
            OpCodeType::Halt => 0,
            OpCodeType::Add => 3,
            OpCodeType::Mul => 3,
            OpCodeType::ReadFromAddr => 3,
            OpCodeType::StoreToAddr => 3
        };
        let specified_param_count = self.parameter_modes.len() as i32;

        if required_param_count < specified_param_count 
        {
            Err(anyhow!("Too many parameters specified in opcode"))?
        }

        let unspecified_param_count = required_param_count - specified_param_count;
        let unspecified_mode_chars = "0".repeat(unspecified_param_count as usize);
        let unspecified_modes = unspecified_mode_chars
                                .chars().map(|m| match m {
            '0' => ParameterMode::Position,
            '1' => ParameterMode::Immediate,
            _ => panic!("Invalid parameter mode in opcode")
        });

        Ok(OpCode{
            kind: self.kind,
            parameter_modes: self.parameter_modes.into_iter()
                            .chain(unspecified_modes.into_iter())
                            .collect(),
            validated: true
        })
    }
}

#[derive(Debug)]
pub struct IntComputer<'a> {
    cpu_state: CpuState,
    memory: &'a mut Vec<i32>,
    ip: i32,
    input: i32,
    output: i32
}
impl IntComputer<'_>
{
    pub fn parse_parameters(&self, parameter_modes: Vec<ParameterMode>)
        -> Vec<Parameter>
    {
        let ip = self.ip as usize;
        let parameters = parameter_modes.iter().enumerate()
        .map(|(i, mode)|{
            match mode {
                ParameterMode::Position => Parameter::Position(self.memory[ip+1+i] as usize),
                ParameterMode::Immediate => Parameter::Immediate(self.memory[ip+1+i])
            }
        })
        .collect::<Vec<Parameter>>();
        parameters
    }
}

fn add_op(vm: &mut IntComputer, parameter_modes: Vec<ParameterMode>)
{
    let ip = vm.ip as usize;
    let parameters = vm.parse_parameters(parameter_modes);
    let param1 = parameters[0].read(&vm.memory);
    let param2 = parameters[1].read(&vm.memory);
    parameters[2].write(&mut vm.memory, param1 + param2)
        .expect("Invalid output parameter for add()");

    vm.ip += 4;
}

fn mul_op(vm: &mut IntComputer, parameter_modes: Vec<ParameterMode>)
{
    let ip = vm.ip as usize;
    let parameters = vm.parse_parameters(parameter_modes);
    let param1 = parameters[0].read(&vm.memory);
    let param2 = parameters[1].read(&vm.memory);
    parameters[2].write(&mut vm.memory, param1 * param2)
        .expect("Invalid output parameter for mul()");

    vm.ip += 4;
}

fn save_op(vm: &mut IntComputer)
{
    let ip = vm.ip as usize;
    let write_addr = vm.memory[ip + 1] as usize;
    assert!(write_addr < vm.memory.len());
    vm.memory[write_addr] = vm.input;
}
fn read_op(vm: &mut IntComputer)
{
    let ip = vm.ip as usize;
    let read_addr = vm.memory[ip + 1] as usize;
    assert!(read_addr < vm.memory.len());
    vm.output = vm.memory[read_addr];
}

fn halt_op(vm: &mut IntComputer)
{
    vm.cpu_state = CpuState::HALTED;
}

pub fn run_vm(program: &Vec<i32>, noun: i32, verb: i32) -> Vec<i32>
{
    let mut memory = program.clone();
    memory[1] = noun;
    memory[2] = verb;

    let mut vm = IntComputer { cpu_state: CpuState::RUNNING, memory: &mut memory, ip: 0, input: 0, output: 0};
    while vm.cpu_state != CpuState::HALTED
    {
        let opcode_int = vm.memory[vm.ip as usize];
        let opcode = OpCode::try_from(opcode_int)
                    .expect("Error parsing opcode")
                    .validate()
                    .expect("Error validating opcode");
                    
        match opcode.kind {
            OpCodeType::Halt => halt_op(&mut vm),
            OpCodeType::Add => add_op(&mut vm, opcode.parameter_modes),
            OpCodeType::Mul => mul_op(&mut vm, opcode.parameter_modes),
            OpCodeType::StoreToAddr => save_op(&mut vm),
            OpCodeType::ReadFromAddr => read_op(&mut vm)
        };
    }
    return memory;
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn unit_test_parameter_modes() {
//         assert_eq!(parse_compound_opcode(01), (OpCodeType::Add, 
//                     vec![ParameterMode::Position,
//                          ParameterMode::Position,
//                          ParameterMode::Position]));
//         assert_eq!(parse_compound_opcode(1101), (OpCodeType::Add, 
//                     vec![ParameterMode::Immediate,
//                          ParameterMode::Immediate,
//                          ParameterMode::Position]));
//         assert_eq!(parse_compound_opcode(1001), (OpCodeType::Add, vec![
//                          ParameterMode::Position,
//                          ParameterMode::Immediate,
//                          ParameterMode::Position]));
//         assert_eq!(parse_compound_opcode(99), (OpCodeType::Halt, vec![]));

//     }
// }