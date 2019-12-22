use std::convert::TryFrom;
use std::convert::From;

use anyhow::{Result, anyhow};

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
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
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
    parameter_modes: Vec<ParameterMode>
}
pub struct OpCodeDef
{
    pub opcode_type: OpCodeType,
    pub parameter_count: u32,
    pub runner: fn(&mut IntComputer, Vec<ParameterMode>)
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
            5 => Ok(OpCodeType::JumpIfTrue),
            6 => Ok(OpCodeType::JumpIfFalse),
            7 => Ok(OpCodeType::LessThan),
            8 => Ok(OpCodeType::Equals),
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
            parameter_modes: parameter_modes.0
        })
    }
}

impl OpCode {
    pub fn validate(self) -> Result<Self> {
        let required_param_count = match self.kind {
            OpCodeType::Halt => 0,
            OpCodeType::Add => 3,
            OpCodeType::Mul => 3,
            OpCodeType::ReadFromAddr => 1,
            OpCodeType::StoreToAddr => 1,
            OpCodeType::JumpIfTrue => 2,
            OpCodeType::JumpIfFalse => 2,
            OpCodeType::LessThan => 3,
            OpCodeType::Equals => 3,
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
                            .collect()
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum CpuState
{
    RUNNING,
    HALTED
}

#[derive(Debug)]
pub struct IntComputer {
    cpu_state: CpuState,
    memory: Vec<i32>,
    ip: usize,
    input: i32,
    output: i32
}
impl IntComputer
{
    pub fn new() -> Self {
        IntComputer { 
            cpu_state: CpuState::RUNNING, 
            memory: Vec::new(), 
            ip: 0, 
            input: 0, 
            output: 0
        }
    }
    pub fn parse_parameters(&self, parameter_modes: Vec<ParameterMode>)
        -> Vec<Parameter>
    {
        let ip = self.ip;
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
    pub fn load_program(&mut self, program: &Vec<i32>) -> &mut Self
    {
        self.memory = program.clone();
        self
    }
    pub fn set_input(&mut self, value: i32) -> &mut Self {
        self.input = value;
        self
    }
    pub fn set_noun(&mut self, noun:i32) -> &mut Self {
        self.memory[1] = noun;
        self
    }
    pub fn set_verb(&mut self, verb: i32) -> &mut Self {
        self.memory[2] = verb;
        self
    }
    pub fn execute(&mut self) -> (Vec<i32>, i32)
    {
        self.cpu_state = CpuState::RUNNING;
        self.ip = 0;
        while self.cpu_state != CpuState::HALTED
        {
            
            let opcode_int = self.memory[self.ip];
            let opcode = OpCode::try_from(opcode_int)
                        .expect("Error parsing opcode")
                        .validate()
                        .expect("Error validating opcode");
            
            let parameters = self.parse_parameters(opcode.parameter_modes);
            // println!("{}, Opcode: {:?}, {:?}", self.ip, &opcode.kind, &parameters);
            match opcode.kind {
                OpCodeType::Halt => halt_op(self),
                OpCodeType::Add => add_op(self, parameters),
                OpCodeType::Mul => mul_op(self, parameters),
                OpCodeType::StoreToAddr => save_op(self, parameters),
                OpCodeType::ReadFromAddr => read_op(self, parameters),
                OpCodeType::JumpIfTrue => jump_if_true_op(self, parameters),
                OpCodeType::JumpIfFalse => jump_if_false_op(self, parameters),
                OpCodeType::LessThan => less_than_op(self, parameters),
                OpCodeType::Equals => equal_op(self, parameters),
            };
        }
        (self.memory.clone(), self.output)
    }
    pub fn read_memory(&self) -> Vec<i32>
    {
        self.memory.clone()
    }
}

fn add_op(vm: &mut IntComputer, parameters: Vec<Parameter>)
{
    let param1 = parameters[0].read(&vm.memory);
    let param2 = parameters[1].read(&vm.memory);
    parameters[2].write(&mut vm.memory, param1 + param2)
        .expect("Invalid output parameter for add()");

    vm.ip += 4;
}

fn mul_op(vm: &mut IntComputer, parameters: Vec<Parameter>)
{
    let param1 = parameters[0].read(&vm.memory);
    let param2 = parameters[1].read(&vm.memory);
    parameters[2].write(&mut vm.memory, param1 * param2)
        .expect("Invalid output parameter for mul()");

    vm.ip += 4;
}

fn save_op(vm: &mut IntComputer, parameters: Vec<Parameter>)
{
    parameters[0].write(&mut vm.memory, vm.input)
                 .expect("Invalid output parameter for save_op()");
    vm.ip += 2;
}
fn read_op(vm: &mut IntComputer, parameters: Vec<Parameter>)
{
    vm.output = parameters[0].read(&vm.memory);
    vm.ip += 2;
}

fn jump_if_true_op(vm: &mut IntComputer, parameters: Vec<Parameter>)
{
    if parameters[0].read(&vm.memory) != 0
    {
        let dst_addr = parameters[1].read(&vm.memory) as usize;
        vm.ip = dst_addr;
    }else{
        vm.ip += 3;
    }
}
fn jump_if_false_op(vm: &mut IntComputer, parameters: Vec<Parameter>)
{
    if parameters[0].read(&vm.memory) == 0
    {
        let dst_addr = parameters[1].read(&vm.memory) as usize;
        vm.ip = dst_addr;
    }else{
        vm.ip += 3;
    }
}
fn less_than_op(vm: &mut IntComputer, parameters: Vec<Parameter>)
{
    let param1 = parameters[0].read(&vm.memory);
    let param2 = parameters[1].read(&vm.memory);
    let output = (param1 < param2) as i32;
    parameters[2].write(&mut vm.memory, output)
                 .expect("Failed to write to memory");
    vm.ip += 4;
}
fn equal_op(vm: &mut IntComputer, parameters: Vec<Parameter>)
{
    let param1 = parameters[0].read(&vm.memory);
    let param2 = parameters[1].read(&vm.memory);
    let output = (param1 == param2) as i32;
    parameters[2].write(&mut vm.memory, output)
                 .expect("Failed to write to memory");
    vm.ip += 4;
}

fn halt_op(vm: &mut IntComputer)
{
    vm.cpu_state = CpuState::HALTED;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unit_tests_day_02() {
        let mut vm = IntComputer::new();
        assert_eq!(vm.load_program(&vec![1,0,0,0,99])
                      .set_noun(0)
                      .set_verb(0)
                      .execute().0,
                   vec![2,0,0,0,99]);
        assert_eq!(vm.load_program(&vec![2,3,0,3,99])
                      .execute().0,
                   vec![2,3,0,6,99]);
        assert_eq!(vm.load_program(&vec![2,4,4,5,99,0])
                      .set_noun(4)
                      .set_verb(4)
                      .execute().0,
                   vec![2,4,4,5,99,9801]);
        assert_eq!(vm.load_program(&vec![1,1,1,4,99,5,6,0,99])
                      .set_noun(1)
                      .set_verb(1)
                      .execute().0,
                   vec![30,1,1,4,2,5,6,0,99]);
        assert_eq!(vm.load_program(&vec![1,9,10,3,2,3,11,0,99,30,40,50])
                      .set_noun(9)
                      .set_verb(10)
                      .execute().0,
                   vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
    }
}