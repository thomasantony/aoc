use std::convert::From;
use std::convert::TryFrom;
use std::collections::VecDeque;
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub enum Parameter {
    Position(usize),
    Immediate(i32),
}

impl Parameter {
    fn read(&self, memory: &Vec<i32>) -> i32 {
        match *self {
            Parameter::Position(addr) => memory[addr],
            Parameter::Immediate(value) => value,
        }
    }
    fn write(&self, memory: &mut Vec<i32>, value: i32) -> Result<()> {
        match *self {
            Parameter::Position(addr) => {
                memory[addr] = value;
                Ok(())
            }
            Parameter::Immediate(_) => {
                Err(anyhow!("Attempted to write to immediate mode parameter"))
            }
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
    Halt,
}

#[derive(Debug, PartialEq)]
pub enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Debug)]
pub struct ParameterModes(Vec<ParameterMode>);

#[derive(Debug)]
pub struct OpCode {
    kind: OpCodeType,
    parameter_modes: Vec<ParameterMode>,
}

type OpCodeFn = fn(&mut IntComputer, Vec<Parameter>);

impl TryFrom<i32> for OpCodeType {
    type Error = anyhow::Error;
    fn try_from(opcode_int: i32) -> Result<OpCodeType> {
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
            _ => Err(anyhow!("Invalid opcode {}", opcode_int)),
        }
    }
}

impl From<i32> for ParameterModes {
    fn from(opcode_int: i32) -> ParameterModes {
        ParameterModes(
            opcode_int
                .to_string()
                .chars()
                .rev()
                .skip(2)
                .map(|m| match m {
                    '0' => ParameterMode::Position,
                    '1' => ParameterMode::Immediate,
                    _ => panic!("Invalid parameter mode in opcode"),
                })
                .collect::<Vec<ParameterMode>>(),
        )
    }
}
impl ParameterModes {
    pub fn get_by_index(&self, index: usize) -> &ParameterMode {
        self.0.get(index).unwrap_or(&ParameterMode::Position)
    }
}

