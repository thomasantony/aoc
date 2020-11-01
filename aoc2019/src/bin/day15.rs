use ::aoc2019::intcode::*;
use ::aoc2019::{parse_numbers_with_delimiter};
use std::collections::{HashMap, HashSet, VecDeque};
use std::thread;
use std::io::{Write, Read};
use std::time::{Instant, Duration};
use termion::{color, cursor, clear, style};
use termion::raw::IntoRawMode;

type Coord = (i64, i64);
type MoveDirection = (i64, i64);

#[derive(Debug, Copy, Clone, PartialEq)]
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
            Unknown => format!(" "),
            Empty => format!("{}·", color::Fg(color::Yellow)),
            Wall=> format!("{}#", color::Fg(color::LightBlue)),
            OxygenSystem => format!("{}*", color::Fg(color::Red)),
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
impl MoveCommand {
    fn reverse(&self) -> Self {
        use MoveCommand::*;
        match self {
            North => South,
            South => North,
            East => West,
            West => East
        }
    }
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

struct Grid (HashMap<Coord, Cell>);
impl Grid {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}
use ::aoc2019::graph::GenericGraph;
impl GenericGraph<Coord> for Grid {
    fn successors(&self, node: &Coord) -> Vec<Coord>
    {
        let all_cmds = vec![MoveCommand::West, MoveCommand::South, MoveCommand::East, MoveCommand::North];
        let all_directions = all_cmds.into_iter().map(|c| MoveDirection::from(c));
        all_directions.map(|dir| {
            (node.0 + dir.0, node.1 + dir.1)
        }).collect()
    }
    fn vertices(&self) -> Vec<Coord>
    {
        self.0.keys().map(|i| i.clone()).collect()
    }
}
struct Robot {
    map: Grid,
    vm: IntComputer
}

pub fn get_position_for_command(position: Coord, command: MoveCommand) -> Coord {
    let walk_direction = MoveDirection::from(command);
    let new_pos = (position.0 + walk_direction.0, 
                                    position.1 + walk_direction.1);
    new_pos
}
pub fn get_new_directions(command: MoveCommand) -> Vec<MoveCommand>
{
    use MoveCommand::*;
    match command
    {
        North => vec![East, North, West],
        East => vec![South, East, North],
        South => vec![West, South, East],
        West => vec![North, West, South,],
    }
}
impl Robot 
{
    pub fn new(program: &Vec<i64>) -> Self {
        let mut vm = IntComputer::new();
        vm.set_ram_size(1048576);
        vm.load_program(program);

        let mut map = Grid::new();
        map.0.insert((0, 0), Cell::Empty);
        Self {
            vm,
            map
        }
    }
    pub fn move_robot(&mut self, command: MoveCommand) -> Cell
    {
        self.vm.push_input(command as i64);
        let output = self.vm.execute();
        match output[0] {
            0 => {
                // We hit a wall in the direction we walked
                Cell::Wall
            },
            1 => {
                // Moved forward and its empty
                Cell::Empty
            },
            2 => {
                // Moved forward and found oxygen system
                Cell::OxygenSystem
            },
            _ => Cell::Unknown
        }
    }
    
