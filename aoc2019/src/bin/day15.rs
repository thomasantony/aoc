use ::aoc2019::intcode::*;
use ::aoc2019::{parse_numbers_with_delimiter};
use std::collections::HashMap;

type Coord = (i64, i64);
type MoveDirection = (i64, i64);

#[derive(Debug, Copy, Clone)]
pub enum Cell {
    Unknown,
    Empty,
    Wall,
    OxygenSystem
}
impl std::fmt::Display for &Cell 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Cell::*;
        let s = match self{
            Unknown => " ",
            Empty => ".",
            Wall=> "#",
            OxygenSystem => "*",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MoveCommand
{
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}
impl From<MoveCommand> for MoveDirection {
    fn from(other: MoveCommand) -> Self
    {
        use MoveCommand::*;
        match other
        {
            North => (0, 1),
            South => (0, -1),
            East => (1, 0),
            West => (-1, 0),
        }
    }
}

type Grid = HashMap<Coord, Cell>;

struct Robot {
    map: Grid,
    position: Coord,
    vm: IntComputer
}

impl Robot 
{
    pub fn new(program: &Vec<i64>) -> Self {
        let mut vm = IntComputer::new();
        vm.set_ram_size(1048576);
        vm.load_program(program);

        let mut map = Grid::new();
        map.insert((0, 0), Cell::Empty);
        Self {
            vm,
            position: (0, 0),
            map
        }
    }
    pub fn interpret_output(&mut self, last_command: MoveCommand, output: i64)
    {
        let walk_direction = MoveDirection::from(last_command);
        let new_pos = (self.position.0 + walk_direction.0, 
                                        self.position.1 + walk_direction.1);
        match output {
            0 => {
                // We hit a wall in the direction we walked
                self.map.insert(new_pos, Cell::Wall);
            },
            1 => {
                // Moved forward and its empty
                self.map.insert(new_pos, Cell::Empty);
                self.position = new_pos;
            },
            2 => {
                // Moved forward and found oxygen system
                self.map.insert(new_pos, Cell::OxygenSystem);
                self.position = new_pos;
            },
            _ => {}
        }
    }
    pub fn step(&mut self, command: MoveCommand)
    {
        self.vm.push_input(command as i64);
        let output = self.vm.execute();
        // Interpret the output and update map
        self.interpret_output(command, output[0]);
    }
    // Renders current map centered on the robot
    pub fn draw_map(&mut self, width: i64, height: i64)
    {
        let min_x = self.position.0 - width/2;
        let _max_x = self.position.0 + width/2 + 1;

        let _min_y = self.position.1 - height/2;
        let max_y = self.position.1 + height/2 + 1;
        
        for j in 0..height+1
        {
            let j = max_y - j - 1;
            for i in 0..width+1
            {
                let i = min_x + i;
                let pos = (i, j);
                if pos == self.position
                {
                    print!("D");
                }else{
                    print!("{}", self.map.get(& pos).unwrap_or(& Cell::Unknown));
                }
            }
            println!();
        }
    }
}

fn main()
{
    let input = include_str!("../../inputs/day15.txt").to_string();
    let program: Vec<i64> = parse_numbers_with_delimiter(&input, ',').collect();

    let mut robot = Robot::new(&program);
    robot.step(MoveCommand::North);
    robot.draw_map(10, 10);
    // println!("Part A: {:?}", painted_cells);
}