impl TryFrom<i32> for OpCode {
    type Error = anyhow::Error;
    fn try_from(opcode_int: i32) -> Result<OpCode> {
        let opcode_type = OpCodeType::try_from(opcode_int)?;
        let parameter_modes = ParameterModes::from(opcode_int);
        Ok(OpCode {
            kind: opcode_type,
            parameter_modes: parameter_modes.0,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum CpuState {
    RUNNING,
    HALTED,
    WAITING,
}

#[derive(Debug)]
pub struct IntComputer {
    pub cpu_state: CpuState,
    memory: Vec<i32>,
    ip: usize,
    input: VecDeque<i32>,
    output: Option<i32>,
}
impl IntComputer {
    pub fn new() -> Self {
        IntComputer {
            cpu_state: CpuState::RUNNING,
            memory: Vec::new(),
            ip: 0,
            input: VecDeque::new(),
            output: None,
        }
    }
    pub fn parse_parameters(
        &self,
        parameter_count: usize,
        parameter_modes: ParameterModes,
    ) -> Vec<Parameter> {
        let ip = self.ip;
        let parameters = (0..parameter_count)
            .map(|i| parameter_modes.get_by_index(i))
            .enumerate()
            .map(|(i, mode)| match mode {
                ParameterMode::Position => Parameter::Position(self.memory[ip + 1 + i] as usize),
                ParameterMode::Immediate => Parameter::Immediate(self.memory[ip + 1 + i]),
            })
            .collect::<Vec<Parameter>>();
        parameters
    }
    pub fn load_program(&mut self, program: &Vec<i32>) -> &mut Self {
        self.memory = program.clone();
        self
    }
    pub fn push_input(&mut self, value: i32) -> &mut Self {
        self.input.push_back(value);
        self
    }
    pub fn set_noun(&mut self, noun: i32) -> &mut Self {
        self.memory[1] = noun;
        self
    }
    pub fn set_verb(&mut self, verb: i32) -> &mut Self {
        self.memory[2] = verb;
        self
    }
    pub fn is_halted(&mut self) -> bool 
    {
        self.cpu_state == CpuState::HALTED
    }
    pub fn execute(&mut self) -> i32 {
        // Reset instruction pointer if we are not waiting on input, else just resume
        // from last position
        if self.cpu_state != CpuState::WAITING {
            self.ip = 0;
        }
        self.cpu_state = CpuState::RUNNING;
        
        while self.cpu_state == CpuState::RUNNING {
            let opcode_int = self.memory[self.ip];
            let opcode = OpCodeType::try_from(opcode_int).expect("Error parsing opcode");
            let parameter_modes = ParameterModes::from(opcode_int);

            // println!("{}, Opcode: {:?}, {:?}", self.ip, &opcode.kind, &parameters);
            let (runner, parameter_count): (OpCodeFn, usize) = match opcode {
                OpCodeType::Halt => (halt_op, 0),
                OpCodeType::Add => (add_op, 3),
                OpCodeType::Mul => (mul_op, 3),
                OpCodeType::StoreToAddr => (store_op, 1),
                OpCodeType::ReadFromAddr => (read_op, 1),
                OpCodeType::JumpIfTrue => (jump_if_true_op, 2),
                OpCodeType::JumpIfFalse => (jump_if_false_op, 2),
                OpCodeType::LessThan => (less_than_op, 3),
                OpCodeType::Equals => (equal_op, 3),
            };
            let parameters = self.parse_parameters(parameter_count, parameter_modes);
            runner(self, parameters);
        }
        self.output.unwrap_or(self.memory[0])
    }
    pub fn read_memory(&self) -> Vec<i32> {
        self.memory.clone()
    }
}

fn add_op(vm: &mut IntComputer, parameters: Vec<Parameter>) {
    let param1 = parameters[0].read(&vm.memory);
    let param2 = parameters[1].read(&vm.memory);
    parameters[2]
        .write(&mut vm.memory, param1 + param2)
        .expect("Invalid output parameter for add()");

    vm.ip += 4;
}

fn mul_op(vm: &mut IntComputer, parameters: Vec<Parameter>) {
    let param1 = parameters[0].read(&vm.memory);
    let param2 = parameters[1].read(&vm.memory);
    parameters[2]
        .write(&mut vm.memory, param1 * param2)
        .expect("Invalid output parameter for mul()");

    vm.ip += 4;
}

fn store_op(vm: &mut IntComputer, parameters: Vec<Parameter>) {
    if let Some(val) = vm.input.pop_front()
    {
        parameters[0]
        .write(
            &mut vm.memory,
            val
        )
        .expect("Invalid output parameter for store_op()");
        vm.ip += 2;
    }else{
        // Wait for input if there is no input available
        vm.cpu_state = CpuState::WAITING;
    }
}
fn read_op(vm: &mut IntComputer, parameters: Vec<Parameter>) {
    vm.output = Some(parameters[0].read(&vm.memory));
    vm.ip += 2;
}

fn jump_if_true_op(vm: &mut IntComputer, parameters: Vec<Parameter>) {
    if parameters[0].read(&vm.memory) != 0 {
        let dst_addr = parameters[1].read(&vm.memory) as usize;
        vm.ip = dst_addr;
    } else {
        vm.ip += 3;
    }
}
fn jump_if_false_op(vm: &mut IntComputer, parameters: Vec<Parameter>) {
    if parameters[0].read(&vm.memory) == 0 {
        let dst_addr = parameters[1].read(&vm.memory) as usize;
        vm.ip = dst_addr;
    } else {
        vm.ip += 3;
    }
}
fn less_than_op(vm: &mut IntComputer, parameters: Vec<Parameter>) {
    let param1 = parameters[0].read(&vm.memory);
    let param2 = parameters[1].read(&vm.memory);
    let output = (param1 < param2) as i32;
    parameters[2]
        .write(&mut vm.memory, output)
        .expect("Failed to write to memory");
    vm.ip += 4;
}
fn equal_op(vm: &mut IntComputer, parameters: Vec<Parameter>) {
    let param1 = parameters[0].read(&vm.memory);
    let param2 = parameters[1].read(&vm.memory);
    let output = (param1 == param2) as i32;
    parameters[2]
        .write(&mut vm.memory, output)
        .expect("Failed to write to memory");
    vm.ip += 4;
}

fn halt_op(vm: &mut IntComputer, _: Vec<Parameter>) {
    vm.cpu_state = CpuState::HALTED;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unit_tests_day_02() {
        let mut vm = IntComputer::new();
        assert_eq!(
            vm.load_program(&vec![1, 0, 0, 0, 99])
                .set_noun(0)
                .set_verb(0)
                .execute(),
            2
        );
        assert_eq!(vm.load_program(&vec![2, 3, 0, 3, 99]).execute(), 2);
        assert_eq!(
            vm.load_program(&vec![2, 4, 4, 5, 99, 0])
                .set_noun(4)
                .set_verb(4)
                .execute(),
            2
        );
        assert_eq!(
            vm.load_program(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99])
                .set_noun(1)
                .set_verb(1)
                .execute(),
            30,
        );
        assert_eq!(
            vm.load_program(&vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50])
                .set_noun(9)
                .set_verb(10)
                .execute(),
            3500
        );
    }
}
