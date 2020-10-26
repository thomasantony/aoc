use crate::intcode::IntComputer;
use std::collections::HashMap;

pub type Coord = (i64, i64);
pub type Grid = HashMap<Coord, Tile>;
const SCORE_COORD : (i64, i64)= (-1, 0);
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}
impl Tile {
    fn from_i64(value: i64) -> Tile {
        use Tile::*;
        match value {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => Paddle,
            4 => Ball,
            _ => panic!("Unknown value: {}", value),
        }
    }
}
#[derive(Debug)]
pub enum MoveCommand {
    Neutral = 0,
    Left = -1,
    Right = 1
}

#[derive(Debug)]
pub struct Bounds {
    pub max_x: u16,
    pub max_y: u16,
}

pub struct Arcade {
    pub grid: Grid,
    program: Vec<i64>,
    vm: IntComputer,
    pub score: i64,
    bounds: Bounds,
}
impl Arcade {
    pub fn new(program: &Vec<i64>) -> Self {
        let mut vm = IntComputer::new();
        vm.set_ram_size(1048576);
        let mut program = program.clone();
        program[0] = 2; // Insert quarter
        vm.load_program(&program);
        Self {
            grid: Grid::new(),
            program,
            vm,
            score: 0,
            bounds: Bounds{max_x: 37, max_y: 21}
        }
    }
    pub fn reset(&mut self)
    {
        self.vm.reset();
        self.vm.load_program(&self.program);
        self.grid.clear();
    }
    pub fn insert_quarter(&mut self)
    {
        self.program[0] = 2;
        self.reset();
    }
    pub fn command(&mut self, command: MoveCommand)
    {
        let cmd_input = command as i64;
        self.vm.push_input(cmd_input);
    }
    pub fn num_blocks_remaining(&self) -> usize
    {
        self.grid.values().filter(|&t| t == &Tile::Block).count()
    }
    pub fn num_empty_remaining(&self) -> usize
    {
        self.grid.values().filter(|&t| t == &Tile::Empty).count()
    }
    pub fn run_once(&mut self)
    {
        let output = self.vm.execute();
        self.grid.clear();
        for command in output.chunks(3)
        {
            if command.len() != 3
            {
                continue;
            }
            let pos = (command[0], command[1]);

            if pos == SCORE_COORD
            {
                self.score = command[2];
            }else{
                let tile = Tile::from_i64(command[2]);
                self.grid.insert(pos, tile);
            }
        }
    }
}