    // returns cell type in the given direction by taking a step in that direction
    // and stepping back if it is an open cell
    pub fn move_and_get_cell_type(&mut self, command: MoveCommand) -> Cell
    {
        let output = self.move_robot(command);
        output
    }
    pub fn map_and_find_o2_system(&mut self) -> (Grid, Option<Coord>)
    {
        let start_node: Coord = (0, 0);
        let mut q = VecDeque::from(vec![start_node]);
        let mut map = Grid::new();

        let all_cmds = vec![MoveCommand::West, MoveCommand::South, MoveCommand::East, MoveCommand::North];
        let mut path: Vec<MoveCommand> = Vec::new();
        
        map.0.insert(start_node, Cell::Empty);
        
        let mut o2_pos = None;

        let mut unexplored_actions = HashMap::new();
        unexplored_actions.insert(start_node, all_cmds);
        while !q.is_empty()
        {
            let node = q.pop_front().unwrap();
            let mut available_cmds= unexplored_actions.remove(&node).unwrap();
            let mut found_new_cell = false;
            while available_cmds.len() > 0
            {
                let cmd = available_cmds.pop().unwrap();
                let new_pos = get_position_for_command(node, cmd);
                let new_cell = self.move_and_get_cell_type(cmd);
                
                map.0.insert(new_pos, new_cell);
                if new_cell == Cell::OxygenSystem
                {
                    // Save O2 system position
                    o2_pos = Some(new_pos);
                }
                if new_cell == Cell::Empty || new_cell == Cell::OxygenSystem
                {
                    path.push(cmd);
                    q.push_back(new_pos);
                    if !unexplored_actions.contains_key(&new_pos)
                    {
                        unexplored_actions.insert(new_pos, get_new_directions(cmd));
                    }
                    found_new_cell = true;
                    break;
                }
            }
            // Exhausted all options in current cell, and found nothing
            // Need to backtrack
            if !found_new_cell && available_cmds.is_empty()
            {
                // If there is nothing more to backtrack, we are done.
                if path.is_empty()  
                {
                    break;
                }
                let last_cmd = path.pop().unwrap();
                let new_cmd = last_cmd.reverse();
                let old_pos = get_position_for_command(node, new_cmd);
                self.move_robot(new_cmd);
                q.push_back(old_pos);
            }
            unexplored_actions.insert(node, available_cmds);
        }
        (map, o2_pos)
    }
    pub fn explore(&mut self)
    {
        
        let (map, o2_pos) = self.map_and_find_o2_system();
        let o2_pos = o2_pos.expect("O2 not found!");
        let path = ::aoc2019::graph::djikstra_generic(&map, (0, 0), o2_pos.clone());
        // println!("Path found of length {}", path.len());
        self.map = map;
        let stdout = std::io::stdout();
        let mut stdout = stdout.lock().into_raw_mode().unwrap();
        write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();
        self.draw_map(&mut stdout, &self.map, (0, 0), 42, 42);
        self.draw_path(&mut stdout, &path, (0, 0), 42, 42);
        write!(stdout, "{}{}", cursor::Restore, style::Reset).ok();
        stdout.flush().unwrap();
    }
    pub fn draw_path<W: Write>(&self, mut out: W, path: &Vec<Coord>, center: Coord, width: i64, height: i64)
    {
        let screen_center = (width/2 + center.0, height/2 + center.1);
        let max_y = center.1 + height/2 + 1;
        let len = path.len();
        for (ctr, pos) in path.iter().enumerate() {
            let j = screen_center.1 - pos.1 - 1;
            let i = screen_center.0 + pos.0;
            let screen_pos = cursor::Goto(i as u16+1, j as u16+1);
            let output = if ctr == 0
            {
                format!("{}D", color::Fg(color::Blue))
            }else if ctr == len {
                format!("{}x", color::Fg(color::Red))
            }else{
                format!("{}o", color::Fg(color::Green))
            };
            write!(out, "{}{}", screen_pos, output).ok();
        }
    }
    // Renders current map centered on the robot
    pub fn draw_map<W: Write>(&self, mut out: W, map:&Grid, center: Coord, width: i64, height: i64)
    {
        let min_x = center.0 - width/2;
        let _max_x = center.0 + width/2 + 1;

        let _min_y = center.1 - height/2;
        let max_y = center.1 + height/2 + 1;
        
        write!(out, "{}", cursor::Goto(1, 1)).ok();
        for j in 0..height+1
        {
            let y = max_y - j - 1;
            for i in 0..width+1
            {
                let x = min_x + i;
                let pos = (x, y);

                let screen_pos = cursor::Goto(i as u16+1, j as u16+1);
                let cell_type = map.0.get(& pos).unwrap_or(& Cell::Unknown);
                write!(out, "{}{}", screen_pos, cell_type).ok();
            }
            writeln!(out).ok();
        }
    }
}

fn main()
{
    let input = include_str!("../../inputs/day15.txt").to_string();
    let program: Vec<i64> = parse_numbers_with_delimiter(&input, ',').collect();

    let mut robot = Robot::new(&program);
    robot.explore();
    // robot.step(MoveCommand::North);
    // robot.draw_map(10, 10);
    // println!("Part A: {:?}", painted_cells);
}