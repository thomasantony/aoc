use ::aoc2019::intcode::*;
use ::aoc2019::parse_numbers_with_delimiter;
use std::collections::HashMap;

type Coord = (i64, i64);
type Grid = HashMap<Coord, Tile>;
const SCORE_COORD : (i64, i64)= (-1, 0);
#[derive(Debug, PartialEq, Clone)]
enum Tile {
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
struct Arcade {
    pub grid: Grid,
    program: Vec<i64>,
    vm: IntComputer,
    score: i64
}
impl Arcade {
    pub fn new(program: &Vec<i64>) -> Self {
        let mut vm = IntComputer::new();
        vm.set_ram_size(2048);
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
fn main() {
    let input = include_str!("../../inputs/day13.txt").to_string();
    let program: Vec<i64> = parse_numbers_with_delimiter(&input, ',').collect();
    let mut arcade = Arcade::new(&program);

    arcade.run_once();
    println!("Part A: {}", arcade.num_blocks_remaining());

    arcade.insert_quarter();

}