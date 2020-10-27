use ::aoc2019::intcode::*;
use ::aoc2019::{parse_numbers_with_delimiter};
use std::collections::HashMap;
use std::thread;
use std::io::{Write, Read};
use std::time::{Instant, Duration};
use termion::{color, cursor, clear, style};
use termion::raw::IntoRawMode;

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
    pub fn explore(&mut self)
    {
        // let mut stdin = termion::async_stdin();
        let mut stdin = std::io::stdin();
        let stdout = std::io::stdout();
        let mut stdout = stdout.lock().into_raw_mode().unwrap();

        let speed = 4;
        write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

        let mut before = Instant::now();

        let mut actions : HashMap<Coord, Vec<MoveCommand>> = HashMap::new();
        let all_cmds = vec![MoveCommand::West, MoveCommand::South, MoveCommand::East, MoveCommand::North];
        for _ in 0..100 {
            let interval = 1000 / speed;
            let now = Instant::now();
            let dt = (now.duration_since(before).subsec_nanos() / 1_000_000) as u64;

            if dt < interval {
                thread::sleep(Duration::from_millis(interval - dt));
                continue;
            }
            before = now;

            write!(stdout, "{}", cursor::Goto(1, 1)).ok();
            self.draw_map(&mut stdout, 50, 50);

            let mut key_bytes = [0];
            stdin.read(&mut key_bytes).unwrap();

            let command = match key_bytes[0] {
                b'q' => break,
                b'A' | b'a' => Some(MoveCommand::West),
                b'D' | b'd' => Some(MoveCommand::East),
                b'W' | b'w' => Some(MoveCommand::North),
                b'S' | b's' => Some(MoveCommand::South),
                _ => None,
            };


            if let Some(command) = command
            {
                // println!("Moving {:?} for {:?}", &command, &self.position);
                self.step(command);
            // }else{
            //     println!("Exiting at {:?}", &self.position);
            //     break;
            }
            
            write!(stdout, "{}", style::Reset).ok();
            stdout.flush().unwrap();
        }
        write!(stdout, "{}{}{}", cursor::Restore, style::Reset, cursor::Goto(1, 1)).ok();
        // self.draw_map(&mut stdout, 10, 10);
        stdout.flush().unwrap();
    }
    // Renders current map centered on the robot
    pub fn draw_map<W: Write>(&self, mut out: W, width: i64, height: i64)
    {
        let min_x = self.position.0 - width/2;
        let _max_x = self.position.0 + width/2 + 1;

        let _min_y = self.position.1 - height/2;
        let max_y = self.position.1 + height/2 + 1;
        
        write!(out, "{}", cursor::Goto(1, 1)).ok();
        for j in 0..height+1
        {
            let y = max_y - j - 1;
            for i in 0..width+1
            {
                let x = min_x + i;
                let pos = (x, y);

                let screen_pos = cursor::Goto(i as u16+1, j as u16+1);
                if pos == self.position
                {
                    write!(out, "{}D", screen_pos).ok();
                }else{
                    write!(out, "{}{}", screen_pos, 
                            self.map.get(& pos).unwrap_or(& Cell::Unknown)).ok();
                }
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