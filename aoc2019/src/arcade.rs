use crate::intcode::IntComputer;
use std::collections::HashMap;

type Coord = (i64, i64);
pub type Grid = HashMap<Coord, Tile>;
const SCORE_COORD : (i64, i64)= (-1, 0);
#[derive(Debug, PartialEq, Clone)]
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

pub struct Bounds {
    min_x: u16,
    min_y: u16,
    max_x: u16,
    max_y: u16,
}

pub struct Arcade {
    pub grid: Grid,
    program: Vec<i64>,
    vm: IntComputer,
    score: i64
}
impl Arcade {
    pub fn new(program: &Vec<i64>) -> Self {
        let mut vm = IntComputer::new();
        vm.set_ram_size(102400);
        vm.load_program(program);

        Self {
            grid: Grid::new(),
            program: program.clone(),
            vm,
            score: 0,
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
    pub fn get_bounds(&self) -> Bounds
    {
        let min_x = self.grid.keys().min_by_key(|p|p.0).unwrap().0 as u16;
        let min_y = self.grid.keys().min_by_key(|p|p.1).unwrap().1 as u16;
        let max_x = self.grid.keys().max_by_key(|p|p.0).unwrap().0 as u16;
        let max_y = self.grid.keys().max_by_key(|p|p.1).unwrap().1 as u16;
        Bounds{min_x, max_x, min_y, max_y}
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
    pub fn run_once(&mut self)
    {
        let output = self.vm.execute();
        self.grid.clear();
        for command in output.chunks(3)
        {
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